use crate::controls_ui::ControlsUI;
use crate::dispatch::Dispatch;
use crate::events;
use crate::filter_state::FilterState;
use crate::heatmap_filter::HeatmapFilter;
use crate::websocket::WS;
use coldmod_msg::web::{self, HeatSrc};
use leptos::*;

#[component]
pub fn HeatMapUI(cx: Scope) -> impl IntoView {
    let rw_filters = create_rw_signal(cx, Option::<HeatmapFilter>::None);

    let dispatch = use_context::<Dispatch>(cx).unwrap();

    let heat_srcs_memo = create_memo(cx, move |_| match rw_filters.get() {
        Some(heatmap) => Some(heatmap.heat_srcs()),
        None => None,
    });

    let ws = use_context::<WS>(cx).unwrap();

    create_effect(cx, move |_| {
        if let Some(heat_srcs) = heat_srcs_memo.get() {
            log!("HeatMapUI/count: {}", heat_srcs.len());

            let filterset = coldmod_msg::proto::FilterSet {
                trace_srcs: heat_srcs.into_iter().map(|hs| hs.trace_src).collect(),
            };
            ws.send(web::Msg::SetFilterSetInContext(filterset));
        }
    });

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
                            each=move || heat_srcs_memo.get().unwrap()
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
    return view! {cx, <div class="container heatmap nodata"></div> };
}
