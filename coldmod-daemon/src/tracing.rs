use async_channel::Sender;
use coldmod_msg::proto::tracing_daemon_server::{TracingDaemon, TracingDaemonServer};
use coldmod_msg::proto::Trace;
use tonic::{transport::Server, Request, Response, Status, Streaming};

#[derive(Debug)]
pub struct ColdmodTracingDaemon {
    sender: Sender<Trace>,
}

#[tonic::async_trait]
impl TracingDaemon for ColdmodTracingDaemon {
    async fn collect(&self, request: Request<Streaming<Trace>>) -> Result<Response<()>, Status> {
        // https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md
        let mut stream = request.into_inner();
        while let Some(trace) = stream.message().await? {
            println!("recv: {:?}", trace);
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
        .add_service(TracingDaemonServer::new(daemon))
        .serve(addr)
        .await
    {
        Ok(_) => println!("grpc server exited"),
        Err(e) => eprintln!("grpc server exited with an error: {:?}", e),
    };
}
