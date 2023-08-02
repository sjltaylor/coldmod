use coldmod_msg::web::HeatMap;

use coldmod_msg::proto::TraceSrc;

mod clients;

use clients::Clients;

fn heatmap_to_functions_and_counts(heatmap: HeatMap) -> Vec<(i64, TraceSrc)> {
    heatmap
        .srcs
        .into_iter()
        .map(|elem| (elem.trace_count, elem.trace_src))
        .collect()
}

// * send trace srcs
// * verify the heatmap is up to date
// * send trace events
// * verify the heatmap is up to date
//
// # avoid accidentally running this descrutive test
#[tokio::test]
#[ignore]
async fn test_heatmap() {
    let clients = Clients::default();
    clients.reset_state().await;
    clients.send_tracing_srcs().await;

    let (trace_stats, heatmap) = clients.connect_and_wait_for_initial_messages().await;

    assert_eq!(trace_stats.count, 0);
    assert!(heatmap.is_some());

    let functions_and_counts: Vec<(i64, TraceSrc)> =
        heatmap_to_functions_and_counts(heatmap.unwrap());

    assert_eq!(functions_and_counts.len(), 3);
    assert!(
        functions_and_counts
            .iter()
            .any(|(count, f)| { f.path == "/a/path/to/a/file" && f.lineno == 7263 && *count == 0 }),
        "there is a function in the heatmap with two traces"
    );
    assert!(
        functions_and_counts.iter().any(|(count, f)| {
            f.path == "/a/path/to/another/file" && f.lineno == 191 && *count == 0
        }),
        "there is a function in the heatmap with 1 trace"
    );
    assert!(
        functions_and_counts
            .iter()
            .any(|(count, f)| { f.path == "/a/path/to/a/file" && f.lineno == 1323 && *count == 0 }),
        "there is a cold function in the heatmap"
    );

    clients.send_some_traces().await;

    let (trace_stats, heatmap) = clients.connect_and_wait_for_initial_messages().await;

    assert_eq!(trace_stats.count, 3);
    assert!(heatmap.is_some());
    let heatmap = heatmap.unwrap();

    let functions_and_counts: Vec<(i64, TraceSrc)> = heatmap_to_functions_and_counts(heatmap);

    assert_eq!(functions_and_counts.len(), 3);
    assert!(
        functions_and_counts
            .iter()
            .any(|(count, f)| { f.path == "/a/path/to/a/file" && f.lineno == 7263 && *count == 2 }),
        "there is a function in the heatmap with two traces"
    );
    assert!(
        functions_and_counts.iter().any(|(count, f)| {
            f.path == "/a/path/to/another/file" && f.lineno == 191 && *count == 1
        }),
        "there is a function in the heatmap with 1 trace"
    );
    assert!(
        functions_and_counts
            .iter()
            .any(|(count, f)| { f.path == "/a/path/to/a/file" && f.lineno == 1323 && *count == 0 }),
        "there is a cold function in the heatmap"
    );
}
