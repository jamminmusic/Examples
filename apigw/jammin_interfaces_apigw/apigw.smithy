// apigw.smithy

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "jammin.interfaces.apigw", crate: "jammin_interfaces_apigw" } ]

namespace jammin.interfaces.apigw

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64


@wasmbus(actorReceive: true)
service Apigw {
  version: "0.1",
  operations: [ Route ]
}

operation Route {
  input: RoutedRequest,
  output: RoutedResponse,
}

structure RoutedRequest {
  @n(0)
  @required
  method: String

  @n(1)
  @required
  path: String

  @n(2)
  @sensitive
  body: Blob
}

structure RoutedResponse {
  @n(0)
  @required
  success: Boolean

  /// If success is false, this may contain an error
  @n(1)
  error: String

  @n(2)
  @sensitive
  body: Blob
}

/// Routed Message Handler
operation HandleRouted {
    input: RoutedRequest
}