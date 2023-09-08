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
                            key=|u| format!("{}-{}", u.trace_src.key, u.trace_count)
                            view=move |cx, s| view! {cx, <HeatSourceUI heat_src=s /> } />
                    </ul>
            </div>
        </Show>
    };
}

#[component]
pub fn HeatSourceUI(cx: Scope, heat_src: HeatSrc) -> impl IntoView {
    let mod_client_connected = use_context::<ReadSignal<bool>>(cx).unwrap();
    let trace_src = heat_src.trace_src;

    return view! {cx,
        <li class="heat-src-row">
            <div class="container heat-src">
                <div class="heat-src-trace-count">TRACES:{heat_src.trace_count}</div>
                <Show
                    when=move || mod_client_connected.get()
                    fallback=|cx| view! { cx, <span/> }>
                        <div class="heat-src-trace-count">REFS:123</div>
                </Show>
                <div class="heat-src-fqn">{trace_src.key}</div>
                <Show
                    when=move || mod_client_connected.get()
                    fallback=|cx| view! { cx, <span/> }>
                        <div class="heat-src-controls button-group">
                            <div class="toggle-button">Ignore</div>
                            <div class="toggle-button">Remove</div>
                        </div>
                </Show>
            </div>
        </li>
    };
}

#[component]
pub fn NoDataUI(cx: Scope) -> impl IntoView {
    return view! {cx, <div class="container heatmap nodata"></div> };
}
