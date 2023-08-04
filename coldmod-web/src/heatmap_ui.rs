use crate::controls_ui::ControlsUI;
use crate::dispatch::Dispatch;
use crate::events;
use crate::filter_state::FilterState;
use crate::heatmap_filter::HeatmapFilter;
use coldmod_msg::web::{self, HeatSrc};
use leptos::*;

#[component]
pub fn HeatMapUI(cx: Scope) -> impl IntoView {
    let rw_filters = create_rw_signal(cx, Option::<HeatmapFilter>::None);

    let dispatch = use_context::<Dispatch>(cx).unwrap();

    let heat_sources = move || rw_filters.get().unwrap().sources();

    dispatch.on_app_event(move |app_event| match app_event {
        events::AppEvent::ColdmodMsg(msg) => match msg {
            web::Msg::HeatMapAvailable(heat_map) => rw_filters.set(Some(HeatmapFilter {
                filter_state: FilterState::default(),
                heatmap: heat_map,
            })),
            web::Msg::HeatMapChanged(ref heatmap_delta) => {
                rw_filters.update(|f| f.as_mut().unwrap().update(heatmap_delta));
            }
            _ => {}
        },
        _ => {}
    });

    return view! {cx,

        <Show
            when=move || rw_filters.get().is_some()
                fallback=|cx| view! { cx, <NoDataUI /> }>
                <div class="container heatmap">
                    <ControlsUI rw_filters />
                    <ul class="container heatmap data">
                        <For
                            each=heat_sources
                            key=|u| format!("{}-{}", u.trace_src.digest, u.trace_count)
                            view=move |cx, s| view! {cx, <HeatSourceUI heat_src=s /> } />
                    </ul>
            </div>
        </Show>
    };
}

#[component]
pub fn HeatSourceUI(cx: Scope, heat_src: HeatSrc) -> impl IntoView {
    let trace_src = heat_src.trace_src;

    let loc = format!("{}:{}", trace_src.path, trace_src.lineno);

    let mut buffer = String::new();

    if let Some(class_name_path) = trace_src.class_name_path {
        buffer.push_str(&class_name_path);
        buffer.push_str(".");
    }

    buffer.push_str(&trace_src.name);

    return view! {cx,
        <li class="container heat-src">
            <div class="heat-src-count">
                <div class="heat-src-count-label">Traces</div>
                <div class="heat-src-count-value">{heat_src.trace_count}</div>
            </div>
            <div class="heat-src-name">{buffer}</div>
            <div class="heat-src-loc">{loc}</div>
        </li>
    };
}

#[component]
pub fn NoDataUI(cx: Scope) -> impl IntoView {
    let hostname = window().location().hostname().unwrap();
    let url = format!("http://{hostname}:7777");
    let cli_cmd = format!("coldmod send --url {url}");

    return view! {cx, <div class="container heatmap nodata">"No data. Use the CLI to scan your source code: "<code>{cli_cmd}</code></div> };
}
