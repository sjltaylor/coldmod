use coldmod_msg::proto::source_daemon_server::{SourceDaemon, SourceDaemonServer};
use coldmod_msg::proto::tracing_daemon_server::{TracingDaemon, TracingDaemonServer};
use coldmod_msg::proto::{SourceScan, Trace};
use tonic::{transport::Server, Request, Response, Status, Streaming};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

use crate::dispatch::WebDispatch;

#[derive(Clone)]
pub struct ColdmodTracingDaemon<Dispatch: WebDispatch> {
    dispatch: Dispatch,
}

#[derive(Clone)]
pub struct ColdmodSourceDaemon<Dispatch: WebDispatch> {
    dispatch: Dispatch,
}

#[tonic::async_trait]
impl<Dispatch: WebDispatch> TracingDaemon for ColdmodTracingDaemon<Dispatch> {
    async fn collect(&self, request: Request<Streaming<Trace>>) -> Result<Response<()>, Status> {
        // https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md
        let mut stream = request.into_inner();
        while let Some(trace) = stream.message().await? {
            let result = self
                .dispatch
                .emit(coldmod_msg::web::Event::TraceReceived(trace))
                .await;

            if let Err(e) = result {
                tracing::error!("failed to send trace: {:?}", e);
            }
        }

        Ok(Response::new(()))
    }
}

#[tonic::async_trait]
impl<Dispatch: WebDispatch> SourceDaemon for ColdmodSourceDaemon<Dispatch> {
    async fn submit(&self, request: Request<SourceScan>) -> Result<Response<()>, Status> {
        let scan = request.into_inner();
        let event = coldmod_msg::web::Event::SourceReceived(scan);
        match self.dispatch.emit(event).await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("failed to emit event: {:?}", e);
                return Err(Status::internal("handling failed"));
            }
        }
        Ok(Response::new(()))
    }
}

pub async fn server<Dispatch: WebDispatch>(dispatch: &Dispatch) {
    let addr = "127.0.0.1:7777".parse().expect("couldn't parse address");
    let tracing_d = ColdmodTracingDaemon {
        dispatch: dispatch.clone(),
    };
    let source_d = ColdmodSourceDaemon {
        dispatch: dispatch.clone(),
    };

    match Server::builder()
        .layer(
            TraceLayer::new_for_grpc()
                .make_span_with(DefaultMakeSpan::default().include_headers(false)),
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
