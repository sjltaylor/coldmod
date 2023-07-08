use coldmod_msg::web::HeatMap;

use coldmod_msg::proto::SourceFn;

mod clients;

use clients::Clients;

fn heatmap_to_functions_and_counts(heatmap: HeatMap) -> Vec<(i64, SourceFn)> {
    heatmap
        .sources
        .into_iter()
        .map(|elem| {
            let function = match elem.source_element.elem.expect("expected a source element") {
                coldmod_msg::proto::source_element::Elem::Fn(f) => f,
            };
            (elem.trace_count, function)
        })
        .collect()
}

#[tokio::test]
#[ignore]
async fn test_heatmap_initialization() {
    trace_before_source().await;
    source_before_trace().await;
}

// Scenario 1 (trace before source):

// * reset the store
// * create a tracing client
// * send some trace messages
// * connect a source client and send source
// * verify that the heatmap is present and correct
// * verify that the trace stats are correct
// * send another trace
// * verify that the heatmap is up to date
// * verify that the trace stats are correct
//
async fn trace_before_source() {
    let clients = Clients::default();
    clients.reset_state().await;

    let (trace_stats, heatmap) = clients.connect_and_wait_for_initial_messages().await;

    assert_eq!(trace_stats.count, 0);
    assert!(heatmap.is_none());

    clients.send_some_traces().await;
    clients.send_the_source().await;

    let (trace_stats, heatmap) = clients.connect_and_wait_for_initial_messages().await;

    assert_eq!(trace_stats.count, 3);
    assert!(heatmap.is_some());
    let heatmap = heatmap.unwrap();

    let functions_and_counts: Vec<(i64, SourceFn)> = heatmap_to_functions_and_counts(heatmap);

    assert_eq!(functions_and_counts.len(), 3);
    assert!(
        functions_and_counts
            .iter()
            .any(|(count, f)| { f.path == "/a/path/to/a/file" && f.line == 7263 && *count == 2 }),
        "there is a function in the heatmap with two traces"
    );
    assert!(
        functions_and_counts.iter().any(|(count, f)| {
            f.path == "/a/path/to/another/file" && f.line == 191 && *count == 1
        }),
        "there is a function in the heatmap with 1 trace"
    );
    assert!(
        functions_and_counts
            .iter()
            .any(|(count, f)| { f.path == "/a/path/to/a/file" && f.line == 1323 && *count == 0 }),
        "there is a cold function in the heatmap"
    );

    clients.send_some_traces().await;

    let (trace_stats, heatmap) = clients.connect_and_wait_for_initial_messages().await;

    assert_eq!(trace_stats.count, 6);
    assert!(heatmap.is_some());

    let functions_and_counts: Vec<(i64, SourceFn)> =
        heatmap_to_functions_and_counts(heatmap.expect("expected a heatmap"));

    assert!(
        functions_and_counts
            .iter()
            .any(|(count, f)| { f.path == "/a/path/to/a/file" && f.line == 7263 && *count == 4 }),
        "there is a function in the heatmap with two traces"
    );
    assert!(
        functions_and_counts.iter().any(|(count, f)| {
            f.path == "/a/path/to/another/file" && f.line == 191 && *count == 2
        }),
        "there is a function in the heatmap with 1 trace"
    );
    assert!(
        functions_and_counts
            .iter()
            .any(|(count, f)| { f.path == "/a/path/to/a/file" && f.line == 1323 && *count == 0 }),
        "there is a cold function in the heatmap"
    );
}

// Scenario 2 (source before trace):

// * send a source
// * verify the heatmap is up to date
// * send trace events
// * verify the heatmap is up to date
async fn source_before_trace() {
    let clients = Clients::default();
    clients.reset_state().await;
    clients.send_the_source().await;

    let (trace_stats, heatmap) = clients.connect_and_wait_for_initial_messages().await;

    assert_eq!(trace_stats.count, 0);
    assert!(heatmap.is_some());

    let functions_and_counts: Vec<(i64, SourceFn)> =
        heatmap_to_functions_and_counts(heatmap.unwrap());

    assert_eq!(functions_and_counts.len(), 3);
    assert!(
        functions_and_counts
            .iter()
            .any(|(count, f)| { f.path == "/a/path/to/a/file" && f.line == 7263 && *count == 0 }),
        "there is a function in the heatmap with two traces"
    );
    assert!(
        functions_and_counts.iter().any(|(count, f)| {
            f.path == "/a/path/to/another/file" && f.line == 191 && *count == 0
        }),
        "there is a function in the heatmap with 1 trace"
    );
    assert!(
        functions_and_counts
            .iter()
            .any(|(count, f)| { f.path == "/a/path/to/a/file" && f.line == 1323 && *count == 0 }),
        "there is a cold function in the heatmap"
    );

    clients.send_some_traces().await;

    let (trace_stats, heatmap) = clients.connect_and_wait_for_initial_messages().await;

    assert_eq!(trace_stats.count, 3);
    assert!(heatmap.is_some());
    let heatmap = heatmap.unwrap();

    let functions_and_counts: Vec<(i64, SourceFn)> = heatmap_to_functions_and_counts(heatmap);

    assert_eq!(functions_and_counts.len(), 3);
    assert!(
        functions_and_counts
            .iter()
            .any(|(count, f)| { f.path == "/a/path/to/a/file" && f.line == 7263 && *count == 2 }),
        "there is a function in the heatmap with two traces"
    );
    assert!(
        functions_and_counts.iter().any(|(count, f)| {
            f.path == "/a/path/to/another/file" && f.line == 191 && *count == 1
        }),
        "there is a function in the heatmap with 1 trace"
    );
    assert!(
        functions_and_counts
            .iter()
            .any(|(count, f)| { f.path == "/a/path/to/a/file" && f.line == 1323 && *count == 0 }),
        "there is a cold function in the heatmap"
    );

    // connect a heatmap socket
    // send a burst of traces
    // check I get more than 1 event
    // and the deltas sum up to the new heatmap values
}
