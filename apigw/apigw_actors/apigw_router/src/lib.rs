#![allow(unused_imports)]
mod methods;

use crate::methods::router::*;
use crate::methods::url::*;
use jammin_interfaces_apigw::RoutedRequest;
use serde_urlencoded;
use std::str::FromStr;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_logging::{debug, error, info, log, warn};

// const BASE_URL: &str = "localhost:8000";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct ApiGwActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for ApiGwActor {
    async fn handle_request(&self, _ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        // example assignments available
        let method = req.clone().method;
        let path = parse_url_path(&req.path).await.expect("path error");
        let query = parse_url_query(&req.path).await.expect("query error");
        // let fragment = parse_url_fragment(&req.path).await.expect("fragment error");

        info!("req: {:?}", req);

        let rreq = RoutedRequest {
            path: path.clone(),
            body: Some(
                serde_urlencoded::to_string(query)
                    .unwrap()
                    .as_bytes()
                    .to_vec(),
            ),
            method: method,
        };

        info!("msg: {:?}", rreq);

        let response = *Router::from_str(path.as_str())
            .expect("Grant type not found")
            .send(_ctx, &rreq)
            .await?;

        info!("Response: {:?}", response.body);

        Ok(HttpResponse {
            body: response.body.unwrap(),
            ..Default::default()
        })
    }
}
