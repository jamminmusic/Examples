// This file is @generated by wasmcloud/weld-codegen 0.6.0.
// It is not intended for manual editing.
// namespace: jammin.interfaces.apigw

#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Borrow, borrow::Cow, io::Write, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    cbor::*,
    common::{
        deserialize, message_format, serialize, Context, Message, MessageDispatch, MessageFormat,
        SendOpts, Transport,
    },
    error::{RpcError, RpcResult},
    Timestamp,
};

#[allow(dead_code)]
pub const SMITHY_VERSION: &str = "1.0";

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RoutedRequest {
    #[serde(with = "serde_bytes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Vec<u8>>,
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub path: String,
}

// Encode RoutedRequest as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_routed_request<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &RoutedRequest,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(3)?;
    if let Some(val) = val.body.as_ref() {
        e.str("body")?;
        e.bytes(val)?;
    } else {
        e.null()?;
    }
    e.str("method")?;
    e.str(&val.method)?;
    e.str("path")?;
    e.str(&val.path)?;
    Ok(())
}

// Decode RoutedRequest from cbor input stream
#[doc(hidden)]
pub fn decode_routed_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<RoutedRequest, RpcError> {
    let __result = {
        let mut body: Option<Option<Vec<u8>>> = Some(None);
        let mut method: Option<String> = None;
        let mut path: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct RoutedRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        body = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.bytes()?.to_vec()))
                        }
                    }
                    1 => method = Some(d.str()?.to_string()),
                    2 => path = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "body" => {
                        body = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.bytes()?.to_vec()))
                        }
                    }
                    "method" => method = Some(d.str()?.to_string()),
                    "path" => path = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        RoutedRequest {
            body: body.unwrap(),

            method: if let Some(__x) = method {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field RoutedRequest.method (#1)".to_string(),
                ));
            },

            path: if let Some(__x) = path {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field RoutedRequest.path (#2)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RoutedResponse {
    #[serde(with = "serde_bytes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Vec<u8>>,
    /// If success is false, this may contain an error
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default)]
    pub success: bool,
}

// Encode RoutedResponse as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_routed_response<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &RoutedResponse,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(3)?;
    if let Some(val) = val.body.as_ref() {
        e.str("body")?;
        e.bytes(val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.error.as_ref() {
        e.str("error")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    e.str("success")?;
    e.bool(val.success)?;
    Ok(())
}

// Decode RoutedResponse from cbor input stream
#[doc(hidden)]
pub fn decode_routed_response(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<RoutedResponse, RpcError> {
    let __result = {
        let mut body: Option<Option<Vec<u8>>> = Some(None);
        let mut error: Option<Option<String>> = Some(None);
        let mut success: Option<bool> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct RoutedResponse, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        body = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.bytes()?.to_vec()))
                        }
                    }
                    1 => {
                        error = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    2 => success = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "body" => {
                        body = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.bytes()?.to_vec()))
                        }
                    }
                    "error" => {
                        error = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "success" => success = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        }
        RoutedResponse {
            body: body.unwrap(),
            error: error.unwrap(),

            success: if let Some(__x) = success {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field RoutedResponse.success (#2)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// wasmbus.actorReceive
#[async_trait]
pub trait Apigw {
    async fn route(&self, ctx: &Context, arg: &RoutedRequest) -> RpcResult<RoutedResponse>;
}

/// ApigwReceiver receives messages defined in the Apigw service trait
#[doc(hidden)]
#[async_trait]
pub trait ApigwReceiver: MessageDispatch + Apigw {
    async fn dispatch(&self, ctx: &Context, message: Message<'_>) -> Result<Vec<u8>, RpcError> {
        match message.method {
            "Route" => {
                let value: RoutedRequest = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'RoutedRequest': {}", e)))?;

                let resp = Apigw::route(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(buf)
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Apigw::{}",
                message.method
            ))),
        }
    }
}

/// ApigwSender sends messages to a Apigw service
/// client for sending Apigw messages
#[derive(Debug)]
pub struct ApigwSender<T: Transport> {
    transport: T,
}

impl<T: Transport> ApigwSender<T> {
    /// Constructs a ApigwSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> ApigwSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl ApigwSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Apigw for ApigwSender<T> {
    #[allow(unused)]
    async fn route(&self, ctx: &Context, arg: &RoutedRequest) -> RpcResult<RoutedResponse> {
        let buf = wasmbus_rpc::common::serialize(arg)?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Apigw.Route",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: RoutedResponse = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': RoutedResponse", e)))?;
        Ok(value)
    }
}
