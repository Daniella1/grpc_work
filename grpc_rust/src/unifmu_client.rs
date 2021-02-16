#![allow(non_snake_case)]

mod grpc_protobuf;
mod unifmu_fmi2_proto;
use grpc_protobuf::{Fmi2CommandRPC, GRPC_Protobuf_Client};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1) start handshake service
    // 2) launch process
    // 3) wait for process to perform handshake
    // 4) Finalize rpc connection, i.e. create struct containing endpoint of slave

    let mut client = GRPC_Protobuf_Client::connect("http://[::1]:50051").unwrap();

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
