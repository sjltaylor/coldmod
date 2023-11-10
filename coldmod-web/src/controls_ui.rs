use crate::heatmap_filter::HeatmapFilter;
use coldmod_msg::proto::HeatSrc;
use coldmod_msg::web::TracingStats;
use leptos::*;

#[component]
pub fn ControlsUI() -> impl IntoView {
    let rw_heatmap_filter: RwSignal<Option<HeatmapFilter>> = use_context().unwrap();
    let memo_heat_srcs: Memo<Option<Vec<HeatSrc>>> = use_context().unwrap();
    let tracing_stats: ReadSignal<Option<TracingStats>> = use_context().unwrap();

    let heat_src_count = move || {
        if let Some(heat_srcs) = memo_heat_srcs.get() {
            return format!("{}", heat_srcs.len());
        }
        return "".to_string();
    };

    let trace_count = move || {
        if let Some(tracing_stats) = tracing_stats.get() {
            return format!("{}", tracing_stats.count);
        }
        return "".to_string();
    };

    let keys = rw_heatmap_filter
        .get_untracked()
        .unwrap()
        .filter_state
        .keys();
    let groups = [keys[0..1].to_vec(), keys[1..keys.len()].to_vec()];

    let buttons_classes = move || {
        let mut cx = vec!["buttons"];
        if rw_heatmap_filter.get().unwrap().filter_state.is_ascending() {
            cx.push("ascending");
        } else {
            cx.push("descending");
        }
        cx.join(" ")
    };

    let _cli_connection = move || return "CLI CONNECTED";

    return view! {
    <div class="area controls">
        <div class="container controls">
            <div class={buttons_classes}>
            {groups.map(|group| {
                    view! {
                    <div class="button-group">
                        {group.into_iter().map(|key| {
                            let (is_on,w_is_on) = create_slice(rw_heatmap_filter,
                                |heatmap_filter| {
                                    heatmap_filter.as_ref().unwrap().filter_state.get(key)
                                },
                                move |heatmap_filter, _: ()| {
                                    heatmap_filter.as_mut().unwrap().filter_state.toggle(key);
                                })
                            ;

                            view! {
                                <ToggleButton label=key is_on w_is_on />
                            }
                        }).collect_view()}
                    </div>
                }}).collect_view()}
            </div>
            <div class="container stats">
                <div class="trace-count">"TRACES:"{trace_count}</div>
                <div class="heat-src-count">"SRCS:"{heat_src_count}</div>
            </div>
        </div>
    </div> };
}

#[component]
pub fn ToggleButton(
    #[prop(into)] label: String,
    is_on: Signal<bool>,
    w_is_on: SignalSetter<()>,
) -> impl IntoView {
    let label_class = label.clone().to_lowercase();

    let class_name = move || {
        if is_on.get() {
            return format!("cm-button on {}", label_class);
        }
        return format!("cm-button off");
    };

    return view! {
    <div class={class_name} on:click=move |_| w_is_on.set(())>
        {label}
    </div> };
}
