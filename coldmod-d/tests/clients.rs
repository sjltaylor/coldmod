use coldmod_msg::proto::traces_client::TracesClient;
use coldmod_msg::proto::{ops_client::OpsClient, HeatMap, Trace, TraceSrc, TraceSrcs};
use coldmod_msg::web::TracingStats;
use futures_util::stream;
use futures_util::StreamExt;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tonic::transport::Channel;

fn coldmod_ws_url() -> String {
    format!("ws://{}/ws", std::env::var("COLDMOD_WEB_HOST").unwrap())
}

fn coldmod_grpc_host() -> String {
    format!("http://{}", std::env::var("COLDMOD_GRPC_HOST").unwrap())
}

async fn traces_client() -> TracesClient<Channel> {
    return TracesClient::connect(coldmod_grpc_host()).await.unwrap();
}

#[derive(Default)]
pub struct Clients {}

impl Clients {
    pub async fn reset_state(&self) {
        let mut client = OpsClient::connect(coldmod_grpc_host()).await.unwrap();

        let response = client.reset_all(()).await;

        assert!(response.is_ok(), "{:?}", response);
    }

    pub async fn send_some_traces(&self) {
        let mut client = traces_client().await;

        let trace_1 = Trace {
            key: "/a/path/to/a/file:7263".into(),
            process_id: "1231231".into(),
            thread_id: "1230920".into(),
        };

        let trace_2 = Trace {
            key: "/a/path/to/another/file:191".into(),
            process_id: "1231231".into(),
            thread_id: "1230920".into(),
        };

        let trace_3 = Trace {
            key: "/a/path/to/a/file:7263".into(),
            process_id: "1231231".into(),
            thread_id: "1230920".into(),
        };

        let traces = vec![trace_1, trace_2, trace_3];
        let response = client.collect(stream::iter(traces)).await;

        assert!(response.is_ok(), "{:?}", response);
    }

    pub async fn send_tracing_srcs(&self) {
        let mut client = traces_client().await;

        let es = vec![
            TraceSrc {
                key: "/a/path/to/a/file:7263".into(),
            },
            TraceSrc {
                key: "/a/path/to/another/file:191".into(),
            },
            TraceSrc {
                key: "/a/path/to/a/file:1323".into(),
            },
        ];

        let source = TraceSrcs { trace_srcs: es };

        let response = client.set(source).await;

        assert!(response.is_ok(), "{:?}", response);
    }

    pub async fn connect_and_wait_for_initial_messages(&self) -> (TracingStats, Option<HeatMap>) {
        let ws_stream = match connect_async(format!("{}/connect/test-key", coldmod_ws_url())).await
        {
            Ok((stream, _)) => stream,
            Err(e) => {
                panic!("WebSocket handshake for client failed with {e}");
            }
        };

        let (_, mut receiver) = ws_stream.split();

        let wait = Duration::from_millis(10);

        let message_1 = timeout(wait, receiver.next())
            .await
            .expect("timed out")
            .expect("there to me a result")
            .expect("there to be a message");

        let message_1: coldmod_msg::web::Msg = match message_1 {
            Message::Binary(payload) => {
                flexbuffers::from_slice(payload.as_slice()).expect("error deserializing message")
            }
            _ => panic!("unexpected websocket message type {:?}", message_1),
        };

        let tracing_stats = match message_1 {
            coldmod_msg::web::Msg::TracingStatsAvailable(tracing_stats) => tracing_stats,
            _ => panic!("expected tracing stats"),
        };

        let message_2 = match timeout(wait, receiver.next()).await {
            Ok(Some(r)) => Some(r.expect("there to be a message")),
            _ => None,
        };

        let heatmap: Option<coldmod_msg::proto::HeatMap> =
            message_2.map(|message_2| match message_2 {
                Message::Binary(payload) => {
                    let msg: coldmod_msg::web::Msg = flexbuffers::from_slice(payload.as_slice())
                        .expect("error deserializing message");
                    match msg {
                        coldmod_msg::web::Msg::HeatMapAvailable(heatmap) => heatmap,
                        _ => panic!("expected heatmap"),
                    }
                }
                _ => panic!("unexpected websocket message type {:?}", message_2),
            });

        (tracing_stats, heatmap)
    }
}
