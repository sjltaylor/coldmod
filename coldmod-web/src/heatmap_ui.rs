use crate::events;
use crate::{dispatch::Dispatch, events::AppEvent};

use crate::controls_ui::ControlsUI;
use coldmod_msg::web::{self, ElementKey, HeatSource};
use leptos::*;

#[component]
pub fn HeatMapUI(cx: Scope) -> impl IntoView {
    let dispatch = use_context::<Dispatch>(cx).unwrap();

    let (heat_map, w_heat_map) = create_signal(cx, Option::<coldmod_msg::web::HeatMap>::None);

    let heat_sources = move || heat_map.get().unwrap().sources;

    dispatch.on_app_event(move |app_event| match app_event {
        events::AppEvent::ColdmodMsg(msg) => match msg {
            web::Msg::HeatMapAvailable(heat_map) => {
                w_heat_map.set(Some(heat_map));
            }
            _ => {}
        },
        _ => {}
    });

    return view! {cx,
        <ControlsUI />
        <Show
            when=move || heat_map.get().is_some()
                fallback=|cx| view! { cx, <NoDataUI /> }>
            <div class="container source-elements">
                <For
                    each=heat_sources
                    key=|u| u.source_element.key()
                    view=move |cx, s| view! {cx, <HeatSourceUI heat_source=s /> } />
            </div>
        </Show>
    };
}

#[component]
pub fn HeatSourceUI(cx: Scope, heat_source: HeatSource) -> impl IntoView {
    let dispatch = use_context::<Dispatch>(cx).unwrap();

    let (count, w_count) = create_signal(cx, heat_source.trace_count);

    if heat_source.source_element.elem.is_none() {
        return view! {cx, <div>"???"</div> };
    }

    let s = match heat_source.source_element.elem.as_ref().unwrap() {
        coldmod_msg::proto::source_element::Elem::Fn(f) => {
            let mut buffer = String::from(format!("{}:{} [name={}]", f.path, f.line, f.name));
            if f.class_name.is_some() {
                buffer.push_str(format!(" [class={}]", f.class_name.as_ref().unwrap()).as_str());
            }
            buffer
        }
    };

    dispatch.on_app_event(move |app_event| match app_event {
        AppEvent::SourceElementTraceCountChanged(ref kd) => {
            if kd.0 == heat_source.source_element.key() {
                w_count.update(|c| *c += kd.1);
            }
        }
        _ => {}
    });

    return view! {cx, <div>{s}" [trace_count="{count}"]"</div> };
}

#[component]
pub fn NoDataUI(cx: Scope) -> impl IntoView {
    let hostname = window().location().hostname().unwrap();
    let url = format!("http://{hostname}:7777");
    let cli_cmd = format!("coldmod send --url {url}");

    return view! {cx, <div class="container message">{"No data, use the CLI to scan your source code: "}<code>{cli_cmd}</code></div> };
}
