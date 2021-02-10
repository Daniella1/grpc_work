#![allow(non_snake_case)]
use crate::unifmu_fmi2_proto;
use num_enum::TryFromPrimitive;
use std::convert::TryInto;
use tokio::runtime::{Builder, Runtime};
use unifmu_fmi2_proto::send_command_client::SendCommandClient;

// Must have compiled the proto file before using this

// To make a blocking grpc client, we follow: https://github.com/hyperium/tonic/blob/master/examples/src/blocking/client.rs
type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T, E = StdError> = ::std::result::Result<T, E>;
pub struct GRPC_Protobuf_Client {
    client: SendCommandClient<tonic::transport::Channel>,
    rt: Runtime,
}

impl GRPC_Protobuf_Client {
    pub fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
    where
        D: std::convert::TryInto<tonic::transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        let rt = Builder::new_multi_thread().enable_all().build().unwrap();
        let client = rt.block_on(SendCommandClient::connect(dst))?;

        Ok(Self { rt, client })
    }
}

#[repr(i32)]
#[derive(serde::Deserialize, TryFromPrimitive, Debug, PartialEq)]
pub enum Fmi2Status {
    Fmi2OK = 0,
    Fmi2Warning = 1,
    Fmi2Discard = 2,
    Fmi2Error = 3,
    Fmi2Fatal = 4,
    Fmi2Pending = 5,
}

trait CoerceToEmpty {
    fn to_optional(self) -> Option<Self>
    where
        Self: Sized;
}

impl<T> CoerceToEmpty for prost::alloc::vec::Vec<T> {
    fn to_optional(self) -> Option<Vec<T>> {
        match self.is_empty() {
            true => None,
            false => Some(self),
        }
    }
}

/// Trait implemented by objects that provide a way to communicate with FMUs using 'Remote Procedure Call' (RPC)
pub trait Fmi2CommandRPC {
    fn fmi2DoStep(&mut self, current_time: f64, step_size: f64, no_step_prior: bool) -> Fmi2Status;
    fn fmi2CancelStep(&mut self) -> Fmi2Status;
    fn fmi2SetDebugLogging(&mut self, categories: &[&str], logging_on: bool) -> Fmi2Status;
    fn fmi2SetupExperiment(
        &mut self,
        start_time: f64,
        stop_time: Option<f64>,
        tolerance: Option<f64>,
    ) -> Fmi2Status;
    fn fmi2EnterInitializationMode(&mut self) -> Fmi2Status;
    fn fmi2ExitInitializationMode(&mut self) -> Fmi2Status;
    fn fmi2Terminate(&mut self) -> Fmi2Status;
    fn fmi2Reset(&mut self) -> Fmi2Status;

    fn fmi2SetReal(&mut self, references: &[u32], values: &[f64]) -> Fmi2Status;
    fn fmi2SetInteger(&mut self, references: &[u32], values: &[i32]) -> Fmi2Status;
    fn fmi2SetBoolean(&mut self, references: &[u32], values: &[bool]) -> Fmi2Status;
    fn fmi2SetString(&mut self, references: &[u32], values: &[&str]) -> Fmi2Status;

    // TODO add status to return
    fn fmi2GetReal(&mut self, references: &[u32]) -> (Fmi2Status, Option<Vec<f64>>);
    fn fmi2GetInteger(&mut self, references: &[u32]) -> (Fmi2Status, Option<Vec<i32>>);
    fn fmi2GetBoolean(&mut self, references: &[u32]) -> (Fmi2Status, Option<Vec<bool>>);
    fn fmi2GetString(&mut self, references: &[u32]) -> (Fmi2Status, Option<Vec<String>>);

    fn serialize(&mut self) -> (Fmi2Status, Option<Vec<u8>>);
    fn deserialize(&mut self, bytes: &[u8]) -> Fmi2Status;

    fn fmi2FreeInstance(&mut self);
}

impl Fmi2CommandRPC for GRPC_Protobuf_Client {
    fn fmi2DoStep(&mut self, current_time: f64, step_size: f64, no_step_prior: bool) -> Fmi2Status {
        let args = tonic::Request::new(unifmu_fmi2_proto::DoStep {
            current_time: current_time,
            step_size: step_size,
            no_step_prior: no_step_prior
        });

        match self.rt.block_on(self.client.fmi2_do_step(args)) {
            Ok(s) => s.into_inner().status.try_into().unwrap(),
            Err(_) => Fmi2Status::Fmi2Error,
        }
    }

    fn fmi2CancelStep(&mut self) -> Fmi2Status {
        todo!()
    }

    fn fmi2SetDebugLogging(&mut self, categories: &[&str], logging_on: bool) -> Fmi2Status {
        todo!()
    }

    fn fmi2SetupExperiment(
        &mut self,
        start_time: f64,
        stop_time: Option<f64>,
        tolerance: Option<f64>,
    ) -> Fmi2Status {
        todo!()
    }

    fn fmi2EnterInitializationMode(&mut self) -> Fmi2Status {
        todo!()
    }

    fn fmi2ExitInitializationMode(&mut self) -> Fmi2Status {
        todo!()
    }

    fn fmi2Terminate(&mut self) -> Fmi2Status {
        todo!()
    }

    fn fmi2Reset(&mut self) -> Fmi2Status {
        todo!()
    }

    fn fmi2SetReal(&mut self, references: &[u32], values: &[f64]) -> Fmi2Status {
        let args = tonic::Request::new(unifmu_fmi2_proto::SetReal {
            references: Vec::from(references),
            values: Vec::from(values),
        });

        match self.rt.block_on(self.client.fmi2_set_real(args)) {
            Ok(s) => s.into_inner().status.try_into().unwrap(),
            Err(_) => Fmi2Status::Fmi2Error,
        }
    }

    fn fmi2SetInteger(&mut self, references: &[u32], values: &[i32]) -> Fmi2Status {
        let args = tonic::Request::new(unifmu_fmi2_proto::SetInteger {
            references: Vec::from(references),
            values: Vec::from(values),
        });

        match self.rt.block_on(self.client.fmi2_set_integer(args)) {
            Ok(s) => s.into_inner().status.try_into().unwrap(),
            Err(_) => Fmi2Status::Fmi2Error,
        }
    }

    fn fmi2SetBoolean(&mut self, references: &[u32], values: &[bool]) -> Fmi2Status {
        let args = tonic::Request::new(unifmu_fmi2_proto::SetBoolean {
            references: Vec::from(references),
            values: Vec::from(values),
        });

        match self.rt.block_on(self.client.fmi2_set_boolean(args)) {
            Ok(s) => s.into_inner().status.try_into().unwrap(),
            Err(_) => Fmi2Status::Fmi2Error,
        }
    }

    fn fmi2SetString(&mut self, references: &[u32], values: &[&str]) -> Fmi2Status {
        let s_values = values.iter().map(|s| s.to_string()).collect();
        let args = tonic::Request::new(unifmu_fmi2_proto::SetString {
            references: Vec::from(references),
            values: s_values,
        });

        match self.rt.block_on(self.client.fmi2_set_string(args)) {
            Ok(s) => s.into_inner().status.try_into().unwrap(),
            Err(_) => Fmi2Status::Fmi2Error,
        }
    }

    fn fmi2GetReal(&mut self, references: &[u32]) -> (Fmi2Status, Option<Vec<f64>>) {
        let args = tonic::Request::new(unifmu_fmi2_proto::GetXxx {
            references: Vec::from(references),
        });

        let res = self.rt.block_on(self.client.fmi2_get_real(args));

        match res {
            Ok(s) => {
                let a = s.into_inner();
                let status = a.status.try_into().unwrap();
                let values = a.values;
                (status, values.to_optional())
            }
            Err(_) => (Fmi2Status::Fmi2Error, None),
        }
    }

    fn fmi2GetInteger(&mut self, references: &[u32]) -> (Fmi2Status, Option<Vec<i32>>) {
        let args = tonic::Request::new(unifmu_fmi2_proto::GetXxx {
            references: Vec::from(references),
        });

        let res = self.rt.block_on(self.client.fmi2_get_integer(args));

        match res {
            Ok(s) => {
                let a = s.into_inner();
                let status = a.status.try_into().unwrap();
                let values = a.values;
                (status, values.to_optional())
            }
            Err(_) => (Fmi2Status::Fmi2Error, None),
        }
    }

    fn fmi2GetBoolean(&mut self, references: &[u32]) -> (Fmi2Status, Option<Vec<bool>>) {
        let args = tonic::Request::new(unifmu_fmi2_proto::GetXxx {
            references: Vec::from(references),
        });

        let res = self.rt.block_on(self.client.fmi2_get_boolean(args));

        match res {
            Ok(s) => {
                let a = s.into_inner();
                let status = a.status.try_into().unwrap();
                let values = a.values;
                (status, values.to_optional())
            }
            Err(_) => (Fmi2Status::Fmi2Error, None),
        }
    }

    fn fmi2GetString(&mut self, references: &[u32]) -> (Fmi2Status, Option<Vec<String>>) {
        let args = tonic::Request::new(unifmu_fmi2_proto::GetXxx {
            references: Vec::from(references),
        });

        let res = self.rt.block_on(self.client.fmi2_get_string(args));

        match res {
            Ok(s) => {
                let a = s.into_inner();
                let status = a.status.try_into().unwrap();
                let values = a.values;
                (status, values.to_optional())
            }
            Err(_) => (Fmi2Status::Fmi2Error, None),
        }
    }

    fn serialize(&mut self) -> (Fmi2Status, Option<Vec<u8>>) {
        todo!()
    }

    fn deserialize(&mut self, bytes: &[u8]) -> Fmi2Status {
        todo!()
    }

    fn fmi2FreeInstance(&mut self) {
        todo!()
    }
}
