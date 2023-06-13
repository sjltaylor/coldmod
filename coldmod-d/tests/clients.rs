use coldmod_msg::proto::{
    ops_daemon_client::OpsDaemonClient, source_daemon_client::SourceDaemonClient, source_element,
    tracing_daemon_client::TracingDaemonClient, SourceElement, SourceFn, SourceScan, Trace,
};
use coldmod_msg::web::{HeatMap, TracingStats};
use futures_util::stream;
use futures_util::{StreamExt};
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
        let mut client = TracingDaemonClient::connect("http://127.0.0.1:7777")
            .await
            .unwrap();

        let trace_1 = Trace {
            path: "/a/path/to/a/file".into(),
            line: 7263,
            process_id: 1231231,
            thread_id: 1230920,
        };

        let trace_2 = Trace {
            path: "/a/path/to/another/file".into(),
            line: 191,
            process_id: 1231231,
            thread_id: 1230920,
        };

        let trace_3 = Trace {
            path: "/a/path/to/a/file".into(),
            line: 7263,
            process_id: 1231231,
            thread_id: 1230920,
        };

        let traces = vec![trace_1, trace_2, trace_3];
        let response = client.collect(stream::iter(traces)).await;

        assert!(response.is_ok(), "{:?}", response);
    }

    pub async fn send_the_source(&self) {
        let mut client = SourceDaemonClient::connect("http://127.0.0.1:7777")
            .await
            .unwrap();

        let es = vec![
            SourceElement {
                elem: Some(source_element::Elem::Fn(SourceFn {
                    name: "fn_name".into(),
                    class_name: None,
                    path: "/a/path/to/a/file".into(),
                    line: 7263,
                })),
            },
            SourceElement {
                elem: Some(source_element::Elem::Fn(SourceFn {
                    name: "fn_name".into(),
                    class_name: None,
                    path: "/a/path/to/another/file".into(),
                    line: 191,
                })),
            },
            SourceElement {
                elem: Some(source_element::Elem::Fn(SourceFn {
                    name: "fn_name".into(),
                    class_name: None,
                    path: "/a/path/to/a/file".into(),
                    line: 1323,
                })),
            },
        ];

        let source = SourceScan {
            coldmod_root_marker_path: "/a".into(),
            source_elements: es,
        };

        let response = client.submit(source).await;

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
