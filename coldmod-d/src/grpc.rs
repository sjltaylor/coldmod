use coldmod_msg::proto::source_daemon_server::{SourceDaemon, SourceDaemonServer};
use coldmod_msg::proto::tracing_daemon_server::{TracingDaemon, TracingDaemonServer};
use coldmod_msg::proto::{SourceScan, Trace};
use std::error::Error;
use tonic::{transport::Server, Request, Response, Status, Streaming};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

#[tonic::async_trait]
pub trait DispatchContext: Clone + Send + Sync + 'static {
    async fn store_source_scan(&self, scan: SourceScan) -> Result<(), Box<dyn Error>>;
    fn emit_trace(&self, trace: Trace) -> Result<(), Box<dyn Error>>;
}

#[derive(Clone)]
pub struct ColdmodTracingDaemon<Dispatch: DispatchContext> {
    dispatch: Dispatch,
}

#[derive(Clone)]
pub struct ColdmodSourceDaemon<Dispatch: DispatchContext> {
    dispatch: Dispatch,
}

#[tonic::async_trait]
impl<Dispatch: DispatchContext> TracingDaemon for ColdmodTracingDaemon<Dispatch> {
    async fn collect(&self, request: Request<Streaming<Trace>>) -> Result<Response<()>, Status> {
        // https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md
        let mut stream = request.into_inner();
        while let Some(trace) = stream.message().await? {
            match self.dispatch.emit_trace(trace) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("failed to send trace: {:?}", e);
                }
            }
        }

        Ok(Response::new(()))
    }
}

#[tonic::async_trait]
impl<Dispatch: DispatchContext> SourceDaemon for ColdmodSourceDaemon<Dispatch> {
    async fn submit(&self, request: Request<SourceScan>) -> Result<Response<()>, Status> {
        let scan = request.into_inner();
        println!("received scan request: {:?}", scan);
        self.dispatch.store_source_scan(scan).await.unwrap();
        Ok(Response::new(()))
    }
}

pub async fn server<Dispatch: DispatchContext>(dispatch: Dispatch) {
    let addr = "127.0.0.1:7777".parse().expect("couldn't parse address");
    let tracing_d = ColdmodTracingDaemon {
        dispatch: dispatch.clone(),
    };
    let source_d = ColdmodSourceDaemon { dispatch };

    match Server::builder()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .add_service(TracingDaemonServer::new(tracing_d))
        .add_service(SourceDaemonServer::new(source_d))
        .serve(addr)
        .await
    {
        Ok(_) => println!("grpc server exited"),
        Err(e) => eprintln!("grpc server exited with an error: {:?}", e),
    };
}
