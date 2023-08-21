use crate::dispatch::Dispatch;
use coldmod_msg::proto::ops_server::{Ops, OpsServer};
use coldmod_msg::proto::traces_server::{Traces, TracesServer};
use coldmod_msg::proto::{FilterSet, FilterSetQuery, OpsStatus, Trace, TraceSrcs};

use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status, Streaming};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

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
                    if let Err(e) = self.dispatch.trace_received(trace).await {
                        tracing::error!("failed to handle trace: {:?}", e);
                    }
                }
                Ok(None) => {
                    return Ok(Response::new(()));
                }
                Err(e) => {
                    tracing::warn!("failed to receive trace: {:?}", e);
                    return Err(Status::internal("failed to receive trace"));
                }
            }
        }
    }

    async fn set(&self, request: Request<TraceSrcs>) -> Result<Response<()>, Status> {
        let trace_srcs = request.into_inner();
        match self.dispatch.set_trace_srcs(trace_srcs).await {
            Ok(_) => {}
            Err(_e) => {
                return Err(Status::internal("handling failed"));
            }
        }
        Ok(Response::new(()))
    }

    type stream_filtersetsStream = ReceiverStream<Result<FilterSet, Status>>;

    async fn stream_filtersets(
        &self,
        request: Request<FilterSetQuery>,
    ) -> Result<Response<Self::stream_filtersetsStream>, Status> {
        let q = request.into_inner();

        // TODO: this could probably use tokio::sync::watch
        // the CLI only needs the latest filterset
        let (tonic_tx, tonic_rx) = mpsc::channel(16);
        let (dispatch_tx, mut dispatch_rx) = mpsc::channel(16);

        let dispatch_clone = self.dispatch.clone();

        tokio::spawn(async move {
            dispatch_clone
                .send_filtersets_until_closed(q, dispatch_tx)
                .await;
        });

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    filterset = dispatch_rx.recv() => {
                        match filterset {
                            Some(filterset) => {
                                tonic_tx.send(Ok(filterset)).await.unwrap();
                            }
                            None => break,
                        }
                    }
                    _ = tonic_tx.closed() => {
                        dispatch_rx.close();
                        tracing::info!("stream_filtersets: stream closed");
                        break;
                    }
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(tonic_rx)))
    }
}

#[tonic::async_trait]
impl Ops for ColdmodOps {
    async fn status(&self, _: Request<()>) -> Result<Response<OpsStatus>, Status> {
        Ok(Response::new(OpsStatus { ok: true }))
    }

    async fn reset_state(&self, _: Request<()>) -> Result<Response<()>, Status> {
        self.dispatch
            .reset_state()
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
