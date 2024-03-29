use crate::dispatch::Dispatch;
use coldmod_msg::proto::ops_server::{Ops, OpsServer};

use coldmod_msg::proto::traces_server::{Traces, TracesServer};
use coldmod_msg::proto::{
    FetchOptions, HeatMap, ModCommand, OpsStatus, SrcMessage, Trace, TraceSrcs,
};

use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::{Identity, ServerTlsConfig};
use tonic::{transport::Server, Request, Response, Status, Streaming};

use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tower_http::validate_request::ValidateRequestHeaderLayer;

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
            let stream_result: Result<Option<Trace>, Status> = stream.message().await;
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

    async fn fetch(&self, request: Request<FetchOptions>) -> Result<Response<HeatMap>, Status> {
        let fetch_options = request.into_inner();
        let heat_map = self.dispatch.fetch(fetch_options).await;

        if let Err(_) = heat_map {
            return Err(Status::internal("handling failed"));
        };

        Ok(Response::new(heat_map.unwrap()))
    }

    type modStream = ReceiverStream<Result<ModCommand, Status>>;

    async fn r#mod(
        &self,
        request: Request<Streaming<SrcMessage>>,
    ) -> Result<Response<Self::modStream>, Status> {
        let mut stream = request.into_inner();

        let (tonic_tx, tonic_rx) = mpsc::channel(16);
        let (mod_command_tx, mut mod_command_rx) = mpsc::channel(16);
        let (src_message_tx, src_message_rx) = mpsc::channel(16);

        let dispatch_clone = self.dispatch.clone();

        tokio::spawn(async move {
            dispatch_clone
                .handle_messages_until_closed(mod_command_tx, src_message_rx)
                .await
        });

        tokio::spawn(async move {
            loop {
                let stream_result = stream.message().await;
                match stream_result {
                    Err(e) => {
                        tracing::error!("src message error {:?}", e);
                    }
                    Ok(None) => {
                        tracing::info!("src message stream closed");
                        break;
                    }
                    Ok(Some(src_message)) => {
                        if let Err(e) = src_message_tx.send(src_message).await {
                            tracing::error!("failed to forward src message: {:?}", e);
                        }
                    }
                }
            }
        });

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    cmd = mod_command_rx.recv() => {
                        match cmd {
                            Some(cmd) => {
                                tonic_tx.send(Ok(cmd)).await.unwrap();
                            }
                            None => break,
                        }
                    }
                    _ = tonic_tx.closed() => {
                        mod_command_rx.close();
                        tracing::info!("connect: stream closed");
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

    async fn reset_heatmap(&self, _: Request<()>) -> Result<Response<()>, Status> {
        self.dispatch.reset_all().await.expect("store to be reset");
        Ok(Response::new(()))
    }

    async fn reset_all(&self, _: Request<()>) -> Result<Response<()>, Status> {
        self.dispatch
            .reset_heatmap()
            .await
            .expect("heatmap to be reset");
        Ok(Response::new(()))
    }
}

pub async fn server(dispatch: &Dispatch) {
    let grpc_host = dispatch.grpc_host();
    let api_key = dispatch.api_key();
    let tls = dispatch.tls();

    let tracing_d = Tracing {
        dispatch: dispatch.clone(),
    };
    let ops_d = ColdmodOps {
        dispatch: dispatch.clone(),
    };

    let trace_layer = TraceLayer::new_for_grpc()
        .make_span_with(DefaultMakeSpan::default().include_headers(false));

    let auth_layer = if let Some(api_key) = api_key {
        Some(ValidateRequestHeaderLayer::bearer(api_key.as_str()))
    } else {
        None
    };

    let layer = ServiceBuilder::new()
        .layer(trace_layer)
        .option_layer(auth_layer);

    let mut builder = Server::builder().layer(layer);

    if let Some((cert, key)) = tls {
        let cert_pem = std::fs::read_to_string(cert).unwrap();
        let key_pem = std::fs::read_to_string(key).unwrap();
        tracing::info!("starting grpc with TLS enabled");
        let config = ServerTlsConfig::new().identity(Identity::from_pem(&cert_pem, &key_pem));
        builder = builder.tls_config(config).unwrap();
    }

    let ops_service = if std::env::var("COLDMOD_OPS").unwrap_or("off".to_string()) == "on" {
        Some(OpsServer::new(ops_d))
    } else {
        None
    };

    let builder = builder.add_service(TracesServer::new(tracing_d));
    let builder = builder.add_optional_service(ops_service);

    tracing::info!("starting grpc server on {}", grpc_host);

    match builder.serve(grpc_host).await {
        Ok(_) => println!("grpc server exited"),
        Err(e) => eprintln!("grpc server exited with an error: {:?}", e),
    };
}
