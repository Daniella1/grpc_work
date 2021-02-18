#![allow(non_snake_case)]

mod grpc_protobuf;
mod handshake_proto;
mod unifmu_fmi2_proto;
use std::{future::Future, path::PathBuf, str, sync::Arc};

use grpc_protobuf::{Fmi2CommandRPC, GRPC_Protobuf_Client};

use handshake_proto::{
    handshaker_server::Handshaker, handshaker_server::HandshakerServer, HandshakeInfo, Void,
};
use mpsc::{channel, sync_channel, Sender, SyncSender};
use std::sync::mpsc;
use subprocess::Popen;
use subprocess::PopenConfig;
use tokio::runtime::{Builder, Runtime};
use tonic::{server, transport::Server, Request, Response, Status};

struct Handshake {
    ip: String,
    port: String,
}

#[derive(Debug)]
pub struct HandshakeCreator {
    handshake_tx: SyncSender<HandshakeInfo>,
}

#[tonic::async_trait]
impl Handshaker for HandshakeCreator {
    async fn perform_handshake(
        &self,
        handshake_info: Request<HandshakeInfo>,
    ) -> Result<Response<Void>, Status> {
        self.handshake_tx.send(handshake_info.into_inner());

        Ok(Response::new(Void {}))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1) start handshake service
    // 2) launch process
    // 3) wait for process to perform handshake
    // 4) Finalize rpc connection, i.e. create struct containing endpoint of slave

    // not sure if this will work on its own, ideally this should check if a variable is set

    let (tx, rx) = sync_channel::<HandshakeInfo>(1);

    let handshake_creator = HandshakeCreator { handshake_tx: tx };

    let addr = "127.0.0.1:50051";
    let rt = Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .expect("failed to obtain a new RunTime object");
    let server_future = Server::builder()
        .add_service(HandshakerServer::new(handshake_creator))
        .serve(addr.parse().unwrap());
    // .serve(addr.parse().unwrap());

    let command = [
        "python",
        "../grpc_python/src/backend_grpc.py",
        "--handshake-endpoint",
        addr,
    ];

    let resources_dir = PathBuf::from(".");
    let _popen = Popen::create(
        &command,
        PopenConfig {
            cwd: Some(resources_dir.as_os_str().to_owned()),
            ..Default::default()
        },
    ).expect(&format!("Unable to start the process using the specified command '{:?}'. Ensure that you can invoke the command directly from a shell", command));

    let _handle = rt.spawn(server_future);

    let a = rx.recv().unwrap();
    println!("Handhsake info: {:?}", a);

    let connection_str = format!("http://{}:{}", a.ip_address, a.port);
    println!("HELLOOOO");
    let mut client = GRPC_Protobuf_Client::connect(connection_str).unwrap();

    // Real
    let res_set_real = client.fmi2SetReal(&[0, 1], &[0.5, 0.6]);
    println!("Setreal RESPONSE={:?}", res_set_real);

    client.fmi2DoStep(0.0, 0.0, false);

    let res_get_real = client.fmi2GetReal(&[0, 1, 2]);
    println!("Getreal RESPONSE={:?}", res_get_real);

    // Integer
    let res_set_int = client.fmi2SetInteger(&[3, 4], &[25, 35]);
    println!("Set int RESPONSE={:?}", res_set_int);

    client.fmi2DoStep(0.0, 0.0, false);

    let res_get_int = client.fmi2GetInteger(&[3, 4, 5]);
    println!("Get int RESPONSE={:?}", res_get_int);

    // Boolean
    let res_set_bool = client.fmi2SetBoolean(&[6, 7], &[true, true]);
    println!("Set bool RESPONSE={:?}", res_set_bool);

    client.fmi2DoStep(0.0, 0.0, false);

    let res_get_bool = client.fmi2GetBoolean(&[6, 7, 8]);
    println!("Get bool RESPONSE={:?}", res_get_bool);

    // String
    let res_set_string = client.fmi2SetString(&[9, 10], &["Hello ", "World!"]);
    println!("Set string RESPONSE={:?}", res_set_string);

    client.fmi2DoStep(0.0, 0.0, false);

    let res_get_string = client.fmi2GetString(&[9, 10, 11]);
    println!("Get string RESPONSE={:?}", res_get_string);

    Ok(())
}
