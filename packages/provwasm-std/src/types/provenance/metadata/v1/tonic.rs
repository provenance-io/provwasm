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
            let path = http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Query/Params");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.metadata.v1.Query", "Params"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn scope(
            &mut self,
            request: impl tonic::IntoRequest<super::ScopeRequest>,
        ) -> std::result::Result<tonic::Response<super::ScopeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Query/Scope");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.metadata.v1.Query", "Scope"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn scopes_all(
            &mut self,
            request: impl tonic::IntoRequest<super::ScopesAllRequest>,
        ) -> std::result::Result<tonic::Response<super::ScopesAllResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Query/ScopesAll");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.metadata.v1.Query", "ScopesAll"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn sessions(
            &mut self,
            request: impl tonic::IntoRequest<super::SessionsRequest>,
        ) -> std::result::Result<tonic::Response<super::SessionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Query/Sessions");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.metadata.v1.Query", "Sessions"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn sessions_all(
            &mut self,
            request: impl tonic::IntoRequest<super::SessionsAllRequest>,
        ) -> std::result::Result<tonic::Response<super::SessionsAllResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Query/SessionsAll");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Query",
                "SessionsAll",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn records(
            &mut self,
            request: impl tonic::IntoRequest<super::RecordsRequest>,
        ) -> std::result::Result<tonic::Response<super::RecordsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Query/Records");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.metadata.v1.Query", "Records"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn records_all(
            &mut self,
            request: impl tonic::IntoRequest<super::RecordsAllRequest>,
        ) -> std::result::Result<tonic::Response<super::RecordsAllResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Query/RecordsAll");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Query",
                "RecordsAll",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn ownership(
            &mut self,
            request: impl tonic::IntoRequest<super::OwnershipRequest>,
        ) -> std::result::Result<tonic::Response<super::OwnershipResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Query/Ownership");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.metadata.v1.Query", "Ownership"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn value_ownership(
            &mut self,
            request: impl tonic::IntoRequest<super::ValueOwnershipRequest>,
        ) -> std::result::Result<tonic::Response<super::ValueOwnershipResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Query/ValueOwnership",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Query",
                "ValueOwnership",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn scope_specification(
            &mut self,
            request: impl tonic::IntoRequest<super::ScopeSpecificationRequest>,
        ) -> std::result::Result<tonic::Response<super::ScopeSpecificationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Query/ScopeSpecification",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Query",
                "ScopeSpecification",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn scope_specifications_all(
            &mut self,
            request: impl tonic::IntoRequest<super::ScopeSpecificationsAllRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ScopeSpecificationsAllResponse>,
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
                "/provenance.metadata.v1.Query/ScopeSpecificationsAll",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Query",
                "ScopeSpecificationsAll",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn contract_specification(
            &mut self,
            request: impl tonic::IntoRequest<super::ContractSpecificationRequest>,
        ) -> std::result::Result<tonic::Response<super::ContractSpecificationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Query/ContractSpecification",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Query",
                "ContractSpecification",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn contract_specifications_all(
            &mut self,
            request: impl tonic::IntoRequest<super::ContractSpecificationsAllRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ContractSpecificationsAllResponse>,
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
                "/provenance.metadata.v1.Query/ContractSpecificationsAll",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Query",
                "ContractSpecificationsAll",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn record_specifications_for_contract_specification(
            &mut self,
            request: impl tonic::IntoRequest<super::RecordSpecificationsForContractSpecificationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RecordSpecificationsForContractSpecificationResponse>,
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
                "/provenance.metadata.v1.Query/RecordSpecificationsForContractSpecification",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Query",
                "RecordSpecificationsForContractSpecification",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn record_specification(
            &mut self,
            request: impl tonic::IntoRequest<super::RecordSpecificationRequest>,
        ) -> std::result::Result<tonic::Response<super::RecordSpecificationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Query/RecordSpecification",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Query",
                "RecordSpecification",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn record_specifications_all(
            &mut self,
            request: impl tonic::IntoRequest<super::RecordSpecificationsAllRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RecordSpecificationsAllResponse>,
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
                "/provenance.metadata.v1.Query/RecordSpecificationsAll",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Query",
                "RecordSpecificationsAll",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_by_addr(
            &mut self,
            request: impl tonic::IntoRequest<super::GetByAddrRequest>,
        ) -> std::result::Result<tonic::Response<super::GetByAddrResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Query/GetByAddr");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.metadata.v1.Query", "GetByAddr"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn os_locator_params(
            &mut self,
            request: impl tonic::IntoRequest<super::OsLocatorParamsRequest>,
        ) -> std::result::Result<tonic::Response<super::OsLocatorParamsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Query/OSLocatorParams",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Query",
                "OSLocatorParams",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn os_locator(
            &mut self,
            request: impl tonic::IntoRequest<super::OsLocatorRequest>,
        ) -> std::result::Result<tonic::Response<super::OsLocatorResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Query/OSLocator");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.metadata.v1.Query", "OSLocator"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn os_locators_by_uri(
            &mut self,
            request: impl tonic::IntoRequest<super::OsLocatorsByUriRequest>,
        ) -> std::result::Result<tonic::Response<super::OsLocatorsByUriResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Query/OSLocatorsByURI",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Query",
                "OSLocatorsByURI",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn os_locators_by_scope(
            &mut self,
            request: impl tonic::IntoRequest<super::OsLocatorsByScopeRequest>,
        ) -> std::result::Result<tonic::Response<super::OsLocatorsByScopeResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Query/OSLocatorsByScope",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Query",
                "OSLocatorsByScope",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn os_all_locators(
            &mut self,
            request: impl tonic::IntoRequest<super::OsAllLocatorsRequest>,
        ) -> std::result::Result<tonic::Response<super::OsAllLocatorsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Query/OSAllLocators");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Query",
                "OSAllLocators",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn account_data(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountDataRequest>,
        ) -> std::result::Result<tonic::Response<super::AccountDataResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Query/AccountData");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Query",
                "AccountData",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn scope_net_asset_values(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryScopeNetAssetValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryScopeNetAssetValuesResponse>,
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
                "/provenance.metadata.v1.Query/ScopeNetAssetValues",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Query",
                "ScopeNetAssetValues",
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
        async fn scope(
            &self,
            request: tonic::Request<super::ScopeRequest>,
        ) -> std::result::Result<tonic::Response<super::ScopeResponse>, tonic::Status>;
        async fn scopes_all(
            &self,
            request: tonic::Request<super::ScopesAllRequest>,
        ) -> std::result::Result<tonic::Response<super::ScopesAllResponse>, tonic::Status>;
        async fn sessions(
            &self,
            request: tonic::Request<super::SessionsRequest>,
        ) -> std::result::Result<tonic::Response<super::SessionsResponse>, tonic::Status>;
        async fn sessions_all(
            &self,
            request: tonic::Request<super::SessionsAllRequest>,
        ) -> std::result::Result<tonic::Response<super::SessionsAllResponse>, tonic::Status>;
        async fn records(
            &self,
            request: tonic::Request<super::RecordsRequest>,
        ) -> std::result::Result<tonic::Response<super::RecordsResponse>, tonic::Status>;
        async fn records_all(
            &self,
            request: tonic::Request<super::RecordsAllRequest>,
        ) -> std::result::Result<tonic::Response<super::RecordsAllResponse>, tonic::Status>;
        async fn ownership(
            &self,
            request: tonic::Request<super::OwnershipRequest>,
        ) -> std::result::Result<tonic::Response<super::OwnershipResponse>, tonic::Status>;
        async fn value_ownership(
            &self,
            request: tonic::Request<super::ValueOwnershipRequest>,
        ) -> std::result::Result<tonic::Response<super::ValueOwnershipResponse>, tonic::Status>;
        async fn scope_specification(
            &self,
            request: tonic::Request<super::ScopeSpecificationRequest>,
        ) -> std::result::Result<tonic::Response<super::ScopeSpecificationResponse>, tonic::Status>;
        async fn scope_specifications_all(
            &self,
            request: tonic::Request<super::ScopeSpecificationsAllRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ScopeSpecificationsAllResponse>,
            tonic::Status,
        >;
        async fn contract_specification(
            &self,
            request: tonic::Request<super::ContractSpecificationRequest>,
        ) -> std::result::Result<tonic::Response<super::ContractSpecificationResponse>, tonic::Status>;
        async fn contract_specifications_all(
            &self,
            request: tonic::Request<super::ContractSpecificationsAllRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ContractSpecificationsAllResponse>,
            tonic::Status,
        >;
        async fn record_specifications_for_contract_specification(
            &self,
            request: tonic::Request<super::RecordSpecificationsForContractSpecificationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RecordSpecificationsForContractSpecificationResponse>,
            tonic::Status,
        >;
        async fn record_specification(
            &self,
            request: tonic::Request<super::RecordSpecificationRequest>,
        ) -> std::result::Result<tonic::Response<super::RecordSpecificationResponse>, tonic::Status>;
        async fn record_specifications_all(
            &self,
            request: tonic::Request<super::RecordSpecificationsAllRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RecordSpecificationsAllResponse>,
            tonic::Status,
        >;
        async fn get_by_addr(
            &self,
            request: tonic::Request<super::GetByAddrRequest>,
        ) -> std::result::Result<tonic::Response<super::GetByAddrResponse>, tonic::Status>;
        async fn os_locator_params(
            &self,
            request: tonic::Request<super::OsLocatorParamsRequest>,
        ) -> std::result::Result<tonic::Response<super::OsLocatorParamsResponse>, tonic::Status>;
        async fn os_locator(
            &self,
            request: tonic::Request<super::OsLocatorRequest>,
        ) -> std::result::Result<tonic::Response<super::OsLocatorResponse>, tonic::Status>;
        async fn os_locators_by_uri(
            &self,
            request: tonic::Request<super::OsLocatorsByUriRequest>,
        ) -> std::result::Result<tonic::Response<super::OsLocatorsByUriResponse>, tonic::Status>;
        async fn os_locators_by_scope(
            &self,
            request: tonic::Request<super::OsLocatorsByScopeRequest>,
        ) -> std::result::Result<tonic::Response<super::OsLocatorsByScopeResponse>, tonic::Status>;
        async fn os_all_locators(
            &self,
            request: tonic::Request<super::OsAllLocatorsRequest>,
        ) -> std::result::Result<tonic::Response<super::OsAllLocatorsResponse>, tonic::Status>;
        async fn account_data(
            &self,
            request: tonic::Request<super::AccountDataRequest>,
        ) -> std::result::Result<tonic::Response<super::AccountDataResponse>, tonic::Status>;
        async fn scope_net_asset_values(
            &self,
            request: tonic::Request<super::QueryScopeNetAssetValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryScopeNetAssetValuesResponse>,
            tonic::Status,
        >;
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
                "/provenance.metadata.v1.Query/Params" => {
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
                "/provenance.metadata.v1.Query/Scope" => {
                    #[allow(non_camel_case_types)]
                    struct ScopeSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::ScopeRequest> for ScopeSvc<T> {
                        type Response = super::ScopeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ScopeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Query>::scope(&inner, request).await };
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
                        let method = ScopeSvc(inner);
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
                "/provenance.metadata.v1.Query/ScopesAll" => {
                    #[allow(non_camel_case_types)]
                    struct ScopesAllSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::ScopesAllRequest> for ScopesAllSvc<T> {
                        type Response = super::ScopesAllResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ScopesAllRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::scopes_all(&inner, request).await };
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
                        let method = ScopesAllSvc(inner);
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
                "/provenance.metadata.v1.Query/Sessions" => {
                    #[allow(non_camel_case_types)]
                    struct SessionsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::SessionsRequest> for SessionsSvc<T> {
                        type Response = super::SessionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SessionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Query>::sessions(&inner, request).await };
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
                        let method = SessionsSvc(inner);
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
                "/provenance.metadata.v1.Query/SessionsAll" => {
                    #[allow(non_camel_case_types)]
                    struct SessionsAllSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::SessionsAllRequest> for SessionsAllSvc<T> {
                        type Response = super::SessionsAllResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SessionsAllRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::sessions_all(&inner, request).await };
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
                        let method = SessionsAllSvc(inner);
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
                "/provenance.metadata.v1.Query/Records" => {
                    #[allow(non_camel_case_types)]
                    struct RecordsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::RecordsRequest> for RecordsSvc<T> {
                        type Response = super::RecordsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RecordsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Query>::records(&inner, request).await };
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
                        let method = RecordsSvc(inner);
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
                "/provenance.metadata.v1.Query/RecordsAll" => {
                    #[allow(non_camel_case_types)]
                    struct RecordsAllSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::RecordsAllRequest> for RecordsAllSvc<T> {
                        type Response = super::RecordsAllResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RecordsAllRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::records_all(&inner, request).await };
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
                        let method = RecordsAllSvc(inner);
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
                "/provenance.metadata.v1.Query/Ownership" => {
                    #[allow(non_camel_case_types)]
                    struct OwnershipSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::OwnershipRequest> for OwnershipSvc<T> {
                        type Response = super::OwnershipResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OwnershipRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Query>::ownership(&inner, request).await };
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
                        let method = OwnershipSvc(inner);
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
                "/provenance.metadata.v1.Query/ValueOwnership" => {
                    #[allow(non_camel_case_types)]
                    struct ValueOwnershipSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::ValueOwnershipRequest> for ValueOwnershipSvc<T> {
                        type Response = super::ValueOwnershipResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ValueOwnershipRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::value_ownership(&inner, request).await };
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
                        let method = ValueOwnershipSvc(inner);
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
                "/provenance.metadata.v1.Query/ScopeSpecification" => {
                    #[allow(non_camel_case_types)]
                    struct ScopeSpecificationSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::ScopeSpecificationRequest>
                        for ScopeSpecificationSvc<T>
                    {
                        type Response = super::ScopeSpecificationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ScopeSpecificationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::scope_specification(&inner, request).await
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
                        let method = ScopeSpecificationSvc(inner);
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
                "/provenance.metadata.v1.Query/ScopeSpecificationsAll" => {
                    #[allow(non_camel_case_types)]
                    struct ScopeSpecificationsAllSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::ScopeSpecificationsAllRequest>
                        for ScopeSpecificationsAllSvc<T>
                    {
                        type Response = super::ScopeSpecificationsAllResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ScopeSpecificationsAllRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::scope_specifications_all(&inner, request).await
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
                        let method = ScopeSpecificationsAllSvc(inner);
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
                "/provenance.metadata.v1.Query/ContractSpecification" => {
                    #[allow(non_camel_case_types)]
                    struct ContractSpecificationSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::ContractSpecificationRequest>
                        for ContractSpecificationSvc<T>
                    {
                        type Response = super::ContractSpecificationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ContractSpecificationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::contract_specification(&inner, request).await
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
                        let method = ContractSpecificationSvc(inner);
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
                "/provenance.metadata.v1.Query/ContractSpecificationsAll" => {
                    #[allow(non_camel_case_types)]
                    struct ContractSpecificationsAllSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::ContractSpecificationsAllRequest>
                        for ContractSpecificationsAllSvc<T>
                    {
                        type Response = super::ContractSpecificationsAllResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ContractSpecificationsAllRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::contract_specifications_all(&inner, request).await
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
                        let method = ContractSpecificationsAllSvc(inner);
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
                "/provenance.metadata.v1.Query/RecordSpecificationsForContractSpecification" => {
                    #[allow(non_camel_case_types)]
                    struct RecordSpecificationsForContractSpecificationSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::RecordSpecificationsForContractSpecificationRequest,
                        > for RecordSpecificationsForContractSpecificationSvc<T>
                    {
                        type Response = super::RecordSpecificationsForContractSpecificationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RecordSpecificationsForContractSpecificationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::record_specifications_for_contract_specification(
                                    &inner, request,
                                )
                                .await
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
                        let method = RecordSpecificationsForContractSpecificationSvc(inner);
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
                "/provenance.metadata.v1.Query/RecordSpecification" => {
                    #[allow(non_camel_case_types)]
                    struct RecordSpecificationSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::RecordSpecificationRequest>
                        for RecordSpecificationSvc<T>
                    {
                        type Response = super::RecordSpecificationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RecordSpecificationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::record_specification(&inner, request).await
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
                        let method = RecordSpecificationSvc(inner);
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
                "/provenance.metadata.v1.Query/RecordSpecificationsAll" => {
                    #[allow(non_camel_case_types)]
                    struct RecordSpecificationsAllSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::RecordSpecificationsAllRequest>
                        for RecordSpecificationsAllSvc<T>
                    {
                        type Response = super::RecordSpecificationsAllResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RecordSpecificationsAllRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::record_specifications_all(&inner, request).await
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
                        let method = RecordSpecificationsAllSvc(inner);
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
                "/provenance.metadata.v1.Query/GetByAddr" => {
                    #[allow(non_camel_case_types)]
                    struct GetByAddrSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::GetByAddrRequest> for GetByAddrSvc<T> {
                        type Response = super::GetByAddrResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetByAddrRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::get_by_addr(&inner, request).await };
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
                        let method = GetByAddrSvc(inner);
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
                "/provenance.metadata.v1.Query/OSLocatorParams" => {
                    #[allow(non_camel_case_types)]
                    struct OSLocatorParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::OsLocatorParamsRequest>
                        for OSLocatorParamsSvc<T>
                    {
                        type Response = super::OsLocatorParamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OsLocatorParamsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::os_locator_params(&inner, request).await
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
                        let method = OSLocatorParamsSvc(inner);
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
                "/provenance.metadata.v1.Query/OSLocator" => {
                    #[allow(non_camel_case_types)]
                    struct OSLocatorSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::OsLocatorRequest> for OSLocatorSvc<T> {
                        type Response = super::OsLocatorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OsLocatorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::os_locator(&inner, request).await };
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
                        let method = OSLocatorSvc(inner);
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
                "/provenance.metadata.v1.Query/OSLocatorsByURI" => {
                    #[allow(non_camel_case_types)]
                    struct OSLocatorsByURISvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::OsLocatorsByUriRequest>
                        for OSLocatorsByURISvc<T>
                    {
                        type Response = super::OsLocatorsByUriResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OsLocatorsByUriRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::os_locators_by_uri(&inner, request).await
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
                        let method = OSLocatorsByURISvc(inner);
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
                "/provenance.metadata.v1.Query/OSLocatorsByScope" => {
                    #[allow(non_camel_case_types)]
                    struct OSLocatorsByScopeSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::OsLocatorsByScopeRequest>
                        for OSLocatorsByScopeSvc<T>
                    {
                        type Response = super::OsLocatorsByScopeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OsLocatorsByScopeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::os_locators_by_scope(&inner, request).await
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
                        let method = OSLocatorsByScopeSvc(inner);
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
                "/provenance.metadata.v1.Query/OSAllLocators" => {
                    #[allow(non_camel_case_types)]
                    struct OSAllLocatorsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::OsAllLocatorsRequest> for OSAllLocatorsSvc<T> {
                        type Response = super::OsAllLocatorsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OsAllLocatorsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::os_all_locators(&inner, request).await };
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
                        let method = OSAllLocatorsSvc(inner);
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
                "/provenance.metadata.v1.Query/AccountData" => {
                    #[allow(non_camel_case_types)]
                    struct AccountDataSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::AccountDataRequest> for AccountDataSvc<T> {
                        type Response = super::AccountDataResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountDataRequest>,
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
                "/provenance.metadata.v1.Query/ScopeNetAssetValues" => {
                    #[allow(non_camel_case_types)]
                    struct ScopeNetAssetValuesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryScopeNetAssetValuesRequest>
                        for ScopeNetAssetValuesSvc<T>
                    {
                        type Response = super::QueryScopeNetAssetValuesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryScopeNetAssetValuesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::scope_net_asset_values(&inner, request).await
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
                        let method = ScopeNetAssetValuesSvc(inner);
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
        const NAME: &'static str = "provenance.metadata.v1.Query";
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
        pub async fn write_scope(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWriteScopeRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgWriteScopeResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Msg/WriteScope");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.metadata.v1.Msg", "WriteScope"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_scope(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteScopeRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgDeleteScopeResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Msg/DeleteScope");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.metadata.v1.Msg", "DeleteScope"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_scope_data_access(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAddScopeDataAccessRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgAddScopeDataAccessResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Msg/AddScopeDataAccess",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Msg",
                "AddScopeDataAccess",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_scope_data_access(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteScopeDataAccessRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgDeleteScopeDataAccessResponse>,
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
                "/provenance.metadata.v1.Msg/DeleteScopeDataAccess",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Msg",
                "DeleteScopeDataAccess",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_scope_owner(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAddScopeOwnerRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgAddScopeOwnerResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Msg/AddScopeOwner");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Msg",
                "AddScopeOwner",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_scope_owner(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteScopeOwnerRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgDeleteScopeOwnerResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Msg/DeleteScopeOwner",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Msg",
                "DeleteScopeOwner",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_value_owners(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateValueOwnersRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdateValueOwnersResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Msg/UpdateValueOwners",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Msg",
                "UpdateValueOwners",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn migrate_value_owner(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMigrateValueOwnerRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgMigrateValueOwnerResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Msg/MigrateValueOwner",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Msg",
                "MigrateValueOwner",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn write_session(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWriteSessionRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgWriteSessionResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Msg/WriteSession");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Msg",
                "WriteSession",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn write_record(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWriteRecordRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgWriteRecordResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Msg/WriteRecord");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.metadata.v1.Msg", "WriteRecord"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_record(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteRecordRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgDeleteRecordResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Msg/DeleteRecord");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Msg",
                "DeleteRecord",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn write_scope_specification(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWriteScopeSpecificationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgWriteScopeSpecificationResponse>,
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
                "/provenance.metadata.v1.Msg/WriteScopeSpecification",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Msg",
                "WriteScopeSpecification",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_scope_specification(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteScopeSpecificationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgDeleteScopeSpecificationResponse>,
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
                "/provenance.metadata.v1.Msg/DeleteScopeSpecification",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Msg",
                "DeleteScopeSpecification",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn write_contract_specification(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWriteContractSpecificationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgWriteContractSpecificationResponse>,
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
                "/provenance.metadata.v1.Msg/WriteContractSpecification",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Msg",
                "WriteContractSpecification",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_contract_specification(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteContractSpecificationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgDeleteContractSpecificationResponse>,
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
                "/provenance.metadata.v1.Msg/DeleteContractSpecification",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Msg",
                "DeleteContractSpecification",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_contract_spec_to_scope_spec(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAddContractSpecToScopeSpecRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgAddContractSpecToScopeSpecResponse>,
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
                "/provenance.metadata.v1.Msg/AddContractSpecToScopeSpec",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Msg",
                "AddContractSpecToScopeSpec",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_contract_spec_from_scope_spec(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteContractSpecFromScopeSpecRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgDeleteContractSpecFromScopeSpecResponse>,
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
                "/provenance.metadata.v1.Msg/DeleteContractSpecFromScopeSpec",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Msg",
                "DeleteContractSpecFromScopeSpec",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn write_record_specification(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWriteRecordSpecificationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgWriteRecordSpecificationResponse>,
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
                "/provenance.metadata.v1.Msg/WriteRecordSpecification",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Msg",
                "WriteRecordSpecification",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_record_specification(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteRecordSpecificationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgDeleteRecordSpecificationResponse>,
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
                "/provenance.metadata.v1.Msg/DeleteRecordSpecification",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Msg",
                "DeleteRecordSpecification",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn bind_os_locator(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBindOsLocatorRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgBindOsLocatorResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Msg/BindOSLocator");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Msg",
                "BindOSLocator",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_os_locator(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteOsLocatorRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgDeleteOsLocatorResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Msg/DeleteOSLocator");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Msg",
                "DeleteOSLocator",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn modify_os_locator(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgModifyOsLocatorRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgModifyOsLocatorResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Msg/ModifyOSLocator");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Msg",
                "ModifyOSLocator",
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
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Msg/SetAccountData");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Msg",
                "SetAccountData",
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
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Msg/AddNetAssetValues",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.metadata.v1.Msg",
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
        async fn write_scope(
            &self,
            request: tonic::Request<super::MsgWriteScopeRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgWriteScopeResponse>, tonic::Status>;
        async fn delete_scope(
            &self,
            request: tonic::Request<super::MsgDeleteScopeRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgDeleteScopeResponse>, tonic::Status>;
        async fn add_scope_data_access(
            &self,
            request: tonic::Request<super::MsgAddScopeDataAccessRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgAddScopeDataAccessResponse>, tonic::Status>;
        async fn delete_scope_data_access(
            &self,
            request: tonic::Request<super::MsgDeleteScopeDataAccessRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgDeleteScopeDataAccessResponse>,
            tonic::Status,
        >;
        async fn add_scope_owner(
            &self,
            request: tonic::Request<super::MsgAddScopeOwnerRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgAddScopeOwnerResponse>, tonic::Status>;
        async fn delete_scope_owner(
            &self,
            request: tonic::Request<super::MsgDeleteScopeOwnerRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgDeleteScopeOwnerResponse>, tonic::Status>;
        async fn update_value_owners(
            &self,
            request: tonic::Request<super::MsgUpdateValueOwnersRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdateValueOwnersResponse>, tonic::Status>;
        async fn migrate_value_owner(
            &self,
            request: tonic::Request<super::MsgMigrateValueOwnerRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgMigrateValueOwnerResponse>, tonic::Status>;
        async fn write_session(
            &self,
            request: tonic::Request<super::MsgWriteSessionRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgWriteSessionResponse>, tonic::Status>;
        async fn write_record(
            &self,
            request: tonic::Request<super::MsgWriteRecordRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgWriteRecordResponse>, tonic::Status>;
        async fn delete_record(
            &self,
            request: tonic::Request<super::MsgDeleteRecordRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgDeleteRecordResponse>, tonic::Status>;
        async fn write_scope_specification(
            &self,
            request: tonic::Request<super::MsgWriteScopeSpecificationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgWriteScopeSpecificationResponse>,
            tonic::Status,
        >;
        async fn delete_scope_specification(
            &self,
            request: tonic::Request<super::MsgDeleteScopeSpecificationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgDeleteScopeSpecificationResponse>,
            tonic::Status,
        >;
        async fn write_contract_specification(
            &self,
            request: tonic::Request<super::MsgWriteContractSpecificationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgWriteContractSpecificationResponse>,
            tonic::Status,
        >;
        async fn delete_contract_specification(
            &self,
            request: tonic::Request<super::MsgDeleteContractSpecificationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgDeleteContractSpecificationResponse>,
            tonic::Status,
        >;
        async fn add_contract_spec_to_scope_spec(
            &self,
            request: tonic::Request<super::MsgAddContractSpecToScopeSpecRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgAddContractSpecToScopeSpecResponse>,
            tonic::Status,
        >;
        async fn delete_contract_spec_from_scope_spec(
            &self,
            request: tonic::Request<super::MsgDeleteContractSpecFromScopeSpecRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgDeleteContractSpecFromScopeSpecResponse>,
            tonic::Status,
        >;
        async fn write_record_specification(
            &self,
            request: tonic::Request<super::MsgWriteRecordSpecificationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgWriteRecordSpecificationResponse>,
            tonic::Status,
        >;
        async fn delete_record_specification(
            &self,
            request: tonic::Request<super::MsgDeleteRecordSpecificationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgDeleteRecordSpecificationResponse>,
            tonic::Status,
        >;
        async fn bind_os_locator(
            &self,
            request: tonic::Request<super::MsgBindOsLocatorRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgBindOsLocatorResponse>, tonic::Status>;
        async fn delete_os_locator(
            &self,
            request: tonic::Request<super::MsgDeleteOsLocatorRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgDeleteOsLocatorResponse>, tonic::Status>;
        async fn modify_os_locator(
            &self,
            request: tonic::Request<super::MsgModifyOsLocatorRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgModifyOsLocatorResponse>, tonic::Status>;
        async fn set_account_data(
            &self,
            request: tonic::Request<super::MsgSetAccountDataRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgSetAccountDataResponse>, tonic::Status>;
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
                "/provenance.metadata.v1.Msg/WriteScope" => {
                    #[allow(non_camel_case_types)]
                    struct WriteScopeSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgWriteScopeRequest> for WriteScopeSvc<T> {
                        type Response = super::MsgWriteScopeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWriteScopeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Msg>::write_scope(&inner, request).await };
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
                        let method = WriteScopeSvc(inner);
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
                "/provenance.metadata.v1.Msg/DeleteScope" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteScopeSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDeleteScopeRequest> for DeleteScopeSvc<T> {
                        type Response = super::MsgDeleteScopeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDeleteScopeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::delete_scope(&inner, request).await };
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
                        let method = DeleteScopeSvc(inner);
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
                "/provenance.metadata.v1.Msg/AddScopeDataAccess" => {
                    #[allow(non_camel_case_types)]
                    struct AddScopeDataAccessSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgAddScopeDataAccessRequest>
                        for AddScopeDataAccessSvc<T>
                    {
                        type Response = super::MsgAddScopeDataAccessResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAddScopeDataAccessRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::add_scope_data_access(&inner, request).await
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
                        let method = AddScopeDataAccessSvc(inner);
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
                "/provenance.metadata.v1.Msg/DeleteScopeDataAccess" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteScopeDataAccessSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDeleteScopeDataAccessRequest>
                        for DeleteScopeDataAccessSvc<T>
                    {
                        type Response = super::MsgDeleteScopeDataAccessResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDeleteScopeDataAccessRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::delete_scope_data_access(&inner, request).await
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
                        let method = DeleteScopeDataAccessSvc(inner);
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
                "/provenance.metadata.v1.Msg/AddScopeOwner" => {
                    #[allow(non_camel_case_types)]
                    struct AddScopeOwnerSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgAddScopeOwnerRequest> for AddScopeOwnerSvc<T> {
                        type Response = super::MsgAddScopeOwnerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAddScopeOwnerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::add_scope_owner(&inner, request).await };
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
                        let method = AddScopeOwnerSvc(inner);
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
                "/provenance.metadata.v1.Msg/DeleteScopeOwner" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteScopeOwnerSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDeleteScopeOwnerRequest>
                        for DeleteScopeOwnerSvc<T>
                    {
                        type Response = super::MsgDeleteScopeOwnerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDeleteScopeOwnerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::delete_scope_owner(&inner, request).await
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
                        let method = DeleteScopeOwnerSvc(inner);
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
                "/provenance.metadata.v1.Msg/UpdateValueOwners" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateValueOwnersSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpdateValueOwnersRequest>
                        for UpdateValueOwnersSvc<T>
                    {
                        type Response = super::MsgUpdateValueOwnersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateValueOwnersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::update_value_owners(&inner, request).await
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
                        let method = UpdateValueOwnersSvc(inner);
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
                "/provenance.metadata.v1.Msg/MigrateValueOwner" => {
                    #[allow(non_camel_case_types)]
                    struct MigrateValueOwnerSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgMigrateValueOwnerRequest>
                        for MigrateValueOwnerSvc<T>
                    {
                        type Response = super::MsgMigrateValueOwnerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgMigrateValueOwnerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::migrate_value_owner(&inner, request).await
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
                        let method = MigrateValueOwnerSvc(inner);
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
                "/provenance.metadata.v1.Msg/WriteSession" => {
                    #[allow(non_camel_case_types)]
                    struct WriteSessionSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgWriteSessionRequest> for WriteSessionSvc<T> {
                        type Response = super::MsgWriteSessionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWriteSessionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::write_session(&inner, request).await };
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
                        let method = WriteSessionSvc(inner);
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
                "/provenance.metadata.v1.Msg/WriteRecord" => {
                    #[allow(non_camel_case_types)]
                    struct WriteRecordSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgWriteRecordRequest> for WriteRecordSvc<T> {
                        type Response = super::MsgWriteRecordResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWriteRecordRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::write_record(&inner, request).await };
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
                        let method = WriteRecordSvc(inner);
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
                "/provenance.metadata.v1.Msg/DeleteRecord" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRecordSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDeleteRecordRequest> for DeleteRecordSvc<T> {
                        type Response = super::MsgDeleteRecordResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDeleteRecordRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::delete_record(&inner, request).await };
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
                        let method = DeleteRecordSvc(inner);
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
                "/provenance.metadata.v1.Msg/WriteScopeSpecification" => {
                    #[allow(non_camel_case_types)]
                    struct WriteScopeSpecificationSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgWriteScopeSpecificationRequest>
                        for WriteScopeSpecificationSvc<T>
                    {
                        type Response = super::MsgWriteScopeSpecificationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWriteScopeSpecificationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::write_scope_specification(&inner, request).await
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
                        let method = WriteScopeSpecificationSvc(inner);
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
                "/provenance.metadata.v1.Msg/DeleteScopeSpecification" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteScopeSpecificationSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgDeleteScopeSpecificationRequest>
                        for DeleteScopeSpecificationSvc<T>
                    {
                        type Response = super::MsgDeleteScopeSpecificationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDeleteScopeSpecificationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::delete_scope_specification(&inner, request).await
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
                        let method = DeleteScopeSpecificationSvc(inner);
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
                "/provenance.metadata.v1.Msg/WriteContractSpecification" => {
                    #[allow(non_camel_case_types)]
                    struct WriteContractSpecificationSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgWriteContractSpecificationRequest>
                        for WriteContractSpecificationSvc<T>
                    {
                        type Response = super::MsgWriteContractSpecificationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWriteContractSpecificationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::write_contract_specification(&inner, request).await
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
                        let method = WriteContractSpecificationSvc(inner);
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
                "/provenance.metadata.v1.Msg/DeleteContractSpecification" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteContractSpecificationSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgDeleteContractSpecificationRequest>
                        for DeleteContractSpecificationSvc<T>
                    {
                        type Response = super::MsgDeleteContractSpecificationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDeleteContractSpecificationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::delete_contract_specification(&inner, request).await
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
                        let method = DeleteContractSpecificationSvc(inner);
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
                "/provenance.metadata.v1.Msg/AddContractSpecToScopeSpec" => {
                    #[allow(non_camel_case_types)]
                    struct AddContractSpecToScopeSpecSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgAddContractSpecToScopeSpecRequest>
                        for AddContractSpecToScopeSpecSvc<T>
                    {
                        type Response = super::MsgAddContractSpecToScopeSpecResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAddContractSpecToScopeSpecRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::add_contract_spec_to_scope_spec(&inner, request).await
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
                        let method = AddContractSpecToScopeSpecSvc(inner);
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
                "/provenance.metadata.v1.Msg/DeleteContractSpecFromScopeSpec" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteContractSpecFromScopeSpecSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<
                            super::MsgDeleteContractSpecFromScopeSpecRequest,
                        > for DeleteContractSpecFromScopeSpecSvc<T>
                    {
                        type Response = super::MsgDeleteContractSpecFromScopeSpecResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::MsgDeleteContractSpecFromScopeSpecRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::delete_contract_spec_from_scope_spec(&inner, request)
                                    .await
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
                        let method = DeleteContractSpecFromScopeSpecSvc(inner);
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
                "/provenance.metadata.v1.Msg/WriteRecordSpecification" => {
                    #[allow(non_camel_case_types)]
                    struct WriteRecordSpecificationSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgWriteRecordSpecificationRequest>
                        for WriteRecordSpecificationSvc<T>
                    {
                        type Response = super::MsgWriteRecordSpecificationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWriteRecordSpecificationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::write_record_specification(&inner, request).await
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
                        let method = WriteRecordSpecificationSvc(inner);
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
                "/provenance.metadata.v1.Msg/DeleteRecordSpecification" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRecordSpecificationSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgDeleteRecordSpecificationRequest>
                        for DeleteRecordSpecificationSvc<T>
                    {
                        type Response = super::MsgDeleteRecordSpecificationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDeleteRecordSpecificationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::delete_record_specification(&inner, request).await
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
                        let method = DeleteRecordSpecificationSvc(inner);
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
                "/provenance.metadata.v1.Msg/BindOSLocator" => {
                    #[allow(non_camel_case_types)]
                    struct BindOSLocatorSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgBindOsLocatorRequest> for BindOSLocatorSvc<T> {
                        type Response = super::MsgBindOsLocatorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgBindOsLocatorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::bind_os_locator(&inner, request).await };
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
                        let method = BindOSLocatorSvc(inner);
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
                "/provenance.metadata.v1.Msg/DeleteOSLocator" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteOSLocatorSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDeleteOsLocatorRequest>
                        for DeleteOSLocatorSvc<T>
                    {
                        type Response = super::MsgDeleteOsLocatorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDeleteOsLocatorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::delete_os_locator(&inner, request).await };
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
                        let method = DeleteOSLocatorSvc(inner);
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
                "/provenance.metadata.v1.Msg/ModifyOSLocator" => {
                    #[allow(non_camel_case_types)]
                    struct ModifyOSLocatorSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgModifyOsLocatorRequest>
                        for ModifyOSLocatorSvc<T>
                    {
                        type Response = super::MsgModifyOsLocatorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgModifyOsLocatorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::modify_os_locator(&inner, request).await };
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
                        let method = ModifyOSLocatorSvc(inner);
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
                "/provenance.metadata.v1.Msg/SetAccountData" => {
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
                "/provenance.metadata.v1.Msg/AddNetAssetValues" => {
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
        const NAME: &'static str = "provenance.metadata.v1.Msg";
    }
}
