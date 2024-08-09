use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
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
        pub async fn create_ask(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateAskRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgCreateAskResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Msg/CreateAsk");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.exchange.v1.Msg", "CreateAsk"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_bid(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateBidRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgCreateBidResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Msg/CreateBid");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.exchange.v1.Msg", "CreateBid"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn commit_funds(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCommitFundsRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgCommitFundsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Msg/CommitFunds");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.exchange.v1.Msg", "CommitFunds"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn cancel_order(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCancelOrderRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgCancelOrderResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Msg/CancelOrder");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.exchange.v1.Msg", "CancelOrder"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn fill_bids(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgFillBidsRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgFillBidsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Msg/FillBids");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.exchange.v1.Msg", "FillBids"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn fill_asks(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgFillAsksRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgFillAsksResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Msg/FillAsks");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.exchange.v1.Msg", "FillAsks"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn market_settle(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMarketSettleRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgMarketSettleResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Msg/MarketSettle");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "MarketSettle",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn market_commitment_settle(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMarketCommitmentSettleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgMarketCommitmentSettleResponse>,
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
                "/provenance.exchange.v1.Msg/MarketCommitmentSettle",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "MarketCommitmentSettle",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn market_release_commitments(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMarketReleaseCommitmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgMarketReleaseCommitmentsResponse>,
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
                "/provenance.exchange.v1.Msg/MarketReleaseCommitments",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "MarketReleaseCommitments",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn market_set_order_external_id(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMarketSetOrderExternalIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgMarketSetOrderExternalIdResponse>,
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
                "/provenance.exchange.v1.Msg/MarketSetOrderExternalID",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "MarketSetOrderExternalID",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn market_withdraw(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMarketWithdrawRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgMarketWithdrawResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Msg/MarketWithdraw");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "MarketWithdraw",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn market_update_details(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMarketUpdateDetailsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgMarketUpdateDetailsResponse>,
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
                "/provenance.exchange.v1.Msg/MarketUpdateDetails",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "MarketUpdateDetails",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn market_update_enabled(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMarketUpdateEnabledRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgMarketUpdateEnabledResponse>,
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
                "/provenance.exchange.v1.Msg/MarketUpdateEnabled",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "MarketUpdateEnabled",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn market_update_accepting_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMarketUpdateAcceptingOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgMarketUpdateAcceptingOrdersResponse>,
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
                "/provenance.exchange.v1.Msg/MarketUpdateAcceptingOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "MarketUpdateAcceptingOrders",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn market_update_user_settle(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMarketUpdateUserSettleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgMarketUpdateUserSettleResponse>,
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
                "/provenance.exchange.v1.Msg/MarketUpdateUserSettle",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "MarketUpdateUserSettle",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn market_update_accepting_commitments(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMarketUpdateAcceptingCommitmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgMarketUpdateAcceptingCommitmentsResponse>,
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
                "/provenance.exchange.v1.Msg/MarketUpdateAcceptingCommitments",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "MarketUpdateAcceptingCommitments",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn market_update_intermediary_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMarketUpdateIntermediaryDenomRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgMarketUpdateIntermediaryDenomResponse>,
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
                "/provenance.exchange.v1.Msg/MarketUpdateIntermediaryDenom",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "MarketUpdateIntermediaryDenom",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn market_manage_permissions(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMarketManagePermissionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgMarketManagePermissionsResponse>,
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
                "/provenance.exchange.v1.Msg/MarketManagePermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "MarketManagePermissions",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn market_manage_req_attrs(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMarketManageReqAttrsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgMarketManageReqAttrsResponse>,
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
                "/provenance.exchange.v1.Msg/MarketManageReqAttrs",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "MarketManageReqAttrs",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_payment(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreatePaymentRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgCreatePaymentResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Msg/CreatePayment");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "CreatePayment",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn accept_payment(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAcceptPaymentRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgAcceptPaymentResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Msg/AcceptPayment");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "AcceptPayment",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn reject_payment(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRejectPaymentRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgRejectPaymentResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Msg/RejectPayment");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "RejectPayment",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn reject_payments(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRejectPaymentsRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgRejectPaymentsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Msg/RejectPayments");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "RejectPayments",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn cancel_payments(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCancelPaymentsRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgCancelPaymentsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Msg/CancelPayments");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "CancelPayments",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn change_payment_target(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgChangePaymentTargetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChangePaymentTargetResponse>,
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
                "/provenance.exchange.v1.Msg/ChangePaymentTarget",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "ChangePaymentTarget",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn gov_create_market(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgGovCreateMarketRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgGovCreateMarketResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Msg/GovCreateMarket");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "GovCreateMarket",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn gov_manage_fees(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgGovManageFeesRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgGovManageFeesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Msg/GovManageFees");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "GovManageFees",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn gov_close_market(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgGovCloseMarketRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgGovCloseMarketResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Msg/GovCloseMarket");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "GovCloseMarket",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn gov_update_params(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgGovUpdateParamsRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgGovUpdateParamsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Msg/GovUpdateParams");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Msg",
                "GovUpdateParams",
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
        async fn create_ask(
            &self,
            request: tonic::Request<super::MsgCreateAskRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgCreateAskResponse>, tonic::Status>;
        async fn create_bid(
            &self,
            request: tonic::Request<super::MsgCreateBidRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgCreateBidResponse>, tonic::Status>;
        async fn commit_funds(
            &self,
            request: tonic::Request<super::MsgCommitFundsRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgCommitFundsResponse>, tonic::Status>;
        async fn cancel_order(
            &self,
            request: tonic::Request<super::MsgCancelOrderRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgCancelOrderResponse>, tonic::Status>;
        async fn fill_bids(
            &self,
            request: tonic::Request<super::MsgFillBidsRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgFillBidsResponse>, tonic::Status>;
        async fn fill_asks(
            &self,
            request: tonic::Request<super::MsgFillAsksRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgFillAsksResponse>, tonic::Status>;
        async fn market_settle(
            &self,
            request: tonic::Request<super::MsgMarketSettleRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgMarketSettleResponse>, tonic::Status>;
        async fn market_commitment_settle(
            &self,
            request: tonic::Request<super::MsgMarketCommitmentSettleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgMarketCommitmentSettleResponse>,
            tonic::Status,
        >;
        async fn market_release_commitments(
            &self,
            request: tonic::Request<super::MsgMarketReleaseCommitmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgMarketReleaseCommitmentsResponse>,
            tonic::Status,
        >;
        async fn market_set_order_external_id(
            &self,
            request: tonic::Request<super::MsgMarketSetOrderExternalIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgMarketSetOrderExternalIdResponse>,
            tonic::Status,
        >;
        async fn market_withdraw(
            &self,
            request: tonic::Request<super::MsgMarketWithdrawRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgMarketWithdrawResponse>, tonic::Status>;
        async fn market_update_details(
            &self,
            request: tonic::Request<super::MsgMarketUpdateDetailsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgMarketUpdateDetailsResponse>,
            tonic::Status,
        >;
        async fn market_update_enabled(
            &self,
            request: tonic::Request<super::MsgMarketUpdateEnabledRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgMarketUpdateEnabledResponse>,
            tonic::Status,
        >;
        async fn market_update_accepting_orders(
            &self,
            request: tonic::Request<super::MsgMarketUpdateAcceptingOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgMarketUpdateAcceptingOrdersResponse>,
            tonic::Status,
        >;
        async fn market_update_user_settle(
            &self,
            request: tonic::Request<super::MsgMarketUpdateUserSettleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgMarketUpdateUserSettleResponse>,
            tonic::Status,
        >;
        async fn market_update_accepting_commitments(
            &self,
            request: tonic::Request<super::MsgMarketUpdateAcceptingCommitmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgMarketUpdateAcceptingCommitmentsResponse>,
            tonic::Status,
        >;
        async fn market_update_intermediary_denom(
            &self,
            request: tonic::Request<super::MsgMarketUpdateIntermediaryDenomRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgMarketUpdateIntermediaryDenomResponse>,
            tonic::Status,
        >;
        async fn market_manage_permissions(
            &self,
            request: tonic::Request<super::MsgMarketManagePermissionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgMarketManagePermissionsResponse>,
            tonic::Status,
        >;
        async fn market_manage_req_attrs(
            &self,
            request: tonic::Request<super::MsgMarketManageReqAttrsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgMarketManageReqAttrsResponse>,
            tonic::Status,
        >;
        async fn create_payment(
            &self,
            request: tonic::Request<super::MsgCreatePaymentRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgCreatePaymentResponse>, tonic::Status>;
        async fn accept_payment(
            &self,
            request: tonic::Request<super::MsgAcceptPaymentRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgAcceptPaymentResponse>, tonic::Status>;
        async fn reject_payment(
            &self,
            request: tonic::Request<super::MsgRejectPaymentRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgRejectPaymentResponse>, tonic::Status>;
        async fn reject_payments(
            &self,
            request: tonic::Request<super::MsgRejectPaymentsRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgRejectPaymentsResponse>, tonic::Status>;
        async fn cancel_payments(
            &self,
            request: tonic::Request<super::MsgCancelPaymentsRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgCancelPaymentsResponse>, tonic::Status>;
        async fn change_payment_target(
            &self,
            request: tonic::Request<super::MsgChangePaymentTargetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChangePaymentTargetResponse>,
            tonic::Status,
        >;
        async fn gov_create_market(
            &self,
            request: tonic::Request<super::MsgGovCreateMarketRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgGovCreateMarketResponse>, tonic::Status>;
        async fn gov_manage_fees(
            &self,
            request: tonic::Request<super::MsgGovManageFeesRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgGovManageFeesResponse>, tonic::Status>;
        async fn gov_close_market(
            &self,
            request: tonic::Request<super::MsgGovCloseMarketRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgGovCloseMarketResponse>, tonic::Status>;
        async fn gov_update_params(
            &self,
            request: tonic::Request<super::MsgGovUpdateParamsRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgGovUpdateParamsResponse>, tonic::Status>;
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
                "/provenance.exchange.v1.Msg/CreateAsk" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAskSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCreateAskRequest> for CreateAskSvc<T> {
                        type Response = super::MsgCreateAskResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreateAskRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Msg>::create_ask(&inner, request).await };
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
                        let method = CreateAskSvc(inner);
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
                "/provenance.exchange.v1.Msg/CreateBid" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBidSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCreateBidRequest> for CreateBidSvc<T> {
                        type Response = super::MsgCreateBidResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreateBidRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Msg>::create_bid(&inner, request).await };
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
                        let method = CreateBidSvc(inner);
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
                "/provenance.exchange.v1.Msg/CommitFunds" => {
                    #[allow(non_camel_case_types)]
                    struct CommitFundsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCommitFundsRequest> for CommitFundsSvc<T> {
                        type Response = super::MsgCommitFundsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCommitFundsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::commit_funds(&inner, request).await };
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
                        let method = CommitFundsSvc(inner);
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
                "/provenance.exchange.v1.Msg/CancelOrder" => {
                    #[allow(non_camel_case_types)]
                    struct CancelOrderSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCancelOrderRequest> for CancelOrderSvc<T> {
                        type Response = super::MsgCancelOrderResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCancelOrderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::cancel_order(&inner, request).await };
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
                        let method = CancelOrderSvc(inner);
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
                "/provenance.exchange.v1.Msg/FillBids" => {
                    #[allow(non_camel_case_types)]
                    struct FillBidsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgFillBidsRequest> for FillBidsSvc<T> {
                        type Response = super::MsgFillBidsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgFillBidsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Msg>::fill_bids(&inner, request).await };
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
                        let method = FillBidsSvc(inner);
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
                "/provenance.exchange.v1.Msg/FillAsks" => {
                    #[allow(non_camel_case_types)]
                    struct FillAsksSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgFillAsksRequest> for FillAsksSvc<T> {
                        type Response = super::MsgFillAsksResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgFillAsksRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Msg>::fill_asks(&inner, request).await };
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
                        let method = FillAsksSvc(inner);
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
                "/provenance.exchange.v1.Msg/MarketSettle" => {
                    #[allow(non_camel_case_types)]
                    struct MarketSettleSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgMarketSettleRequest> for MarketSettleSvc<T> {
                        type Response = super::MsgMarketSettleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgMarketSettleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::market_settle(&inner, request).await };
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
                        let method = MarketSettleSvc(inner);
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
                "/provenance.exchange.v1.Msg/MarketCommitmentSettle" => {
                    #[allow(non_camel_case_types)]
                    struct MarketCommitmentSettleSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgMarketCommitmentSettleRequest>
                        for MarketCommitmentSettleSvc<T>
                    {
                        type Response = super::MsgMarketCommitmentSettleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgMarketCommitmentSettleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::market_commitment_settle(&inner, request).await
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
                        let method = MarketCommitmentSettleSvc(inner);
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
                "/provenance.exchange.v1.Msg/MarketReleaseCommitments" => {
                    #[allow(non_camel_case_types)]
                    struct MarketReleaseCommitmentsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgMarketReleaseCommitmentsRequest>
                        for MarketReleaseCommitmentsSvc<T>
                    {
                        type Response = super::MsgMarketReleaseCommitmentsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgMarketReleaseCommitmentsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::market_release_commitments(&inner, request).await
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
                        let method = MarketReleaseCommitmentsSvc(inner);
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
                "/provenance.exchange.v1.Msg/MarketSetOrderExternalID" => {
                    #[allow(non_camel_case_types)]
                    struct MarketSetOrderExternalIDSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgMarketSetOrderExternalIdRequest>
                        for MarketSetOrderExternalIDSvc<T>
                    {
                        type Response = super::MsgMarketSetOrderExternalIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgMarketSetOrderExternalIdRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::market_set_order_external_id(&inner, request).await
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
                        let method = MarketSetOrderExternalIDSvc(inner);
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
                "/provenance.exchange.v1.Msg/MarketWithdraw" => {
                    #[allow(non_camel_case_types)]
                    struct MarketWithdrawSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgMarketWithdrawRequest> for MarketWithdrawSvc<T> {
                        type Response = super::MsgMarketWithdrawResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgMarketWithdrawRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::market_withdraw(&inner, request).await };
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
                        let method = MarketWithdrawSvc(inner);
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
                "/provenance.exchange.v1.Msg/MarketUpdateDetails" => {
                    #[allow(non_camel_case_types)]
                    struct MarketUpdateDetailsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgMarketUpdateDetailsRequest>
                        for MarketUpdateDetailsSvc<T>
                    {
                        type Response = super::MsgMarketUpdateDetailsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgMarketUpdateDetailsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::market_update_details(&inner, request).await
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
                        let method = MarketUpdateDetailsSvc(inner);
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
                "/provenance.exchange.v1.Msg/MarketUpdateEnabled" => {
                    #[allow(non_camel_case_types)]
                    struct MarketUpdateEnabledSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgMarketUpdateEnabledRequest>
                        for MarketUpdateEnabledSvc<T>
                    {
                        type Response = super::MsgMarketUpdateEnabledResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgMarketUpdateEnabledRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::market_update_enabled(&inner, request).await
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
                        let method = MarketUpdateEnabledSvc(inner);
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
                "/provenance.exchange.v1.Msg/MarketUpdateAcceptingOrders" => {
                    #[allow(non_camel_case_types)]
                    struct MarketUpdateAcceptingOrdersSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgMarketUpdateAcceptingOrdersRequest>
                        for MarketUpdateAcceptingOrdersSvc<T>
                    {
                        type Response = super::MsgMarketUpdateAcceptingOrdersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgMarketUpdateAcceptingOrdersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::market_update_accepting_orders(&inner, request).await
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
                        let method = MarketUpdateAcceptingOrdersSvc(inner);
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
                "/provenance.exchange.v1.Msg/MarketUpdateUserSettle" => {
                    #[allow(non_camel_case_types)]
                    struct MarketUpdateUserSettleSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgMarketUpdateUserSettleRequest>
                        for MarketUpdateUserSettleSvc<T>
                    {
                        type Response = super::MsgMarketUpdateUserSettleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgMarketUpdateUserSettleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::market_update_user_settle(&inner, request).await
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
                        let method = MarketUpdateUserSettleSvc(inner);
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
                "/provenance.exchange.v1.Msg/MarketUpdateAcceptingCommitments" => {
                    #[allow(non_camel_case_types)]
                    struct MarketUpdateAcceptingCommitmentsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<
                            super::MsgMarketUpdateAcceptingCommitmentsRequest,
                        > for MarketUpdateAcceptingCommitmentsSvc<T>
                    {
                        type Response = super::MsgMarketUpdateAcceptingCommitmentsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::MsgMarketUpdateAcceptingCommitmentsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::market_update_accepting_commitments(&inner, request)
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
                        let method = MarketUpdateAcceptingCommitmentsSvc(inner);
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
                "/provenance.exchange.v1.Msg/MarketUpdateIntermediaryDenom" => {
                    #[allow(non_camel_case_types)]
                    struct MarketUpdateIntermediaryDenomSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgMarketUpdateIntermediaryDenomRequest>
                        for MarketUpdateIntermediaryDenomSvc<T>
                    {
                        type Response = super::MsgMarketUpdateIntermediaryDenomResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgMarketUpdateIntermediaryDenomRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::market_update_intermediary_denom(&inner, request).await
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
                        let method = MarketUpdateIntermediaryDenomSvc(inner);
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
                "/provenance.exchange.v1.Msg/MarketManagePermissions" => {
                    #[allow(non_camel_case_types)]
                    struct MarketManagePermissionsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgMarketManagePermissionsRequest>
                        for MarketManagePermissionsSvc<T>
                    {
                        type Response = super::MsgMarketManagePermissionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgMarketManagePermissionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::market_manage_permissions(&inner, request).await
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
                        let method = MarketManagePermissionsSvc(inner);
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
                "/provenance.exchange.v1.Msg/MarketManageReqAttrs" => {
                    #[allow(non_camel_case_types)]
                    struct MarketManageReqAttrsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgMarketManageReqAttrsRequest>
                        for MarketManageReqAttrsSvc<T>
                    {
                        type Response = super::MsgMarketManageReqAttrsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgMarketManageReqAttrsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::market_manage_req_attrs(&inner, request).await
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
                        let method = MarketManageReqAttrsSvc(inner);
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
                "/provenance.exchange.v1.Msg/CreatePayment" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePaymentSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCreatePaymentRequest> for CreatePaymentSvc<T> {
                        type Response = super::MsgCreatePaymentResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreatePaymentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::create_payment(&inner, request).await };
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
                        let method = CreatePaymentSvc(inner);
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
                "/provenance.exchange.v1.Msg/AcceptPayment" => {
                    #[allow(non_camel_case_types)]
                    struct AcceptPaymentSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgAcceptPaymentRequest> for AcceptPaymentSvc<T> {
                        type Response = super::MsgAcceptPaymentResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAcceptPaymentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::accept_payment(&inner, request).await };
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
                        let method = AcceptPaymentSvc(inner);
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
                "/provenance.exchange.v1.Msg/RejectPayment" => {
                    #[allow(non_camel_case_types)]
                    struct RejectPaymentSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgRejectPaymentRequest> for RejectPaymentSvc<T> {
                        type Response = super::MsgRejectPaymentResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgRejectPaymentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::reject_payment(&inner, request).await };
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
                        let method = RejectPaymentSvc(inner);
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
                "/provenance.exchange.v1.Msg/RejectPayments" => {
                    #[allow(non_camel_case_types)]
                    struct RejectPaymentsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgRejectPaymentsRequest> for RejectPaymentsSvc<T> {
                        type Response = super::MsgRejectPaymentsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgRejectPaymentsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::reject_payments(&inner, request).await };
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
                        let method = RejectPaymentsSvc(inner);
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
                "/provenance.exchange.v1.Msg/CancelPayments" => {
                    #[allow(non_camel_case_types)]
                    struct CancelPaymentsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCancelPaymentsRequest> for CancelPaymentsSvc<T> {
                        type Response = super::MsgCancelPaymentsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCancelPaymentsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::cancel_payments(&inner, request).await };
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
                        let method = CancelPaymentsSvc(inner);
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
                "/provenance.exchange.v1.Msg/ChangePaymentTarget" => {
                    #[allow(non_camel_case_types)]
                    struct ChangePaymentTargetSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgChangePaymentTargetRequest>
                        for ChangePaymentTargetSvc<T>
                    {
                        type Response = super::MsgChangePaymentTargetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgChangePaymentTargetRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::change_payment_target(&inner, request).await
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
                        let method = ChangePaymentTargetSvc(inner);
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
                "/provenance.exchange.v1.Msg/GovCreateMarket" => {
                    #[allow(non_camel_case_types)]
                    struct GovCreateMarketSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgGovCreateMarketRequest>
                        for GovCreateMarketSvc<T>
                    {
                        type Response = super::MsgGovCreateMarketResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgGovCreateMarketRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::gov_create_market(&inner, request).await };
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
                        let method = GovCreateMarketSvc(inner);
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
                "/provenance.exchange.v1.Msg/GovManageFees" => {
                    #[allow(non_camel_case_types)]
                    struct GovManageFeesSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgGovManageFeesRequest> for GovManageFeesSvc<T> {
                        type Response = super::MsgGovManageFeesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgGovManageFeesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::gov_manage_fees(&inner, request).await };
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
                        let method = GovManageFeesSvc(inner);
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
                "/provenance.exchange.v1.Msg/GovCloseMarket" => {
                    #[allow(non_camel_case_types)]
                    struct GovCloseMarketSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgGovCloseMarketRequest> for GovCloseMarketSvc<T> {
                        type Response = super::MsgGovCloseMarketResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgGovCloseMarketRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::gov_close_market(&inner, request).await };
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
                        let method = GovCloseMarketSvc(inner);
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
                "/provenance.exchange.v1.Msg/GovUpdateParams" => {
                    #[allow(non_camel_case_types)]
                    struct GovUpdateParamsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgGovUpdateParamsRequest>
                        for GovUpdateParamsSvc<T>
                    {
                        type Response = super::MsgGovUpdateParamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgGovUpdateParamsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::gov_update_params(&inner, request).await };
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
                        let method = GovUpdateParamsSvc(inner);
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
        const NAME: &'static str = "provenance.exchange.v1.Msg";
    }
}
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
        pub async fn order_fee_calc(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryOrderFeeCalcRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryOrderFeeCalcResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Query/OrderFeeCalc");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Query",
                "OrderFeeCalc",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_order(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetOrderRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetOrderResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Query/GetOrder");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.exchange.v1.Query", "GetOrder"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_order_by_external_id(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetOrderByExternalIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetOrderByExternalIdResponse>,
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
                "/provenance.exchange.v1.Query/GetOrderByExternalID",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Query",
                "GetOrderByExternalID",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_market_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetMarketOrdersRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetMarketOrdersResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.exchange.v1.Query/GetMarketOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Query",
                "GetMarketOrders",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_owner_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetOwnerOrdersRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetOwnerOrdersResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.exchange.v1.Query/GetOwnerOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Query",
                "GetOwnerOrders",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_asset_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetAssetOrdersRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetAssetOrdersResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.exchange.v1.Query/GetAssetOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Query",
                "GetAssetOrders",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_all_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetAllOrdersRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetAllOrdersResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Query/GetAllOrders");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Query",
                "GetAllOrders",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_commitment(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetCommitmentRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetCommitmentResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Query/GetCommitment");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Query",
                "GetCommitment",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_account_commitments(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetAccountCommitmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetAccountCommitmentsResponse>,
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
                "/provenance.exchange.v1.Query/GetAccountCommitments",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Query",
                "GetAccountCommitments",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_market_commitments(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetMarketCommitmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetMarketCommitmentsResponse>,
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
                "/provenance.exchange.v1.Query/GetMarketCommitments",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Query",
                "GetMarketCommitments",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_all_commitments(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetAllCommitmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetAllCommitmentsResponse>,
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
                "/provenance.exchange.v1.Query/GetAllCommitments",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Query",
                "GetAllCommitments",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_market(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetMarketRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetMarketResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Query/GetMarket");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.exchange.v1.Query", "GetMarket"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_all_markets(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetAllMarketsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetAllMarketsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Query/GetAllMarkets");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Query",
                "GetAllMarkets",
            ));
            self.inner.unary(req, path, codec).await
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
            let path = http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Query/Params");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("provenance.exchange.v1.Query", "Params"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn commitment_settlement_fee_calc(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCommitmentSettlementFeeCalcRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryCommitmentSettlementFeeCalcResponse>,
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
                "/provenance.exchange.v1.Query/CommitmentSettlementFeeCalc",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Query",
                "CommitmentSettlementFeeCalc",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn validate_create_market(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValidateCreateMarketRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryValidateCreateMarketResponse>,
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
                "/provenance.exchange.v1.Query/ValidateCreateMarket",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Query",
                "ValidateCreateMarket",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn validate_market(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValidateMarketRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryValidateMarketResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.exchange.v1.Query/ValidateMarket",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Query",
                "ValidateMarket",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn validate_manage_fees(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValidateManageFeesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryValidateManageFeesResponse>,
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
                "/provenance.exchange.v1.Query/ValidateManageFees",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Query",
                "ValidateManageFees",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_payment(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetPaymentRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetPaymentResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.exchange.v1.Query/GetPayment");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Query",
                "GetPayment",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_payments_with_source(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetPaymentsWithSourceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetPaymentsWithSourceResponse>,
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
                "/provenance.exchange.v1.Query/GetPaymentsWithSource",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Query",
                "GetPaymentsWithSource",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_payments_with_target(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetPaymentsWithTargetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetPaymentsWithTargetResponse>,
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
                "/provenance.exchange.v1.Query/GetPaymentsWithTarget",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Query",
                "GetPaymentsWithTarget",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_all_payments(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetAllPaymentsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetAllPaymentsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.exchange.v1.Query/GetAllPayments",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Query",
                "GetAllPayments",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn payment_fee_calc(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPaymentFeeCalcRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryPaymentFeeCalcResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.exchange.v1.Query/PaymentFeeCalc",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "provenance.exchange.v1.Query",
                "PaymentFeeCalc",
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
        async fn order_fee_calc(
            &self,
            request: tonic::Request<super::QueryOrderFeeCalcRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryOrderFeeCalcResponse>, tonic::Status>;
        async fn get_order(
            &self,
            request: tonic::Request<super::QueryGetOrderRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetOrderResponse>, tonic::Status>;
        async fn get_order_by_external_id(
            &self,
            request: tonic::Request<super::QueryGetOrderByExternalIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetOrderByExternalIdResponse>,
            tonic::Status,
        >;
        async fn get_market_orders(
            &self,
            request: tonic::Request<super::QueryGetMarketOrdersRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetMarketOrdersResponse>, tonic::Status>;
        async fn get_owner_orders(
            &self,
            request: tonic::Request<super::QueryGetOwnerOrdersRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetOwnerOrdersResponse>, tonic::Status>;
        async fn get_asset_orders(
            &self,
            request: tonic::Request<super::QueryGetAssetOrdersRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetAssetOrdersResponse>, tonic::Status>;
        async fn get_all_orders(
            &self,
            request: tonic::Request<super::QueryGetAllOrdersRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetAllOrdersResponse>, tonic::Status>;
        async fn get_commitment(
            &self,
            request: tonic::Request<super::QueryGetCommitmentRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetCommitmentResponse>, tonic::Status>;
        async fn get_account_commitments(
            &self,
            request: tonic::Request<super::QueryGetAccountCommitmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetAccountCommitmentsResponse>,
            tonic::Status,
        >;
        async fn get_market_commitments(
            &self,
            request: tonic::Request<super::QueryGetMarketCommitmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetMarketCommitmentsResponse>,
            tonic::Status,
        >;
        async fn get_all_commitments(
            &self,
            request: tonic::Request<super::QueryGetAllCommitmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetAllCommitmentsResponse>,
            tonic::Status,
        >;
        async fn get_market(
            &self,
            request: tonic::Request<super::QueryGetMarketRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetMarketResponse>, tonic::Status>;
        async fn get_all_markets(
            &self,
            request: tonic::Request<super::QueryGetAllMarketsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetAllMarketsResponse>, tonic::Status>;
        async fn params(
            &self,
            request: tonic::Request<super::QueryParamsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>;
        async fn commitment_settlement_fee_calc(
            &self,
            request: tonic::Request<super::QueryCommitmentSettlementFeeCalcRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryCommitmentSettlementFeeCalcResponse>,
            tonic::Status,
        >;
        async fn validate_create_market(
            &self,
            request: tonic::Request<super::QueryValidateCreateMarketRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryValidateCreateMarketResponse>,
            tonic::Status,
        >;
        async fn validate_market(
            &self,
            request: tonic::Request<super::QueryValidateMarketRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryValidateMarketResponse>, tonic::Status>;
        async fn validate_manage_fees(
            &self,
            request: tonic::Request<super::QueryValidateManageFeesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryValidateManageFeesResponse>,
            tonic::Status,
        >;
        async fn get_payment(
            &self,
            request: tonic::Request<super::QueryGetPaymentRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetPaymentResponse>, tonic::Status>;
        async fn get_payments_with_source(
            &self,
            request: tonic::Request<super::QueryGetPaymentsWithSourceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetPaymentsWithSourceResponse>,
            tonic::Status,
        >;
        async fn get_payments_with_target(
            &self,
            request: tonic::Request<super::QueryGetPaymentsWithTargetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetPaymentsWithTargetResponse>,
            tonic::Status,
        >;
        async fn get_all_payments(
            &self,
            request: tonic::Request<super::QueryGetAllPaymentsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetAllPaymentsResponse>, tonic::Status>;
        async fn payment_fee_calc(
            &self,
            request: tonic::Request<super::QueryPaymentFeeCalcRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryPaymentFeeCalcResponse>, tonic::Status>;
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
                "/provenance.exchange.v1.Query/OrderFeeCalc" => {
                    #[allow(non_camel_case_types)]
                    struct OrderFeeCalcSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryOrderFeeCalcRequest> for OrderFeeCalcSvc<T> {
                        type Response = super::QueryOrderFeeCalcResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryOrderFeeCalcRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::order_fee_calc(&inner, request).await };
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
                        let method = OrderFeeCalcSvc(inner);
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
                "/provenance.exchange.v1.Query/GetOrder" => {
                    #[allow(non_camel_case_types)]
                    struct GetOrderSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGetOrderRequest> for GetOrderSvc<T> {
                        type Response = super::QueryGetOrderResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetOrderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Query>::get_order(&inner, request).await };
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
                        let method = GetOrderSvc(inner);
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
                "/provenance.exchange.v1.Query/GetOrderByExternalID" => {
                    #[allow(non_camel_case_types)]
                    struct GetOrderByExternalIDSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryGetOrderByExternalIdRequest>
                        for GetOrderByExternalIDSvc<T>
                    {
                        type Response = super::QueryGetOrderByExternalIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetOrderByExternalIdRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::get_order_by_external_id(&inner, request).await
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
                        let method = GetOrderByExternalIDSvc(inner);
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
                "/provenance.exchange.v1.Query/GetMarketOrders" => {
                    #[allow(non_camel_case_types)]
                    struct GetMarketOrdersSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGetMarketOrdersRequest>
                        for GetMarketOrdersSvc<T>
                    {
                        type Response = super::QueryGetMarketOrdersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetMarketOrdersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::get_market_orders(&inner, request).await
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
                        let method = GetMarketOrdersSvc(inner);
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
                "/provenance.exchange.v1.Query/GetOwnerOrders" => {
                    #[allow(non_camel_case_types)]
                    struct GetOwnerOrdersSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGetOwnerOrdersRequest>
                        for GetOwnerOrdersSvc<T>
                    {
                        type Response = super::QueryGetOwnerOrdersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetOwnerOrdersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::get_owner_orders(&inner, request).await
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
                        let method = GetOwnerOrdersSvc(inner);
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
                "/provenance.exchange.v1.Query/GetAssetOrders" => {
                    #[allow(non_camel_case_types)]
                    struct GetAssetOrdersSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGetAssetOrdersRequest>
                        for GetAssetOrdersSvc<T>
                    {
                        type Response = super::QueryGetAssetOrdersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetAssetOrdersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::get_asset_orders(&inner, request).await
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
                        let method = GetAssetOrdersSvc(inner);
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
                "/provenance.exchange.v1.Query/GetAllOrders" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllOrdersSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGetAllOrdersRequest> for GetAllOrdersSvc<T> {
                        type Response = super::QueryGetAllOrdersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetAllOrdersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::get_all_orders(&inner, request).await };
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
                        let method = GetAllOrdersSvc(inner);
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
                "/provenance.exchange.v1.Query/GetCommitment" => {
                    #[allow(non_camel_case_types)]
                    struct GetCommitmentSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGetCommitmentRequest>
                        for GetCommitmentSvc<T>
                    {
                        type Response = super::QueryGetCommitmentResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetCommitmentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::get_commitment(&inner, request).await };
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
                        let method = GetCommitmentSvc(inner);
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
                "/provenance.exchange.v1.Query/GetAccountCommitments" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountCommitmentsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryGetAccountCommitmentsRequest>
                        for GetAccountCommitmentsSvc<T>
                    {
                        type Response = super::QueryGetAccountCommitmentsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetAccountCommitmentsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::get_account_commitments(&inner, request).await
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
                        let method = GetAccountCommitmentsSvc(inner);
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
                "/provenance.exchange.v1.Query/GetMarketCommitments" => {
                    #[allow(non_camel_case_types)]
                    struct GetMarketCommitmentsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryGetMarketCommitmentsRequest>
                        for GetMarketCommitmentsSvc<T>
                    {
                        type Response = super::QueryGetMarketCommitmentsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetMarketCommitmentsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::get_market_commitments(&inner, request).await
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
                        let method = GetMarketCommitmentsSvc(inner);
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
                "/provenance.exchange.v1.Query/GetAllCommitments" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllCommitmentsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGetAllCommitmentsRequest>
                        for GetAllCommitmentsSvc<T>
                    {
                        type Response = super::QueryGetAllCommitmentsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetAllCommitmentsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::get_all_commitments(&inner, request).await
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
                        let method = GetAllCommitmentsSvc(inner);
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
                "/provenance.exchange.v1.Query/GetMarket" => {
                    #[allow(non_camel_case_types)]
                    struct GetMarketSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGetMarketRequest> for GetMarketSvc<T> {
                        type Response = super::QueryGetMarketResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetMarketRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::get_market(&inner, request).await };
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
                        let method = GetMarketSvc(inner);
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
                "/provenance.exchange.v1.Query/GetAllMarkets" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllMarketsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGetAllMarketsRequest>
                        for GetAllMarketsSvc<T>
                    {
                        type Response = super::QueryGetAllMarketsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetAllMarketsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::get_all_markets(&inner, request).await };
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
                        let method = GetAllMarketsSvc(inner);
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
                "/provenance.exchange.v1.Query/Params" => {
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
                "/provenance.exchange.v1.Query/CommitmentSettlementFeeCalc" => {
                    #[allow(non_camel_case_types)]
                    struct CommitmentSettlementFeeCalcSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryCommitmentSettlementFeeCalcRequest>
                        for CommitmentSettlementFeeCalcSvc<T>
                    {
                        type Response = super::QueryCommitmentSettlementFeeCalcResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryCommitmentSettlementFeeCalcRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::commitment_settlement_fee_calc(&inner, request).await
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
                        let method = CommitmentSettlementFeeCalcSvc(inner);
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
                "/provenance.exchange.v1.Query/ValidateCreateMarket" => {
                    #[allow(non_camel_case_types)]
                    struct ValidateCreateMarketSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryValidateCreateMarketRequest>
                        for ValidateCreateMarketSvc<T>
                    {
                        type Response = super::QueryValidateCreateMarketResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryValidateCreateMarketRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::validate_create_market(&inner, request).await
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
                        let method = ValidateCreateMarketSvc(inner);
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
                "/provenance.exchange.v1.Query/ValidateMarket" => {
                    #[allow(non_camel_case_types)]
                    struct ValidateMarketSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryValidateMarketRequest>
                        for ValidateMarketSvc<T>
                    {
                        type Response = super::QueryValidateMarketResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryValidateMarketRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::validate_market(&inner, request).await };
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
                        let method = ValidateMarketSvc(inner);
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
                "/provenance.exchange.v1.Query/ValidateManageFees" => {
                    #[allow(non_camel_case_types)]
                    struct ValidateManageFeesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryValidateManageFeesRequest>
                        for ValidateManageFeesSvc<T>
                    {
                        type Response = super::QueryValidateManageFeesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryValidateManageFeesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::validate_manage_fees(&inner, request).await
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
                        let method = ValidateManageFeesSvc(inner);
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
                "/provenance.exchange.v1.Query/GetPayment" => {
                    #[allow(non_camel_case_types)]
                    struct GetPaymentSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGetPaymentRequest> for GetPaymentSvc<T> {
                        type Response = super::QueryGetPaymentResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetPaymentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::get_payment(&inner, request).await };
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
                        let method = GetPaymentSvc(inner);
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
                "/provenance.exchange.v1.Query/GetPaymentsWithSource" => {
                    #[allow(non_camel_case_types)]
                    struct GetPaymentsWithSourceSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryGetPaymentsWithSourceRequest>
                        for GetPaymentsWithSourceSvc<T>
                    {
                        type Response = super::QueryGetPaymentsWithSourceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetPaymentsWithSourceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::get_payments_with_source(&inner, request).await
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
                        let method = GetPaymentsWithSourceSvc(inner);
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
                "/provenance.exchange.v1.Query/GetPaymentsWithTarget" => {
                    #[allow(non_camel_case_types)]
                    struct GetPaymentsWithTargetSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryGetPaymentsWithTargetRequest>
                        for GetPaymentsWithTargetSvc<T>
                    {
                        type Response = super::QueryGetPaymentsWithTargetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetPaymentsWithTargetRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::get_payments_with_target(&inner, request).await
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
                        let method = GetPaymentsWithTargetSvc(inner);
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
                "/provenance.exchange.v1.Query/GetAllPayments" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllPaymentsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGetAllPaymentsRequest>
                        for GetAllPaymentsSvc<T>
                    {
                        type Response = super::QueryGetAllPaymentsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetAllPaymentsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::get_all_payments(&inner, request).await
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
                        let method = GetAllPaymentsSvc(inner);
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
                "/provenance.exchange.v1.Query/PaymentFeeCalc" => {
                    #[allow(non_camel_case_types)]
                    struct PaymentFeeCalcSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryPaymentFeeCalcRequest>
                        for PaymentFeeCalcSvc<T>
                    {
                        type Response = super::QueryPaymentFeeCalcResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPaymentFeeCalcRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::payment_fee_calc(&inner, request).await
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
                        let method = PaymentFeeCalcSvc(inner);
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
        const NAME: &'static str = "provenance.exchange.v1.Query";
    }
}
