#![allow(unused_imports)]
use anyhow::Error;
use base64::{
    engine::{
        self,
        general_purpose::{self, URL_SAFE_NO_PAD},
    },
    Engine as _,
};
use jammin_interfaces_messaging::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use url::Url;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpclient::*;
use wasmcloud_interface_keyvalue::{KeyValue, KeyValueSender};
use wasmcloud_interface_logging::{debug, error, info, log, warn};
use wasmcloud_interface_numbergen::{generate_guid, random_32, random_in_range};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, MessageSubscriber)]
struct PkceLogin {}

#[async_trait]

//// CONVERT THIS TO USE THE APIGW HANDLER
impl MessageSubscriber for PkceLogin {
    async fn handle_message(&self, ctx: &Context, msg: &SubMessage) -> RpcResult<()> {
        // extract social provider and grant type from SubMessage
        // SubMessage struct = pub struct SubMessage { pub body: Vec<u8>, pub reply_to: Option<String>, pub subject: String }
        // access msg.body -> deserialize -> set vector containing provider and grant type as login_params variable below.

        // deserialize query string
        let qs = serde_json::from_slice::<Oauth2Request>(&msg.body)
            .map_err(|e| tag_err("deserializing", e))?;
        info!(
            "Subject: {}, Body: {:?},{:?}, Context: {:?}, Struct: {:?}",
            &msg.subject.to_lowercase(),
            qs.provider,
            qs.grant,
            ctx,
            qs
        );
        login(&ctx, &qs)
            .await
            .map_err(|e| tag_err("calling login function", e))?;

        // To avoid giving away helpful information to an attacker, the API Gateway should return errors of not_found (404) on failure.
        Ok(())
    }
}

// Concider using a thread to store auth data concurrently
// https://docs.rs/tokio/latest/tokio/task/
async fn login(ctx: &Context, req: &Oauth2Request) -> Result<bool, anyhow::Error> {
    let auth_config = get_auth_config(ctx, req).await?;
    info!("{:?}", auth_config);

    // replace with auth_code
    let url_res = get_auth_url(&auth_config).await?;
    info!("{:?}", url_res);

    // TODO
    // store_data(ctx, &url_res).await?;

    push_auth_url(&ctx, &url_res.url, req).await?;

    //TODO
    // pull_auth_confirmation().await?;
    // store_data().await?;
    // authorize_user().await?;
    // store_data().await?;

    Ok(true)
}

// function to get specific auth provider configuration from KV-Vault
async fn get_auth_config(
    ctx: &Context,
    req: &Oauth2Request,
) -> Result<AuthUrlRequest, anyhow::Error> {
    // get key based on req parameter field for social provider defined above
    let res = KeyValueSender::new_with_link("vault")?
        .get(ctx, &req.provider)
        .await?;
    info!("{:?}", res);

    // res contains JSON structured as follows: {"auth_url":"some_url","client_id":"some_id","client_secret":"some_secret","redirect_url":"some_callback","scope":"some_scope","token_url":"some_token_url"}
    let mut config: AuthUrlRequest = serde_json::from_str(res.value.as_str())?;
    config.grant_type = Some(String::from(&req.grant));
    info!("{:?}", req);

    Ok(config)
}

async fn get_auth_url(config: &AuthUrlRequest) -> Result<AuthUrlResponse, anyhow::Error> {
    // may need to generate larger state number
    let state: String = random_in_range(1000000000, 4200000000).await?.to_string();
    let code_verifier: String = random_in_range(1000000000, 4200000000).await?.to_string();
    let code_challenge =
        general_purpose::URL_SAFE_NO_PAD.encode(&Sha256::digest(code_verifier.as_bytes()));

    let mut auth_url = Url::parse(config.auth_url.as_str()).unwrap();
    auth_url
        .query_pairs_mut()
        .append_pair("client_id", config.client_id.as_str())
        .append_pair("redirect_uri", config.redirect_url.as_str())
        // response type must be token or code
        .append_pair("response_type", "code")
        .append_pair("scope", config.scope.as_str())
        .append_pair(
            "state",
            general_purpose::URL_SAFE_NO_PAD.encode(&state).as_str(),
        )
        .append_pair("code_challenge", code_challenge.as_str())
        .append_pair("code_challenge_method", "S256");

    let res = AuthUrlResponse {
        url: auth_url.to_string(),
        csrf_state: state,
    };

    Ok(res)
}

// Function to push requested auth_url to ngs - needs to be a unique subject that the client already knows about
// Concider passing third piece of data with client request that will absolutely and MUST be unique.
// Make sure to check if subject already exists for unique value before pushing.
async fn push_auth_url(
    ctx: &Context,
    url: &String,
    sub: &Oauth2Request,
) -> Result<String, anyhow::Error> {
    let client = MessagingSender::new();
    let res = client
        .request(
            ctx,
            &RequestMessage {
                subject: sub.id.to_string(),
                body: url.as_bytes().to_vec(),
                timeout_ms: 1000,
            },
        )
        .await?;
    info!("{:?}", res);

    Ok("ok".to_string())
}

// function to pull returned data from ngs after user confirms authorization
async fn pull_auth_confirmation() -> Result<(), anyhow::Error> {
    todo!();
}

async fn authorize_user() -> Result<(), anyhow::Error> {
    todo!();
    // generate_client_state();
    // store_data();
}

async fn generate_client_state() -> Result<(), anyhow::Error> {
    // NEED TO MAKE SURE IT IS GLOBALLY UNIQUE FOR ALL USERS
    todo!();
    let user_state_id: String = generate_guid().await?; // SHould this be random in range
}

// function to store relevant data from processes - keep generic and pass in data to be stored as parameters
// return true if success and false if fail
async fn store_data(ctx: &Context, data: &AuthUrlResponse) -> Result<bool, anyhow::Error> {
    info!("Context: {:?}, Data: {:?}", ctx, data);
    todo!();
    // If storing salt, verify that the salt does not already exist for other users!
    // salt_data(data.csrf_state, data.salt);
}

async fn salt_data(ctx: &Context) -> Result<(), anyhow::Error> {
    todo!();
    // use numbergen
    // shouldn't need to sign csrf_state, sign tokens and store salt beside hashed value
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Oauth2Request {
    provider: String,
    grant: String,
    id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AuthUrlRequest {
    // Option so that if None can return error
    grant_type: Option<String>,
    client_id: String,
    client_secret: String,
    auth_url: String,
    token_url: String,
    redirect_url: String,
    scope: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]

struct AuthUrlResponse {
    url: String,
    csrf_state: String,
}
// helper function to give a little more information about where the error came from
fn tag_err<T: std::string::ToString>(msg: &str, e: T) -> RpcError {
    RpcError::ActorHandler(format!("{}: {}", msg, e.to_string()))
}
