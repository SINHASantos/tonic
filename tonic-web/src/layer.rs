use super::{BoxBody, GrpcWebService};

use tower_layer::Layer;
use tower_service::Service;

/// Layer implementing the grpc-web protocol.
#[derive(Debug, Default, Clone)]
pub struct GrpcWebLayer {
    _priv: (),
}

impl GrpcWebLayer {
    /// Create a new grpc-web layer.
    pub fn new() -> GrpcWebLayer {
        Self::default()
    }
}

impl<S> Layer<S> for GrpcWebLayer
where
    S: Service<http::Request<BoxBody>, Response = http::Response<BoxBody>>,
{
    type Service = GrpcWebService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        GrpcWebService::new(inner)
    }
}
