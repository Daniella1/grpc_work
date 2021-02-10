#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetReal {
    /// array of uint32 --> in rust is array of c_uint
    #[prost(uint32, repeated, tag = "1")]
    pub references: ::prost::alloc::vec::Vec<u32>,
    #[prost(double, repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetInteger {
    #[prost(uint32, repeated, tag = "1")]
    pub references: ::prost::alloc::vec::Vec<u32>,
    #[prost(int32, repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetBoolean {
    #[prost(uint32, repeated, tag = "1")]
    pub references: ::prost::alloc::vec::Vec<u32>,
    #[prost(bool, repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetString {
    #[prost(uint32, repeated, tag = "1")]
    pub references: ::prost::alloc::vec::Vec<u32>,
    #[prost(string, repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetXxx {
    #[prost(uint32, repeated, tag = "1")]
    pub references: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoStep {
    #[prost(double, tag = "1")]
    pub current_time: f64,
    #[prost(double, tag = "2")]
    pub step_size: f64,
    #[prost(bool, tag = "3")]
    pub no_step_prior: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnterInitializationMode {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExitInitializationMode {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FreeInstance {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Terminate {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reset {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetupExperiment {
    #[prost(double, tag = "1")]
    pub start_time: f64,
    /// currently optional scalars are not supported by python
    /// until supported we will use extra bool fields
    /// stop_time: double=null;
    /// tolerance: double=null;
    #[prost(double, tag = "2")]
    pub stop_time: f64,
    #[prost(double, tag = "3")]
    pub tolerance: f64,
    #[prost(bool, tag = "4")]
    pub has_stop_time: bool,
    #[prost(bool, tag = "5")]
    pub has_tolerance: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerializeMessage {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeserializeMessage {
    #[prost(bytes = "vec", tag = "1")]
    pub state: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDirectionalDerivatives {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetInputDerivatives {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOutputDerivatives {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelStep {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetXxxStatus {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDebugLogging {
    #[prost(string, tag = "1")]
    pub categories: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub logging_on: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fmi2Command {
    #[prost(
        oneof = "fmi2_command::Args",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23"
    )]
    pub args: ::core::option::Option<fmi2_command::Args>,
}
/// Nested message and enum types in `Fmi2Command`.
pub mod fmi2_command {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Args {
        /// Field numbers between 1-15 only use one byte, therefore the functions that are called the most will be between 1-15
        ///
        /// Co-sim
        #[prost(int32, tag = "1")]
        DoStep(i32),
        #[prost(int32, tag = "2")]
        SetReal(i32),
        #[prost(int32, tag = "3")]
        SetInteger(i32),
        #[prost(int32, tag = "4")]
        SetBoolean(i32),
        #[prost(int32, tag = "5")]
        SetString(i32),
        #[prost(int32, tag = "6")]
        GetReal(i32),
        #[prost(int32, tag = "7")]
        GetInteger(i32),
        #[prost(int32, tag = "8")]
        GetBoolean(i32),
        #[prost(int32, tag = "9")]
        GetString(i32),
        #[prost(int32, tag = "10")]
        SetDebugLogging(i32),
        #[prost(int32, tag = "11")]
        SetupExperiment(i32),
        #[prost(int32, tag = "12")]
        FreeInstance(i32),
        #[prost(int32, tag = "13")]
        EnterInitializationMode(i32),
        #[prost(int32, tag = "14")]
        ExitInitializationMode(i32),
        #[prost(int32, tag = "15")]
        Terminate(i32),
        #[prost(int32, tag = "16")]
        Reset(i32),
        #[prost(int32, tag = "17")]
        Serialize(i32),
        #[prost(int32, tag = "18")]
        Deserialize(i32),
        /// Co-sim
        #[prost(int32, tag = "19")]
        GetDirectionalDerivatives(i32),
        #[prost(int32, tag = "20")]
        SetInputDerivatives(i32),
        #[prost(int32, tag = "21")]
        GetOutputDerivatives(i32),
        #[prost(int32, tag = "22")]
        CancelStep(i32),
        #[prost(int32, tag = "23")]
        GetXxxStatus(i32),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusReturn {
    #[prost(enumeration = "FmiStatus", tag = "1")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRealReturn {
    #[prost(enumeration = "FmiStatus", tag = "1")]
    pub status: i32,
    #[prost(double, repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIntegerReturn {
    #[prost(enumeration = "FmiStatus", tag = "1")]
    pub status: i32,
    #[prost(int32, repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBooleanReturn {
    #[prost(enumeration = "FmiStatus", tag = "1")]
    pub status: i32,
    #[prost(bool, repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStringReturn {
    #[prost(enumeration = "FmiStatus", tag = "1")]
    pub status: i32,
    #[prost(string, repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerializeReturn {
    #[prost(bytes = "vec", tag = "1")]
    pub state: ::prost::alloc::vec::Vec<u8>,
}
///  cannot define void return message, see https://groups.google.com/g/protobuf/c/4SCxInAktSc?pli=1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Void {}
// return values
// i.e. messages sent from fmu to wrapper

// several FMI2 operations simply return a status
// this is used as a common return type

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FmiStatus {
    Ok = 0,
    Warning = 1,
    Discard = 2,
    Error = 3,
    Fatal = 4,
    Pending = 5,
}
#[doc = r" Generated client implementations."]
pub mod send_command_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct SendCommandClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SendCommandClient<tonic::transport::Channel> {
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
    impl<T> SendCommandClient<T>
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
        #[doc = " Set and Get variable value methods"]
        pub async fn fmi2_set_real(
            &mut self,
            request: impl tonic::IntoRequest<super::SetReal>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/unifmu_fmi2_proto.SendCommand/Fmi2SetReal");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn fmi2_get_real(
            &mut self,
            request: impl tonic::IntoRequest<super::GetXxx>,
        ) -> Result<tonic::Response<super::GetRealReturn>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/unifmu_fmi2_proto.SendCommand/Fmi2GetReal");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn fmi2_set_integer(
            &mut self,
            request: impl tonic::IntoRequest<super::SetInteger>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/unifmu_fmi2_proto.SendCommand/Fmi2SetInteger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn fmi2_get_integer(
            &mut self,
            request: impl tonic::IntoRequest<super::GetXxx>,
        ) -> Result<tonic::Response<super::GetIntegerReturn>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/unifmu_fmi2_proto.SendCommand/Fmi2GetInteger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn fmi2_set_boolean(
            &mut self,
            request: impl tonic::IntoRequest<super::SetBoolean>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/unifmu_fmi2_proto.SendCommand/Fmi2SetBoolean",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn fmi2_get_boolean(
            &mut self,
            request: impl tonic::IntoRequest<super::GetXxx>,
        ) -> Result<tonic::Response<super::GetBooleanReturn>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/unifmu_fmi2_proto.SendCommand/Fmi2GetBoolean",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn fmi2_set_string(
            &mut self,
            request: impl tonic::IntoRequest<super::SetString>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/unifmu_fmi2_proto.SendCommand/Fmi2SetString",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn fmi2_get_string(
            &mut self,
            request: impl tonic::IntoRequest<super::GetXxx>,
        ) -> Result<tonic::Response<super::GetStringReturn>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/unifmu_fmi2_proto.SendCommand/Fmi2GetString",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Initialization, termination and resetting fmus"]
        pub async fn fmi2_enter_initialization_mode(
            &mut self,
            request: impl tonic::IntoRequest<super::EnterInitializationMode>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/unifmu_fmi2_proto.SendCommand/Fmi2EnterInitializationMode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn fmi2_exit_initialization_mode(
            &mut self,
            request: impl tonic::IntoRequest<super::ExitInitializationMode>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/unifmu_fmi2_proto.SendCommand/Fmi2ExitInitializationMode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn fmi2_terminate(
            &mut self,
            request: impl tonic::IntoRequest<super::Terminate>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/unifmu_fmi2_proto.SendCommand/Fmi2Terminate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn fmi2_reset(
            &mut self,
            request: impl tonic::IntoRequest<super::Reset>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/unifmu_fmi2_proto.SendCommand/Fmi2Reset");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creation, destruction and logging of fmu instances"]
        pub async fn fmi2_free_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::FreeInstance>,
        ) -> Result<tonic::Response<super::Void>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/unifmu_fmi2_proto.SendCommand/Fmi2FreeInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn fmi2_set_debug_logging(
            &mut self,
            request: impl tonic::IntoRequest<super::SetDebugLogging>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/unifmu_fmi2_proto.SendCommand/Fmi2SetDebugLogging",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Setting and Getting complete fmu state"]
        pub async fn serialize(
            &mut self,
            request: impl tonic::IntoRequest<super::SerializeMessage>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/unifmu_fmi2_proto.SendCommand/Serialize");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn deserialize(
            &mut self,
            request: impl tonic::IntoRequest<super::DeserializeMessage>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/unifmu_fmi2_proto.SendCommand/Deserialize");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 4.2.2 Computation"]
        pub async fn fmi2_do_step(
            &mut self,
            request: impl tonic::IntoRequest<super::DoStep>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/unifmu_fmi2_proto.SendCommand/Fmi2DoStep");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn fmi2_cancel_step(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelStep>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/unifmu_fmi2_proto.SendCommand/Fmi2CancelStep",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for SendCommandClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SendCommandClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SendCommandClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod send_command_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with SendCommandServer."]
    #[async_trait]
    pub trait SendCommand: Send + Sync + 'static {
        #[doc = " Set and Get variable value methods"]
        async fn fmi2_set_real(
            &self,
            request: tonic::Request<super::SetReal>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status>;
        async fn fmi2_get_real(
            &self,
            request: tonic::Request<super::GetXxx>,
        ) -> Result<tonic::Response<super::GetRealReturn>, tonic::Status>;
        async fn fmi2_set_integer(
            &self,
            request: tonic::Request<super::SetInteger>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status>;
        async fn fmi2_get_integer(
            &self,
            request: tonic::Request<super::GetXxx>,
        ) -> Result<tonic::Response<super::GetIntegerReturn>, tonic::Status>;
        async fn fmi2_set_boolean(
            &self,
            request: tonic::Request<super::SetBoolean>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status>;
        async fn fmi2_get_boolean(
            &self,
            request: tonic::Request<super::GetXxx>,
        ) -> Result<tonic::Response<super::GetBooleanReturn>, tonic::Status>;
        async fn fmi2_set_string(
            &self,
            request: tonic::Request<super::SetString>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status>;
        async fn fmi2_get_string(
            &self,
            request: tonic::Request<super::GetXxx>,
        ) -> Result<tonic::Response<super::GetStringReturn>, tonic::Status>;
        #[doc = " Initialization, termination and resetting fmus"]
        async fn fmi2_enter_initialization_mode(
            &self,
            request: tonic::Request<super::EnterInitializationMode>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status>;
        async fn fmi2_exit_initialization_mode(
            &self,
            request: tonic::Request<super::ExitInitializationMode>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status>;
        async fn fmi2_terminate(
            &self,
            request: tonic::Request<super::Terminate>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status>;
        async fn fmi2_reset(
            &self,
            request: tonic::Request<super::Reset>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status>;
        #[doc = " Creation, destruction and logging of fmu instances"]
        async fn fmi2_free_instance(
            &self,
            request: tonic::Request<super::FreeInstance>,
        ) -> Result<tonic::Response<super::Void>, tonic::Status>;
        async fn fmi2_set_debug_logging(
            &self,
            request: tonic::Request<super::SetDebugLogging>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status>;
        #[doc = " Setting and Getting complete fmu state"]
        async fn serialize(
            &self,
            request: tonic::Request<super::SerializeMessage>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status>;
        async fn deserialize(
            &self,
            request: tonic::Request<super::DeserializeMessage>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status>;
        #[doc = " 4.2.2 Computation"]
        async fn fmi2_do_step(
            &self,
            request: tonic::Request<super::DoStep>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status>;
        async fn fmi2_cancel_step(
            &self,
            request: tonic::Request<super::CancelStep>,
        ) -> Result<tonic::Response<super::StatusReturn>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SendCommandServer<T: SendCommand> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: SendCommand> SendCommandServer<T> {
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
    impl<T, B> Service<http::Request<B>> for SendCommandServer<T>
    where
        T: SendCommand,
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
                "/unifmu_fmi2_proto.SendCommand/Fmi2SetReal" => {
                    #[allow(non_camel_case_types)]
                    struct Fmi2SetRealSvc<T: SendCommand>(pub Arc<T>);
                    impl<T: SendCommand> tonic::server::UnaryService<super::SetReal> for Fmi2SetRealSvc<T> {
                        type Response = super::StatusReturn;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetReal>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).fmi2_set_real(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = Fmi2SetRealSvc(inner);
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
                "/unifmu_fmi2_proto.SendCommand/Fmi2GetReal" => {
                    #[allow(non_camel_case_types)]
                    struct Fmi2GetRealSvc<T: SendCommand>(pub Arc<T>);
                    impl<T: SendCommand> tonic::server::UnaryService<super::GetXxx> for Fmi2GetRealSvc<T> {
                        type Response = super::GetRealReturn;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::GetXxx>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).fmi2_get_real(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = Fmi2GetRealSvc(inner);
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
                "/unifmu_fmi2_proto.SendCommand/Fmi2SetInteger" => {
                    #[allow(non_camel_case_types)]
                    struct Fmi2SetIntegerSvc<T: SendCommand>(pub Arc<T>);
                    impl<T: SendCommand> tonic::server::UnaryService<super::SetInteger> for Fmi2SetIntegerSvc<T> {
                        type Response = super::StatusReturn;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetInteger>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).fmi2_set_integer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = Fmi2SetIntegerSvc(inner);
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
                "/unifmu_fmi2_proto.SendCommand/Fmi2GetInteger" => {
                    #[allow(non_camel_case_types)]
                    struct Fmi2GetIntegerSvc<T: SendCommand>(pub Arc<T>);
                    impl<T: SendCommand> tonic::server::UnaryService<super::GetXxx> for Fmi2GetIntegerSvc<T> {
                        type Response = super::GetIntegerReturn;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::GetXxx>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).fmi2_get_integer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = Fmi2GetIntegerSvc(inner);
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
                "/unifmu_fmi2_proto.SendCommand/Fmi2SetBoolean" => {
                    #[allow(non_camel_case_types)]
                    struct Fmi2SetBooleanSvc<T: SendCommand>(pub Arc<T>);
                    impl<T: SendCommand> tonic::server::UnaryService<super::SetBoolean> for Fmi2SetBooleanSvc<T> {
                        type Response = super::StatusReturn;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetBoolean>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).fmi2_set_boolean(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = Fmi2SetBooleanSvc(inner);
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
                "/unifmu_fmi2_proto.SendCommand/Fmi2GetBoolean" => {
                    #[allow(non_camel_case_types)]
                    struct Fmi2GetBooleanSvc<T: SendCommand>(pub Arc<T>);
                    impl<T: SendCommand> tonic::server::UnaryService<super::GetXxx> for Fmi2GetBooleanSvc<T> {
                        type Response = super::GetBooleanReturn;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::GetXxx>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).fmi2_get_boolean(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = Fmi2GetBooleanSvc(inner);
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
                "/unifmu_fmi2_proto.SendCommand/Fmi2SetString" => {
                    #[allow(non_camel_case_types)]
                    struct Fmi2SetStringSvc<T: SendCommand>(pub Arc<T>);
                    impl<T: SendCommand> tonic::server::UnaryService<super::SetString> for Fmi2SetStringSvc<T> {
                        type Response = super::StatusReturn;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetString>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).fmi2_set_string(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = Fmi2SetStringSvc(inner);
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
                "/unifmu_fmi2_proto.SendCommand/Fmi2GetString" => {
                    #[allow(non_camel_case_types)]
                    struct Fmi2GetStringSvc<T: SendCommand>(pub Arc<T>);
                    impl<T: SendCommand> tonic::server::UnaryService<super::GetXxx> for Fmi2GetStringSvc<T> {
                        type Response = super::GetStringReturn;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::GetXxx>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).fmi2_get_string(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = Fmi2GetStringSvc(inner);
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
                "/unifmu_fmi2_proto.SendCommand/Fmi2EnterInitializationMode" => {
                    #[allow(non_camel_case_types)]
                    struct Fmi2EnterInitializationModeSvc<T: SendCommand>(pub Arc<T>);
                    impl<T: SendCommand> tonic::server::UnaryService<super::EnterInitializationMode>
                        for Fmi2EnterInitializationModeSvc<T>
                    {
                        type Response = super::StatusReturn;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EnterInitializationMode>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).fmi2_enter_initialization_mode(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = Fmi2EnterInitializationModeSvc(inner);
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
                "/unifmu_fmi2_proto.SendCommand/Fmi2ExitInitializationMode" => {
                    #[allow(non_camel_case_types)]
                    struct Fmi2ExitInitializationModeSvc<T: SendCommand>(pub Arc<T>);
                    impl<T: SendCommand> tonic::server::UnaryService<super::ExitInitializationMode>
                        for Fmi2ExitInitializationModeSvc<T>
                    {
                        type Response = super::StatusReturn;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExitInitializationMode>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).fmi2_exit_initialization_mode(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = Fmi2ExitInitializationModeSvc(inner);
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
                "/unifmu_fmi2_proto.SendCommand/Fmi2Terminate" => {
                    #[allow(non_camel_case_types)]
                    struct Fmi2TerminateSvc<T: SendCommand>(pub Arc<T>);
                    impl<T: SendCommand> tonic::server::UnaryService<super::Terminate> for Fmi2TerminateSvc<T> {
                        type Response = super::StatusReturn;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Terminate>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).fmi2_terminate(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = Fmi2TerminateSvc(inner);
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
                "/unifmu_fmi2_proto.SendCommand/Fmi2Reset" => {
                    #[allow(non_camel_case_types)]
                    struct Fmi2ResetSvc<T: SendCommand>(pub Arc<T>);
                    impl<T: SendCommand> tonic::server::UnaryService<super::Reset> for Fmi2ResetSvc<T> {
                        type Response = super::StatusReturn;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Reset>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).fmi2_reset(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = Fmi2ResetSvc(inner);
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
                "/unifmu_fmi2_proto.SendCommand/Fmi2FreeInstance" => {
                    #[allow(non_camel_case_types)]
                    struct Fmi2FreeInstanceSvc<T: SendCommand>(pub Arc<T>);
                    impl<T: SendCommand> tonic::server::UnaryService<super::FreeInstance> for Fmi2FreeInstanceSvc<T> {
                        type Response = super::Void;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FreeInstance>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).fmi2_free_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = Fmi2FreeInstanceSvc(inner);
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
                "/unifmu_fmi2_proto.SendCommand/Fmi2SetDebugLogging" => {
                    #[allow(non_camel_case_types)]
                    struct Fmi2SetDebugLoggingSvc<T: SendCommand>(pub Arc<T>);
                    impl<T: SendCommand> tonic::server::UnaryService<super::SetDebugLogging>
                        for Fmi2SetDebugLoggingSvc<T>
                    {
                        type Response = super::StatusReturn;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetDebugLogging>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).fmi2_set_debug_logging(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = Fmi2SetDebugLoggingSvc(inner);
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
                "/unifmu_fmi2_proto.SendCommand/Serialize" => {
                    #[allow(non_camel_case_types)]
                    struct SerializeSvc<T: SendCommand>(pub Arc<T>);
                    impl<T: SendCommand> tonic::server::UnaryService<super::SerializeMessage> for SerializeSvc<T> {
                        type Response = super::StatusReturn;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SerializeMessage>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).serialize(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SerializeSvc(inner);
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
                "/unifmu_fmi2_proto.SendCommand/Deserialize" => {
                    #[allow(non_camel_case_types)]
                    struct DeserializeSvc<T: SendCommand>(pub Arc<T>);
                    impl<T: SendCommand> tonic::server::UnaryService<super::DeserializeMessage> for DeserializeSvc<T> {
                        type Response = super::StatusReturn;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeserializeMessage>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).deserialize(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeserializeSvc(inner);
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
                "/unifmu_fmi2_proto.SendCommand/Fmi2DoStep" => {
                    #[allow(non_camel_case_types)]
                    struct Fmi2DoStepSvc<T: SendCommand>(pub Arc<T>);
                    impl<T: SendCommand> tonic::server::UnaryService<super::DoStep> for Fmi2DoStepSvc<T> {
                        type Response = super::StatusReturn;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::DoStep>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).fmi2_do_step(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = Fmi2DoStepSvc(inner);
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
                "/unifmu_fmi2_proto.SendCommand/Fmi2CancelStep" => {
                    #[allow(non_camel_case_types)]
                    struct Fmi2CancelStepSvc<T: SendCommand>(pub Arc<T>);
                    impl<T: SendCommand> tonic::server::UnaryService<super::CancelStep> for Fmi2CancelStepSvc<T> {
                        type Response = super::StatusReturn;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelStep>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).fmi2_cancel_step(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = Fmi2CancelStepSvc(inner);
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
    impl<T: SendCommand> Clone for SendCommandServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: SendCommand> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SendCommand> tonic::transport::NamedService for SendCommandServer<T> {
        const NAME: &'static str = "unifmu_fmi2_proto.SendCommand";
    }
}
