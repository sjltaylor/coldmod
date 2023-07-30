use crate::controls_ui::ControlsUI;
use crate::dispatch::Dispatch;
use crate::events;
use crate::filter_state::FilterState;
use crate::heatmap_filter::HeatmapFilter;
use coldmod_msg::web::{self, HeatSource};
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
                    <div class="container heatmap data">
                        <For
                            each=heat_sources
                            key=|u| format!("{}-{}", u.source_element.digest, u.trace_count)
                            view=move |cx, s| view! {cx, <HeatSourceUI heat_source=s /> } />
                    </div>
            </div>
        </Show>
    };
}

#[component]
pub fn HeatSourceUI(cx: Scope, heat_source: HeatSource) -> impl IntoView {
    let trace_src = heat_source.source_element;

    let mut buffer = String::from(format!(
        "{}:{} [name={}]",
        trace_src.path, trace_src.lineno, trace_src.name
    ));
    if let Some(class_name_path) = trace_src.class_name_path {
        buffer.push_str(format!(" [class={}]", class_name_path).as_str());
    }

    return view! {cx, <div>{buffer}" [trace_count="{heat_source.trace_count}"]"</div> };
}

#[component]
pub fn NoDataUI(cx: Scope) -> impl IntoView {
    let hostname = window().location().hostname().unwrap();
    let url = format!("http://{hostname}:7777");
    let cli_cmd = format!("coldmod send --url {url}");

    return view! {cx, <div class="container heatmap nodata">"No data. Use the CLI to scan your source code: "<code>{cli_cmd}</code></div> };
}
