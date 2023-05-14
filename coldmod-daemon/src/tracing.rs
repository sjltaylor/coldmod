use coldmod_msg::proto::tracing_daemon_server::{TracingDaemon, TracingDaemonServer};
use coldmod_msg::proto::Trace;
use tonic::{transport::Server, Request, Response, Status, Streaming};

#[derive(Debug, Default)]
pub struct ColdmodTracingDaemon {}

#[tonic::async_trait]
impl TracingDaemon for ColdmodTracingDaemon {
    async fn collect(&self, request: Request<Streaming<Trace>>) -> Result<Response<()>, Status> {
        let mut stream = request.into_inner();
        while let Some(trace) = stream.message().await? {
            println!("recv: {:?}", trace);
            // see: https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md
        }

        Ok(Response::new(()))
    }
}

pub async fn server() {
    let addr = "127.0.0.1:7777".parse().expect("couldn't parse address");
    let daemon = ColdmodTracingDaemon::default();
    match Server::builder()
        .add_service(TracingDaemonServer::new(daemon))
        .serve(addr)
        .await
    {
        Ok(_) => println!("grpc server exited"),
        Err(e) => eprintln!("grpc server exited with an error: {:?}", e),
    };
}
