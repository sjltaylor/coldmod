use async_channel::Sender;
use coldmod_msg::proto::tracing_daemon_server::{TracingDaemon, TracingDaemonServer};
use coldmod_msg::proto::Trace;
use tonic::{transport::Server, Request, Response, Status, Streaming};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

pub struct ColdmodTracingDaemon {
    sender: Sender<Trace>,
}

#[tonic::async_trait]
impl TracingDaemon for ColdmodTracingDaemon {
    async fn collect(&self, request: Request<Streaming<Trace>>) -> Result<Response<()>, Status> {
        // https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md
        let mut stream = request.into_inner();
        while let Some(trace) = stream.message().await? {
            match self.sender.try_send(trace) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("failed to send trace: {:?}", e);
                }
            }
        }

        Ok(Response::new(()))
    }
}

pub async fn server(sender: Sender<Trace>) {
    let addr = "127.0.0.1:7777".parse().expect("couldn't parse address");
    let daemon = ColdmodTracingDaemon { sender };
    match Server::builder()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .add_service(TracingDaemonServer::new(daemon))
        .serve(addr)
        .await
    {
        Ok(_) => println!("grpc server exited"),
        Err(e) => eprintln!("grpc server exited with an error: {:?}", e),
    };
}
