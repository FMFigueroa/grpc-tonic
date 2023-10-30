use autometrics::{autometrics, prometheus_exporter};
use axum::{routing::get, Router};
use std::net::SocketAddr;

use tonic::{transport::Server, Request, Response, Status};

use job::{
    job_runner_server::{JobRunner, JobRunnerServer},
    Empty, Job, JobList, JobReply, JobRequest,
};

pub mod job {
    tonic::include_proto!("job");
}

#[derive(Debug, Default)]
pub struct MyJobRunner {}

#[tonic::async_trait]
impl JobRunner for MyJobRunner {
    #[autometrics]
    async fn send_job(&self, request: Request<JobRequest>) -> Result<Response<JobReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr().unwrap());
        println!("->> {request:?}\n");

        let reply = JobReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }

    #[autometrics]
    async fn list_jobs(&self, request: Request<Empty>) -> Result<Response<JobList>, Status> {
        println!("Got a request from {:?}", request.remote_addr().unwrap());
        println!("->> {request:?}\n");

        let reply = JobList {
            job: vec![Job {
                id: 1,
                name: "test".into(),
            }],
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the exporter to collect metrics
    prometheus_exporter::init();

    // gRPC server
    let grpc_addr = "127.0.0.1:50051".parse()?;
    println!("GrpcServer listening on http://{grpc_addr}\n");
    let grpc_service = MyJobRunner::default();

    tokio::spawn(async move {
        Server::builder()
            .add_service(JobRunnerServer::new(grpc_service))
            .serve(grpc_addr)
            .await
            .expect("gRPC server failed")
    });

    // Web server with Axum for Prometheus
    let web_addr: SocketAddr = "127.0.0.1:8080".parse()?;
    println!("WebServer listening on http://{web_addr}\n");

    let app = Router::new().route("/", get(handler)).route(
        "/metrics",
        get(|| async { prometheus_exporter::encode_http_response() }),
    );

    axum::Server::bind(&web_addr)
        .serve(app.into_make_service())
        .await
        .expect("Web server failed");

    Ok(())
}

async fn handler() -> &'static str {
    "Hello, World!"
}
