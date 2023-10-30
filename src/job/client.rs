use job::{job_runner_client::JobRunnerClient, Empty, JobRequest};

pub mod job {
    tonic::include_proto!("job"); // package name
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = JobRunnerClient::connect("http://127.0.0.1:50051").await?;

    // send_job method
    let request = tonic::Request::new(JobRequest {
        name: "Tonic".into(),
    });
    let response = client.send_job(request).await?;
    println!("{:?}\n", response);

    // list_jobs method
    let request = tonic::Request::new(Empty {});
    let response = client.list_jobs(request).await?;
    println!("{:?}\n", response);

    Ok(())
}
