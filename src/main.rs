//noinspection ALL,RsUnresolvedPath
use system_info::system_info_service_client::SystemInfoServiceClient;
use system_info::SysInfoRequest;
use ::clap::{Parser};

pub mod system_info {
    tonic::include_proto!("system_info");
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "sistem_info - a simple CLI for send messages to a server", long_about = None)]
struct ClientCli {
    #[arg(short = 's', long = "server", default_value = "127.0.0.1")]
    server: String,
    #[arg(short = 'p', long = "port", default_value = "50052")]
    port: u16,

    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = ClientCli::parse();
    let mut client = SystemInfoServiceClient::connect(format!("https://{}:{}", cli.server, cli.port)).await?;
    let request = tonic::Request::new(SysInfoRequest { message: cli.message });
    let response = client.sys_info(request).await?;

    println!("RESPONSE = {:?}", response.into_inner().message);

    Ok(())
}