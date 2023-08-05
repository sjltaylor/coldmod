use coldmod_msg::proto::ops_server::{Ops, OpsServer};
use coldmod_msg::proto::traces_server::{Traces, TracesServer};
use coldmod_msg::proto::{FilterSet, FilterSetQuery, OpsStatus, Trace, TraceSrcs};
use coldmod_msg::web::Msg;
use tonic::{transport::Server, Request, Response, Status, Streaming};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

use crate::dispatch::Dispatch;

#[derive(Clone)]
pub struct Tracing {
    dispatch: Dispatch,
}

#[derive(Clone)]
pub struct ColdmodOps {
    dispatch: Dispatch,
}

#[tonic::async_trait]
impl Traces for Tracing {
    async fn collect(&self, request: Request<Streaming<Trace>>) -> Result<Response<()>, Status> {
        // https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md
        let mut stream = request.into_inner();

        loop {
            let stream_result = stream.message().await;
            match stream_result {
                Ok(Some(trace)) => {
                    let result = self
                        .dispatch
                        .handle(coldmod_msg::web::Msg::TraceReceived(trace))
                        .await;
                    if let Err(e) = result {
                        tracing::error!("failed to handle trace: {:?}", e);
                    }
                }
                Ok(None) => {
                    tracing::warn!("the trace was None");
                    continue;
                }
                Err(e) => {
                    tracing::warn!("failed to receive trace: {:?}", e);
                    return Err(Status::internal("failed to receive trace"));
                }
            }
        }
    }

    async fn set(&self, request: Request<TraceSrcs>) -> Result<Response<()>, Status> {
        let scan = request.into_inner();
        match self
            .dispatch
            .handle(coldmod_msg::web::Msg::SetTraceSrcs(scan))
            .await
        {
            Ok(_) => {}
            Err(_e) => {
                return Err(Status::internal("handling failed"));
            }
        }
        Ok(Response::new(()))
    }

    async fn get_filterset(
        &self,
        request: Request<FilterSetQuery>,
    ) -> Result<Response<FilterSet>, Status> {
        let query = request.into_inner();
        match self
            .dispatch
            .handle(coldmod_msg::web::Msg::GetFilterSet(query))
            .await
        {
            Ok(mut replies) => match replies.len() {
                1 => {
                    match replies.remove(0) {
                        Msg::FilterSetAvailable(filter_set) => {
                            return Ok(Response::new(filter_set))
                        }
                        _ => {
                            tracing::error!("unexpected reply type from dispatch");
                            return Err(Status::internal("handling failed"));
                        }
                    };
                }
                _ => {
                    tracing::error!("unexpected reply count from dispatch");
                    return Err(Status::internal("handling failed"));
                }
            },
            Err(_e) => {
                return Err(Status::internal("handling failed"));
            }
        }
    }
}

#[tonic::async_trait]
impl Ops for ColdmodOps {
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
    let tracing_d = Tracing {
        dispatch: dispatch.clone(),
    };
    let ops_d = ColdmodOps {
        dispatch: dispatch.clone(),
    };

    let mut builder = Server::builder()
        .layer(
            TraceLayer::new_for_grpc()
                .make_span_with(DefaultMakeSpan::default().include_headers(false)),
        )
        .add_service(TracesServer::new(tracing_d));

    if let Ok(_) = std::env::var("COLDMOD_OPS") {
        builder = builder.add_service(OpsServer::new(ops_d));
    }

    match builder.serve(addr).await {
        Ok(_) => println!("grpc server exited"),
        Err(e) => eprintln!("grpc server exited with an error: {:?}", e),
    };
}
