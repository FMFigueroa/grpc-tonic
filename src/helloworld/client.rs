use hello_world::{greeter_client::GreeterClient, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld"); // package name
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://127.0.0.1:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("{:?}", response);

    Ok(())
}
