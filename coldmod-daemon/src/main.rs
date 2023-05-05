use coldmod_daemon::proto::tracing_collector_server::{TracingCollector, TracingCollectorServer};
use coldmod_daemon::proto::Trace;
use tonic::{transport::Server, Request, Response, Status, Streaming};

#[derive(Debug, Default)]
pub struct TracingCollectorDaemon {}

#[tonic::async_trait]
impl TracingCollector for TracingCollectorDaemon {
    async fn trace(&self, request: Request<Streaming<Trace>>) -> Result<Response<()>, Status> {
        let mut stream = request.into_inner();

        while let Some(trace) = stream.message().await? {
            println!("received: {:?}", trace);
            // see: https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md
        }

        Ok(Response::new(()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:7777".parse()?;
    let daemon = TracingCollectorDaemon::default();

    Server::builder()
        .add_service(TracingCollectorServer::new(daemon))
        .serve(addr)
        .await?;

    Ok(())
}
