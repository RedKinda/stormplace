/// Public ID of each client. Used to identify changes and logins. 'server' is reserved
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicId {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateId {
    #[prost(message, optional, tag="1")]
    pub public_id: ::core::option::Option<PublicId>,
    #[prost(string, tag="2")]
    pub token: ::prost::alloc::string::String,
}
/// Pixel change
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PixelUpdate {
    /// New pixel color
    #[prost(uint32, tag="1")]
    pub color: u32,
    #[prost(uint64, tag="2")]
    pub x: u64,
    #[prost(uint64, tag="3")]
    pub y: u64,
    #[prost(message, optional, tag="4")]
    pub source: ::core::option::Option<PublicId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PixelPaintRequest {
    /// New pixel color
    #[prost(uint32, tag="1")]
    pub color: u32,
    #[prost(uint64, tag="2")]
    pub x: u64,
    #[prost(uint64, tag="3")]
    pub y: u64,
    #[prost(message, optional, tag="4")]
    pub source: ::core::option::Option<PublicId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PixelPaintResponse {
    #[prost(bool, tag="1")]
    pub success: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanvasMetadataRequest {
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<PublicId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanvasMetadata {
    #[prost(uint64, tag="1")]
    pub x_size: u64,
    #[prost(uint64, tag="2")]
    pub y_size: u64,
    #[prost(uint64, tag="3")]
    pub subscriber_count: u64,
}
/// Generated client implementations.
pub mod stormplace_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct StormplaceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StormplaceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> StormplaceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> StormplaceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            StormplaceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn stream_changes(
            &mut self,
            request: impl tonic::IntoRequest<super::PublicId>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::PixelUpdate>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/stormplace.Stormplace/StreamChanges",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn get_canvas_state_once(
            &mut self,
            request: impl tonic::IntoRequest<super::PublicId>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::PixelUpdate>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/stormplace.Stormplace/GetCanvasStateOnce",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn paint_pixel(
            &mut self,
            request: impl tonic::IntoRequest<super::PixelPaintRequest>,
        ) -> Result<tonic::Response<super::PixelPaintResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/stormplace.Stormplace/PaintPixel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::CanvasMetadataRequest>,
        ) -> Result<tonic::Response<super::CanvasMetadata>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/stormplace.Stormplace/GetMetadata",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod stormplace_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with StormplaceServer.
    #[async_trait]
    pub trait Stormplace: Send + Sync + 'static {
        ///Server streaming response type for the StreamChanges method.
        type StreamChangesStream: futures_core::Stream<
                Item = Result<super::PixelUpdate, tonic::Status>,
            >
            + Send
            + 'static;
        async fn stream_changes(
            &self,
            request: tonic::Request<super::PublicId>,
        ) -> Result<tonic::Response<Self::StreamChangesStream>, tonic::Status>;
        ///Server streaming response type for the GetCanvasStateOnce method.
        type GetCanvasStateOnceStream: futures_core::Stream<
                Item = Result<super::PixelUpdate, tonic::Status>,
            >
            + Send
            + 'static;
        async fn get_canvas_state_once(
            &self,
            request: tonic::Request<super::PublicId>,
        ) -> Result<tonic::Response<Self::GetCanvasStateOnceStream>, tonic::Status>;
        async fn paint_pixel(
            &self,
            request: tonic::Request<super::PixelPaintRequest>,
        ) -> Result<tonic::Response<super::PixelPaintResponse>, tonic::Status>;
        async fn get_metadata(
            &self,
            request: tonic::Request<super::CanvasMetadataRequest>,
        ) -> Result<tonic::Response<super::CanvasMetadata>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct StormplaceServer<T: Stormplace> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Stormplace> StormplaceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StormplaceServer<T>
    where
        T: Stormplace,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/stormplace.Stormplace/StreamChanges" => {
                    #[allow(non_camel_case_types)]
                    struct StreamChangesSvc<T: Stormplace>(pub Arc<T>);
                    impl<
                        T: Stormplace,
                    > tonic::server::ServerStreamingService<super::PublicId>
                    for StreamChangesSvc<T> {
                        type Response = super::PixelUpdate;
                        type ResponseStream = T::StreamChangesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PublicId>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).stream_changes(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamChangesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/stormplace.Stormplace/GetCanvasStateOnce" => {
                    #[allow(non_camel_case_types)]
                    struct GetCanvasStateOnceSvc<T: Stormplace>(pub Arc<T>);
                    impl<
                        T: Stormplace,
                    > tonic::server::ServerStreamingService<super::PublicId>
                    for GetCanvasStateOnceSvc<T> {
                        type Response = super::PixelUpdate;
                        type ResponseStream = T::GetCanvasStateOnceStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PublicId>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_canvas_state_once(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCanvasStateOnceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/stormplace.Stormplace/PaintPixel" => {
                    #[allow(non_camel_case_types)]
                    struct PaintPixelSvc<T: Stormplace>(pub Arc<T>);
                    impl<
                        T: Stormplace,
                    > tonic::server::UnaryService<super::PixelPaintRequest>
                    for PaintPixelSvc<T> {
                        type Response = super::PixelPaintResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PixelPaintRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).paint_pixel(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PaintPixelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/stormplace.Stormplace/GetMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct GetMetadataSvc<T: Stormplace>(pub Arc<T>);
                    impl<
                        T: Stormplace,
                    > tonic::server::UnaryService<super::CanvasMetadataRequest>
                    for GetMetadataSvc<T> {
                        type Response = super::CanvasMetadata;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CanvasMetadataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_metadata(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMetadataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Stormplace> Clone for StormplaceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Stormplace> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Stormplace> tonic::transport::NamedService for StormplaceServer<T> {
        const NAME: &'static str = "stormplace.Stormplace";
    }
}
