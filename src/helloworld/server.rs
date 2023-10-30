// tested with grpcurl
// grpcurl -plaintext -import-path ./proto/helloworld -proto helloworld.proto -d '{"name": "Tonic"}' '[::1]:50051' helloworld_pack.Greeter/SayHello

use tonic::{transport::Server, Request, Response, Status};

use hello_world::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};

pub mod hello_world {
    tonic::include_proto!("helloworld"); // package name
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self, request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr().unwrap());
        println!("->> {request:?}\n");

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("Server listening on http://{addr}\n");

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
