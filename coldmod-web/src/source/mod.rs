use crate::dispatch::Dispatch;
use crate::events;

use coldmod_msg::web::{self, ElementKey, HeatSource};
use leptos::*;

#[component]
pub fn SourceView(cx: Scope) -> impl IntoView {
    let dispatch = use_context::<Dispatch>(cx).unwrap();

    let (heat_map, w_heat_map) = create_signal(cx, Option::<coldmod_msg::web::HeatMap>::None);

    let heat_sources = move || heat_map().unwrap().sources;

    dispatch.on_app_event(move |app_event| match app_event {
        events::AppEvent::ColdmodMsg(msg) => match msg {
            web::Msg::HeatMapAvailable(heat_map) => {
                w_heat_map.set(Some(heat_map));
            }
            web::Msg::HeatMapChanged(delta) => {
                w_heat_map.update(|heatmap| {
                    if heatmap.is_none() {
                        error!("no heatmap but we got a heatmap delta :/");
                        return;
                    }
                    let heatmap = heatmap.as_mut().unwrap();

                    for source in heatmap.sources.iter_mut() {
                        let key = source.source_element.key();
                        if let Some(d) = delta.deltas.get(&key) {
                            source.trace_count += d;
                        }
                    }
                });
            }
            _ => {}
        },
        _ => {}
    });

    return view! {cx,
        <Show
            when=move || heat_map().is_some()
            fallback=|cx| view! { cx, <div>"no data."</div> }>
                <Show
                    when=move || heat_map().is_some()
                        fallback=|cx| view! { cx, <NoData /> }>
                    <div class="container source-elements">
                        <For
                            each=heat_sources
                            key=|u| u.source_element.key()
                            view=move |cx, s| view! {cx, <HeatSourceView heat_source=s /> } />
                    </div>
                </Show>
        </Show>
    };
}

#[component]
pub fn HeatSourceView(cx: Scope, heat_source: HeatSource) -> impl IntoView {
    if heat_source.source_element.elem.is_none() {
        return view! {cx, <div>"???"</div> };
    }
    let s = match heat_source.source_element.elem.unwrap() {
        coldmod_msg::proto::source_element::Elem::Fn(f) => {
            let mut buffer = String::from(format!("{}:{} [name={}]", f.path, f.line, f.name));
            if f.class_name.is_some() {
                buffer.push_str(format!(" [class={}]", f.class_name.unwrap()).as_str());
            }
            buffer.push_str(format!(" [trace_count={}]", heat_source.trace_count).as_str());
            buffer
        }
    };
    return view! {cx, <div>{s}</div> };
}

#[component]
pub fn NoData(cx: Scope) -> impl IntoView {
    let hostname = window().location().hostname().unwrap();
    let url = format!("http://{hostname}:7777");
    let cli_cmd = format!("coldmod send --url {url}");

    return view! {cx, <div class="container message">{"No data, use the CLI to scan your source code: "}<code>{cli_cmd}</code></div> };
}
