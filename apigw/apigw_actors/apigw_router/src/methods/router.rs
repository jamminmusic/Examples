use jammin_interfaces_apigw::*;
use strum::EnumString;
use wasmbus_rpc::actor::prelude::*;
use wasmbus_rpc::error::RpcError;

#[derive(Clone, Debug, PartialEq, EnumString)]
pub enum Router {
    #[strum(ascii_case_insensitive)]
    Login,
    #[strum(ascii_case_insensitive)]
    Other,
}

impl Router {
    pub async fn send(&self, ctx: &Context, req: &RoutedRequest) -> RpcResult<Box<RoutedResponse>> {
        let res = match self {
            // User Flow - User interaction with auth_url needed.
            Router::Login => match req.method.as_str() {
                // Route to login actor, there will be different routes for different login types
                "GET" => Box::new(ApigwSender::to_actor("login/pkce").route(ctx, req).await?),

                "POST" => Box::new(ApigwSender::to_actor("login/pkce").route(ctx, req).await?),

                _ => Err(RpcError::MethodNotHandled(req.method.clone()))?,
            },
            Router::Other => match req.method.as_str() {
                "PUT" => todo!(),
                _ => Err(RpcError::MethodNotHandled(req.method.clone()))?,
            },
        };
        Ok(res)
    }
}
