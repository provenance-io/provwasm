use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
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
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Query/Params");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.marker.v1.Query", "Params"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn all_markers(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllMarkersRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllMarkersResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.marker.v1.Query/AllMarkers");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.marker.v1.Query", "AllMarkers"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn marker(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryMarkerRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryMarkerResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Query/Marker");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.marker.v1.Query", "Marker"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn holding(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryHoldingRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryHoldingResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Query/Holding");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.marker.v1.Query", "Holding"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn supply(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySupplyRequest>,
        ) -> std::result::Result<tonic::Response<super::QuerySupplyResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Query/Supply");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.marker.v1.Query", "Supply"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn escrow(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryEscrowRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryEscrowResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Query/Escrow");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.marker.v1.Query", "Escrow"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn access(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAccessRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAccessResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Query/Access");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.marker.v1.Query", "Access"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn denom_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDenomMetadataRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryDenomMetadataResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.marker.v1.Query/DenomMetadata");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.marker.v1.Query",
                "DenomMetadata",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn account_data(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAccountDataRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAccountDataResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.marker.v1.Query/AccountData");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.marker.v1.Query", "AccountData"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn net_asset_values(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryNetAssetValuesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryNetAssetValuesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.marker.v1.Query/NetAssetValues");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.marker.v1.Query",
                "NetAssetValues",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod query_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QueryServer.
    #[async_trait]
    pub trait Query: Send + Sync + 'static {
        async fn params(
            &self,
            request: tonic::Request<super::QueryParamsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>;
        async fn all_markers(
            &self,
            request: tonic::Request<super::QueryAllMarkersRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllMarkersResponse>, tonic::Status>;
        async fn marker(
            &self,
            request: tonic::Request<super::QueryMarkerRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryMarkerResponse>, tonic::Status>;
        async fn holding(
            &self,
            request: tonic::Request<super::QueryHoldingRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryHoldingResponse>, tonic::Status>;
        async fn supply(
            &self,
            request: tonic::Request<super::QuerySupplyRequest>,
        ) -> std::result::Result<tonic::Response<super::QuerySupplyResponse>, tonic::Status>;
        async fn escrow(
            &self,
            request: tonic::Request<super::QueryEscrowRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryEscrowResponse>, tonic::Status>;
        async fn access(
            &self,
            request: tonic::Request<super::QueryAccessRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAccessResponse>, tonic::Status>;
        async fn denom_metadata(
            &self,
            request: tonic::Request<super::QueryDenomMetadataRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryDenomMetadataResponse>, tonic::Status>;
        async fn account_data(
            &self,
            request: tonic::Request<super::QueryAccountDataRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAccountDataResponse>, tonic::Status>;
        async fn net_asset_values(
            &self,
            request: tonic::Request<super::QueryNetAssetValuesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryNetAssetValuesResponse>, tonic::Status>;
    }
    impl<T: Query> QueryServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QueryServer<T>
    where
        T: Query,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/provenance.marker.v1.Query/Params" => {
                    #[allow(non_camel_case_types)]
                    struct ParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryParamsRequest> for ParamsSvc<T> {
                        type Response = super::QueryParamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryParamsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Query>::params(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ParamsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Query/AllMarkers" => {
                    #[allow(non_camel_case_types)]
                    struct AllMarkersSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAllMarkersRequest> for AllMarkersSvc<T> {
                        type Response = super::QueryAllMarkersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllMarkersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::all_markers(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AllMarkersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Query/Marker" => {
                    #[allow(non_camel_case_types)]
                    struct MarkerSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryMarkerRequest> for MarkerSvc<T> {
                        type Response = super::QueryMarkerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryMarkerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Query>::marker(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarkerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Query/Holding" => {
                    #[allow(non_camel_case_types)]
                    struct HoldingSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryHoldingRequest> for HoldingSvc<T> {
                        type Response = super::QueryHoldingResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryHoldingRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Query>::holding(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HoldingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Query/Supply" => {
                    #[allow(non_camel_case_types)]
                    struct SupplySvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QuerySupplyRequest> for SupplySvc<T> {
                        type Response = super::QuerySupplyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QuerySupplyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Query>::supply(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SupplySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Query/Escrow" => {
                    #[allow(non_camel_case_types)]
                    struct EscrowSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryEscrowRequest> for EscrowSvc<T> {
                        type Response = super::QueryEscrowResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryEscrowRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Query>::escrow(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EscrowSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Query/Access" => {
                    #[allow(non_camel_case_types)]
                    struct AccessSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAccessRequest> for AccessSvc<T> {
                        type Response = super::QueryAccessResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAccessRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Query>::access(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccessSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Query/DenomMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct DenomMetadataSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryDenomMetadataRequest>
                        for DenomMetadataSvc<T>
                    {
                        type Response = super::QueryDenomMetadataResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDenomMetadataRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::denom_metadata(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DenomMetadataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Query/AccountData" => {
                    #[allow(non_camel_case_types)]
                    struct AccountDataSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAccountDataRequest> for AccountDataSvc<T> {
                        type Response = super::QueryAccountDataResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAccountDataRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::account_data(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountDataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Query/NetAssetValues" => {
                    #[allow(non_camel_case_types)]
                    struct NetAssetValuesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryNetAssetValuesRequest>
                        for NetAssetValuesSvc<T>
                    {
                        type Response = super::QueryNetAssetValuesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryNetAssetValuesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::net_asset_values(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NetAssetValuesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
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
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Query> Clone for QueryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Query> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Query> tonic::server::NamedService for QueryServer<T> {
        const NAME: &'static str = "provenance.marker.v1.Query";
    }
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgClient<T>
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
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn finalize(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgFinalizeRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgFinalizeResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/Finalize");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.marker.v1.Msg", "Finalize"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn activate(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgActivateRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgActivateResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/Activate");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.marker.v1.Msg", "Activate"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn cancel(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCancelRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgCancelResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/Cancel");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.marker.v1.Msg", "Cancel"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgDeleteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/Delete");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.marker.v1.Msg", "Delete"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn mint(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMintRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgMintResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/Mint");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.marker.v1.Msg", "Mint"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn burn(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBurnRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgBurnResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/Burn");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.marker.v1.Msg", "Burn"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_access(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAddAccessRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgAddAccessResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/AddAccess");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.marker.v1.Msg", "AddAccess"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_access(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteAccessRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgDeleteAccessResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/DeleteAccess");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.marker.v1.Msg", "DeleteAccess"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn withdraw(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWithdrawRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgWithdrawResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/Withdraw");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.marker.v1.Msg", "Withdraw"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_marker(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAddMarkerRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgAddMarkerResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/AddMarker");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.marker.v1.Msg", "AddMarker"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn transfer(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgTransferRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgTransferResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/Transfer");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.marker.v1.Msg", "Transfer"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn ibc_transfer(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgIbcTransferRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgIbcTransferResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/IbcTransfer");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.marker.v1.Msg", "IbcTransfer"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_denom_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSetDenomMetadataRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgSetDenomMetadataResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/SetDenomMetadata");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.marker.v1.Msg",
                "SetDenomMetadata",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn grant_allowance(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgGrantAllowanceRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgGrantAllowanceResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/GrantAllowance");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.marker.v1.Msg",
                "GrantAllowance",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_finalize_activate_marker(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAddFinalizeActivateMarkerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgAddFinalizeActivateMarkerResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.marker.v1.Msg/AddFinalizeActivateMarker",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.marker.v1.Msg",
                "AddFinalizeActivateMarker",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn supply_increase_proposal(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSupplyIncreaseProposalRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSupplyIncreaseProposalResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.marker.v1.Msg/SupplyIncreaseProposal",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.marker.v1.Msg",
                "SupplyIncreaseProposal",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_required_attributes(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateRequiredAttributesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateRequiredAttributesResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.marker.v1.Msg/UpdateRequiredAttributes",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.marker.v1.Msg",
                "UpdateRequiredAttributes",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_forced_transfer(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateForcedTransferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateForcedTransferResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.marker.v1.Msg/UpdateForcedTransfer",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.marker.v1.Msg",
                "UpdateForcedTransfer",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_account_data(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSetAccountDataRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgSetAccountDataResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/SetAccountData");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.marker.v1.Msg",
                "SetAccountData",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_send_deny_list(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateSendDenyListRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdateSendDenyListResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.marker.v1.Msg/UpdateSendDenyList",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.marker.v1.Msg",
                "UpdateSendDenyList",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_net_asset_values(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAddNetAssetValuesRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgAddNetAssetValuesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/AddNetAssetValues");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.marker.v1.Msg",
                "AddNetAssetValues",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod msg_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MsgServer.
    #[async_trait]
    pub trait Msg: Send + Sync + 'static {
        async fn finalize(
            &self,
            request: tonic::Request<super::MsgFinalizeRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgFinalizeResponse>, tonic::Status>;
        async fn activate(
            &self,
            request: tonic::Request<super::MsgActivateRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgActivateResponse>, tonic::Status>;
        async fn cancel(
            &self,
            request: tonic::Request<super::MsgCancelRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgCancelResponse>, tonic::Status>;
        async fn delete(
            &self,
            request: tonic::Request<super::MsgDeleteRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgDeleteResponse>, tonic::Status>;
        async fn mint(
            &self,
            request: tonic::Request<super::MsgMintRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgMintResponse>, tonic::Status>;
        async fn burn(
            &self,
            request: tonic::Request<super::MsgBurnRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgBurnResponse>, tonic::Status>;
        async fn add_access(
            &self,
            request: tonic::Request<super::MsgAddAccessRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgAddAccessResponse>, tonic::Status>;
        async fn delete_access(
            &self,
            request: tonic::Request<super::MsgDeleteAccessRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgDeleteAccessResponse>, tonic::Status>;
        async fn withdraw(
            &self,
            request: tonic::Request<super::MsgWithdrawRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgWithdrawResponse>, tonic::Status>;
        async fn add_marker(
            &self,
            request: tonic::Request<super::MsgAddMarkerRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgAddMarkerResponse>, tonic::Status>;
        async fn transfer(
            &self,
            request: tonic::Request<super::MsgTransferRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgTransferResponse>, tonic::Status>;
        async fn ibc_transfer(
            &self,
            request: tonic::Request<super::MsgIbcTransferRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgIbcTransferResponse>, tonic::Status>;
        async fn set_denom_metadata(
            &self,
            request: tonic::Request<super::MsgSetDenomMetadataRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgSetDenomMetadataResponse>, tonic::Status>;
        async fn grant_allowance(
            &self,
            request: tonic::Request<super::MsgGrantAllowanceRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgGrantAllowanceResponse>, tonic::Status>;
        async fn add_finalize_activate_marker(
            &self,
            request: tonic::Request<super::MsgAddFinalizeActivateMarkerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgAddFinalizeActivateMarkerResponse>,
            tonic::Status,
        >;
        async fn supply_increase_proposal(
            &self,
            request: tonic::Request<super::MsgSupplyIncreaseProposalRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSupplyIncreaseProposalResponse>,
            tonic::Status,
        >;
        async fn update_required_attributes(
            &self,
            request: tonic::Request<super::MsgUpdateRequiredAttributesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateRequiredAttributesResponse>,
            tonic::Status,
        >;
        async fn update_forced_transfer(
            &self,
            request: tonic::Request<super::MsgUpdateForcedTransferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateForcedTransferResponse>,
            tonic::Status,
        >;
        async fn set_account_data(
            &self,
            request: tonic::Request<super::MsgSetAccountDataRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgSetAccountDataResponse>, tonic::Status>;
        async fn update_send_deny_list(
            &self,
            request: tonic::Request<super::MsgUpdateSendDenyListRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdateSendDenyListResponse>, tonic::Status>;
        async fn add_net_asset_values(
            &self,
            request: tonic::Request<super::MsgAddNetAssetValuesRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgAddNetAssetValuesResponse>, tonic::Status>;
    }
    impl<T: Msg> MsgServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MsgServer<T>
    where
        T: Msg,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/provenance.marker.v1.Msg/Finalize" => {
                    #[allow(non_camel_case_types)]
                    struct FinalizeSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgFinalizeRequest> for FinalizeSvc<T> {
                        type Response = super::MsgFinalizeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgFinalizeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Msg>::finalize(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FinalizeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/Activate" => {
                    #[allow(non_camel_case_types)]
                    struct ActivateSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgActivateRequest> for ActivateSvc<T> {
                        type Response = super::MsgActivateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgActivateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Msg>::activate(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ActivateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/Cancel" => {
                    #[allow(non_camel_case_types)]
                    struct CancelSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCancelRequest> for CancelSvc<T> {
                        type Response = super::MsgCancelResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCancelRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Msg>::cancel(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CancelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/Delete" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDeleteRequest> for DeleteSvc<T> {
                        type Response = super::MsgDeleteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDeleteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Msg>::delete(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/Mint" => {
                    #[allow(non_camel_case_types)]
                    struct MintSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgMintRequest> for MintSvc<T> {
                        type Response = super::MsgMintResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgMintRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Msg>::mint(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MintSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/Burn" => {
                    #[allow(non_camel_case_types)]
                    struct BurnSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgBurnRequest> for BurnSvc<T> {
                        type Response = super::MsgBurnResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgBurnRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Msg>::burn(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BurnSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/AddAccess" => {
                    #[allow(non_camel_case_types)]
                    struct AddAccessSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgAddAccessRequest> for AddAccessSvc<T> {
                        type Response = super::MsgAddAccessResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAddAccessRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Msg>::add_access(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddAccessSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/DeleteAccess" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAccessSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDeleteAccessRequest> for DeleteAccessSvc<T> {
                        type Response = super::MsgDeleteAccessResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDeleteAccessRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::delete_access(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteAccessSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/Withdraw" => {
                    #[allow(non_camel_case_types)]
                    struct WithdrawSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgWithdrawRequest> for WithdrawSvc<T> {
                        type Response = super::MsgWithdrawResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWithdrawRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Msg>::withdraw(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WithdrawSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/AddMarker" => {
                    #[allow(non_camel_case_types)]
                    struct AddMarkerSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgAddMarkerRequest> for AddMarkerSvc<T> {
                        type Response = super::MsgAddMarkerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAddMarkerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Msg>::add_marker(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddMarkerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/Transfer" => {
                    #[allow(non_camel_case_types)]
                    struct TransferSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgTransferRequest> for TransferSvc<T> {
                        type Response = super::MsgTransferResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgTransferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Msg>::transfer(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/IbcTransfer" => {
                    #[allow(non_camel_case_types)]
                    struct IbcTransferSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgIbcTransferRequest> for IbcTransferSvc<T> {
                        type Response = super::MsgIbcTransferResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgIbcTransferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::ibc_transfer(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IbcTransferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/SetDenomMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct SetDenomMetadataSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSetDenomMetadataRequest>
                        for SetDenomMetadataSvc<T>
                    {
                        type Response = super::MsgSetDenomMetadataResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSetDenomMetadataRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::set_denom_metadata(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetDenomMetadataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/GrantAllowance" => {
                    #[allow(non_camel_case_types)]
                    struct GrantAllowanceSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgGrantAllowanceRequest> for GrantAllowanceSvc<T> {
                        type Response = super::MsgGrantAllowanceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgGrantAllowanceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::grant_allowance(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GrantAllowanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/AddFinalizeActivateMarker" => {
                    #[allow(non_camel_case_types)]
                    struct AddFinalizeActivateMarkerSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgAddFinalizeActivateMarkerRequest>
                        for AddFinalizeActivateMarkerSvc<T>
                    {
                        type Response = super::MsgAddFinalizeActivateMarkerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAddFinalizeActivateMarkerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::add_finalize_activate_marker(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddFinalizeActivateMarkerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/SupplyIncreaseProposal" => {
                    #[allow(non_camel_case_types)]
                    struct SupplyIncreaseProposalSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgSupplyIncreaseProposalRequest>
                        for SupplyIncreaseProposalSvc<T>
                    {
                        type Response = super::MsgSupplyIncreaseProposalResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSupplyIncreaseProposalRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::supply_increase_proposal(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SupplyIncreaseProposalSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/UpdateRequiredAttributes" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateRequiredAttributesSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgUpdateRequiredAttributesRequest>
                        for UpdateRequiredAttributesSvc<T>
                    {
                        type Response = super::MsgUpdateRequiredAttributesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateRequiredAttributesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::update_required_attributes(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateRequiredAttributesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/UpdateForcedTransfer" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateForcedTransferSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpdateForcedTransferRequest>
                        for UpdateForcedTransferSvc<T>
                    {
                        type Response = super::MsgUpdateForcedTransferResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateForcedTransferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::update_forced_transfer(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateForcedTransferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/SetAccountData" => {
                    #[allow(non_camel_case_types)]
                    struct SetAccountDataSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSetAccountDataRequest> for SetAccountDataSvc<T> {
                        type Response = super::MsgSetAccountDataResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSetAccountDataRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::set_account_data(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetAccountDataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/UpdateSendDenyList" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateSendDenyListSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpdateSendDenyListRequest>
                        for UpdateSendDenyListSvc<T>
                    {
                        type Response = super::MsgUpdateSendDenyListResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateSendDenyListRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::update_send_deny_list(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateSendDenyListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/AddNetAssetValues" => {
                    #[allow(non_camel_case_types)]
                    struct AddNetAssetValuesSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgAddNetAssetValuesRequest>
                        for AddNetAssetValuesSvc<T>
                    {
                        type Response = super::MsgAddNetAssetValuesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAddNetAssetValuesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::add_net_asset_values(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddNetAssetValuesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
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
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Msg> Clone for MsgServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Msg> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Msg> tonic::server::NamedService for MsgServer<T> {
        const NAME: &'static str = "provenance.marker.v1.Msg";
    }
}
