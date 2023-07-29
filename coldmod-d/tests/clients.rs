use coldmod_msg::proto::traces_client::TracesClient;
use coldmod_msg::proto::{ops_daemon_client::OpsDaemonClient, Trace, TraceSrc, TraceSrcs};
use coldmod_msg::web::{HeatMap, TracingStats};
use futures_util::stream;
use futures_util::StreamExt;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

const COLDMOD_D_URL: &str = "ws://127.0.0.1:3333/ws";

#[derive(Default)]
pub struct Clients {}

impl Clients {
    pub async fn reset_state(&self) {
        let mut client = OpsDaemonClient::connect("http://127.0.0.1:7777")
            .await
            .unwrap();

        let response = client.reset_state(()).await;

        assert!(response.is_ok(), "{:?}", response);
    }

    pub async fn send_some_traces(&self) {
        let mut client = TracesClient::connect("http://127.0.0.1:7777")
            .await
            .unwrap();

        let trace_1 = Trace {
            digest: "7263".into(),
            process_id: 1231231,
            thread_id: 1230920,
        };

        let trace_2 = Trace {
            digest: "191".into(),
            process_id: 1231231,
            thread_id: 1230920,
        };

        let trace_3 = Trace {
            digest: "7263".into(),
            process_id: 1231231,
            thread_id: 1230920,
        };

        let traces = vec![trace_1, trace_2, trace_3];
        let response = client.collect(stream::iter(traces)).await;

        assert!(response.is_ok(), "{:?}", response);
    }

    pub async fn send_the_source(&self) {
        let mut client = TracesClient::connect("http://127.0.0.1:7777")
            .await
            .unwrap();

        let es = vec![
            TraceSrc {
                name: "fn_name".into(),
                class_name_path: None,
                path: "/a/path/to/a/file".into(),
                src: "src_code".into(),
                digest: "7263".into(),
                lineno: 7263,
            },
            TraceSrc {
                name: "fn_name".into(),
                class_name_path: None,
                path: "/a/path/to/another/file".into(),
                src: "src_code".into(),
                digest: "191".into(),
                lineno: 191,
            },
            TraceSrc {
                name: "fn_name".into(),
                path: "/a/path/to/a/file".into(),
                class_name_path: None,
                src: "src_code".into(),
                digest: "1323".into(),
                lineno: 1323,
            },
        ];

        let source = TraceSrcs {
            root_path: "/a".into(),
            trace_srcs: es,
        };

        let response = client.register(source).await;

        assert!(response.is_ok(), "{:?}", response);
    }

    pub async fn connect_and_wait_for_initial_messages(&self) -> (TracingStats, Option<HeatMap>) {
        let ws_stream = match connect_async(COLDMOD_D_URL).await {
            Ok((stream, _)) => stream,
            Err(e) => {
                panic!("WebSocket handshake for client failed with {e}!");
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

        let heatmap: Option<coldmod_msg::web::HeatMap> =
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
