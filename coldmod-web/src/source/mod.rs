use crate::dispatch::Dispatch;
use crate::events;
use coldmod_msg::proto::SourceElement;
use coldmod_msg::web;
use leptos::*;

#[component]
pub fn SourceView(cx: Scope) -> impl IntoView {
    let dispatch = use_context::<Dispatch>(cx).unwrap();

    let (source_data, w_source_scan) =
        create_signal(cx, Option::<Option<coldmod_msg::proto::SourceScan>>::None);

    let source_elements = move || source_data().unwrap().unwrap().source_elements;

    dispatch.on_app_event(move |app_event| match app_event {
        events::AppEvent::ColdmodMsg(msg) => match msg {
            web::Msg::SourceDataAvailable(maybe_source_scan) => {
                w_source_scan.set(Some(maybe_source_scan));
            }
            _ => {}
        },
        _ => {}
    });

    // dispatch.emit(events::AppEvent::ColdmodMsg(
    //     coldmod_msg::web::Msg::RequestSourceData,
    // ));

    return view! {cx,
        <Show
            when=move || source_data().is_some()
            fallback=|cx| view! { cx, <div>"Loading..."</div> }>
                <Show
                    when=move || source_data().unwrap().is_some()
                        fallback=|cx| view! { cx, <NoData /> }>
                    <div class="container source-elements">
                        <For
                            each=source_elements
                            key=|u| format!("{:?}", u)
                            view=move |cx, s| view! {cx, <SourceElementView source_element=s /> } />
                    </div>
                </Show>
        </Show>
    };
}

#[component]
pub fn SourceElementView(cx: Scope, source_element: SourceElement) -> impl IntoView {
    if source_element.elem.is_none() {
        return view! {cx, <div>"???"</div> };
    }
    let s = match source_element.elem.unwrap() {
        coldmod_msg::proto::source_element::Elem::Fn(f) => {
            let mut buffer = String::from(format!("{}:{} [name={}]", f.path, f.line, f.name));
            if f.class_name.is_some() {
                buffer.push_str(format!(" [class={}]", f.class_name.unwrap()).as_str());
            }
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
