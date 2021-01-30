use unifmu_fmi2_proto::send_command_client::SendCommandClient;
use unifmu_fmi2_proto::SetReal;

// use unifmu_fmi2_proto::DoStep;
// use unifmu_fmi2_proto::GetRealReturn;
// use unifmu_fmi2_proto::GetXxx;

pub mod unifmu_fmi2_proto {
    tonic::include_proto!("unifmu_fmi2_proto");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SendCommandClient::connect("http://[::1]:50051").await?;

    let set_real = tonic::Request::new(SetReal {
        references: vec![0, 1],
        values: vec![0.5, 0.6],
    });

    let response = client.fmi2_set_real(set_real).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
