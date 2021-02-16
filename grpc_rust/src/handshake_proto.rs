/// Message containing the handshake information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandshakeInfo {
    #[prost(string, tag = "1")]
    pub ip_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub port: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod handshaker_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The greeting service definition."]
    pub struct HandshakerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl HandshakerClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> HandshakerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Send a message for performing a handshake"]
        pub async fn perform_handshake(
            &mut self,
            request: impl tonic::IntoRequest<super::HandshakeInfo>,
        ) -> Result<tonic::Response<super::HandshakeInfo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/handshake_proto.Handshaker/PerformHandshake",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for HandshakerClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for HandshakerClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "HandshakerClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod handshaker_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with HandshakerServer."]
    #[async_trait]
    pub trait Handshaker: Send + Sync + 'static {
        #[doc = " Send a message for performing a handshake"]
        async fn perform_handshake(
            &self,
            request: tonic::Request<super::HandshakeInfo>,
        ) -> Result<tonic::Response<super::HandshakeInfo>, tonic::Status>;
    }
    #[doc = " The greeting service definition."]
    #[derive(Debug)]
    pub struct HandshakerServer<T: Handshaker> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Handshaker> HandshakerServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for HandshakerServer<T>
    where
        T: Handshaker,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/handshake_proto.Handshaker/PerformHandshake" => {
                    #[allow(non_camel_case_types)]
                    struct PerformHandshakeSvc<T: Handshaker>(pub Arc<T>);
                    impl<T: Handshaker> tonic::server::UnaryService<super::HandshakeInfo> for PerformHandshakeSvc<T> {
                        type Response = super::HandshakeInfo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HandshakeInfo>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).perform_handshake(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = PerformHandshakeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Handshaker> Clone for HandshakerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Handshaker> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Handshaker> tonic::transport::NamedService for HandshakerServer<T> {
        const NAME: &'static str = "handshake_proto.Handshaker";
    }
}
