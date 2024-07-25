use tonic::{transport::Server, Request, Response, Status, IntoRequest};

use system_info::system_info_service_server::{SystemInfoService, SystemInfoServiceServer};
use system_info::{SysInfoRequest, SysInfoResponce};

use ::clap::{Parser};

use sysinfo::{System};

pub mod system_info {
    tonic::include_proto!("system_info");
}

#[derive(Debug, Default)]
pub struct SysInfo {}

//noinspection ALL
#[tonic::async_trait]
impl SystemInfoService for SysInfo {
    async fn sys_info(&self, request: Request<SysInfoRequest>) -> Result<Response<SysInfoResponce>, Status> {
        println!("Got a request: {:?}", request);
        let reply = SysInfoResponce {

            message: format!("{:?}", System::kernel_version()),
        };
        Ok(Response::new(reply))
    }
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "system_info - a simple sys_info microservice", long_about = None)]
struct ServerCli {
    #[arg(short = 's', long = "server", default_value = "127.0.0.1")]
    server: String,
    #[arg(short = 'p', long = "port", default_value = "50052")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cli = ServerCli::parse();
    let addr = format!("{}:{}", cli.server, cli.port).parse()?;
    let sysinfo = SysInfo::default();

    println!("Server listenin on {}", addr);

    Server::builder()
        .add_service(SystemInfoServiceServer::new(sysinfo))
        .serve(addr)
        .await?;

    Ok(())
}