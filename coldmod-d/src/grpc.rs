use coldmod_msg::proto::ops_daemon_server::{OpsDaemon, OpsDaemonServer};
use coldmod_msg::proto::source_daemon_server::{SourceDaemon, SourceDaemonServer};
use coldmod_msg::proto::tracing_daemon_server::{TracingDaemon, TracingDaemonServer};
use coldmod_msg::proto::{OpsStatus, SourceScan, Trace};
use coldmod_msg::web::Msg;
use tonic::{transport::Server, Request, Response, Status, Streaming};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

use crate::dispatch::Dispatch;

#[derive(Clone)]
pub struct ColdmodTracingDaemon {
    dispatch: Dispatch,
}

#[derive(Clone)]
pub struct ColdmodSourceDaemon {
    dispatch: Dispatch,
}

#[derive(Clone)]
pub struct ColdmodOpsDaemon {
    dispatch: Dispatch,
}

#[tonic::async_trait]
impl TracingDaemon for ColdmodTracingDaemon {
    async fn collect(&self, request: Request<Streaming<Trace>>) -> Result<Response<()>, Status> {
        // https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md
        let mut stream = request.into_inner();
        while let Some(trace) = stream.message().await? {
            let result = self
                .dispatch
                .handle(coldmod_msg::web::Msg::TraceReceived(trace))
                .await;

            if let Err(e) = result {
                tracing::error!("failed to send trace: {:?}", e);
            }
        }

        Ok(Response::new(()))
    }
}

#[tonic::async_trait]
impl SourceDaemon for ColdmodSourceDaemon {
    async fn submit(&self, request: Request<SourceScan>) -> Result<Response<()>, Status> {
        let scan = request.into_inner();
        match self
            .dispatch
            .handle(coldmod_msg::web::Msg::SourceReceived(scan))
            .await
        {
            Ok(_) => {}
            Err(_e) => {
                return Err(Status::internal("handling failed"));
            }
        }
        Ok(Response::new(()))
    }
}

#[tonic::async_trait]
impl OpsDaemon for ColdmodOpsDaemon {
    async fn status(&self, _: Request<()>) -> Result<Response<OpsStatus>, Status> {
        Ok(Response::new(OpsStatus { ok: true }))
    }

    async fn reset_state(&self, _: Request<()>) -> Result<Response<()>, Status> {
        self.dispatch
            .handle(Msg::Reset)
            .await
            .expect("store to be reset");
        Ok(Response::new(()))
    }
}

pub async fn server(dispatch: &Dispatch) {
    let addr = "127.0.0.1:7777".parse().expect("couldn't parse address");
    let tracing_d = ColdmodTracingDaemon {
        dispatch: dispatch.clone(),
    };
    let source_d = ColdmodSourceDaemon {
        dispatch: dispatch.clone(),
    };
    let ops_d = ColdmodOpsDaemon {
        dispatch: dispatch.clone(),
    };

    let mut builder = Server::builder()
        .layer(
            TraceLayer::new_for_grpc()
                .make_span_with(DefaultMakeSpan::default().include_headers(false)),
        )
        .add_service(TracingDaemonServer::new(tracing_d))
        .add_service(SourceDaemonServer::new(source_d));

    if let Ok(_) = std::env::var("COLDMOD_OPS") {
        builder = builder.add_service(OpsDaemonServer::new(ops_d));
    }

    match builder.serve(addr).await {
        Ok(_) => println!("grpc server exited"),
        Err(e) => eprintln!("grpc server exited with an error: {:?}", e),
    };
}
