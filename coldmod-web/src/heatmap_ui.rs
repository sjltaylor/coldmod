use crate::controls_ui::ControlsUI;
use coldmod_msg::web::HeatSrc;
use leptos::*;

#[component]
pub fn HeatMapUI(cx: Scope) -> impl IntoView {
    let heat_srcs_memo = use_context::<Memo<Option<Vec<HeatSrc>>>>(cx).unwrap();

    return view! {cx,
        <Show
            when=move || heat_srcs_memo.get().is_some()
                fallback=|cx| view! { cx, <NoDataUI /> }>
                <div class="container heatmap">
                    <ControlsUI />
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
