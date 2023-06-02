use crate::dispatch::Dispatch;
use crate::events;
use coldmod_msg::proto::SourceElement;
use leptos::*;

#[component]
pub fn SourceView(cx: Scope) -> impl IntoView {
    let dispatch = use_context::<Dispatch>(cx).expect("no dispatch");

    let (source_data, w_source_scan) =
        create_signal(cx, Option::<Option<coldmod_msg::proto::SourceScan>>::None);

    let source_elements = move || source_data().unwrap().unwrap().source_elements;

    if let Err(e) = dispatch.send(events::AppEvent::ColdmodMsg(
        coldmod_msg::web::Event::RequestSourceData,
    )) {
        error!("failed emit hydrate event {}", e);
    }

    leptos::spawn_local(async move {
        log!("ui waiting for source data");
        while let Ok(app_event) = dispatch.receive().await {
            log!("ui got app event {:?}", app_event);
            match app_event {
                events::AppEvent::ColdmodMsg(event) => {
                    log!("ui matched coldmod msg {:?}", event);
                    match event {
                        coldmod_msg::web::Event::SourceDataAvailable(source_scan) => {
                            log!("ui source data is available, setting: {:?}", source_scan);
                            w_source_scan.set(Some(source_scan));
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    });

    return view! {cx,
        <Show
            when=move || source_data().is_some()
            fallback=|cx| view! { cx, <div>"Loading..."</div> }>
                <Show
                    when=move || source_data().unwrap().is_some()
                        fallback=|cx| view! { cx, <div>"No Data..."</div> }>
                    <For
                        each=source_elements
                        key=|u| format!("{:?}", u)
                        view=move |cx, s| view! {cx, <SourceElementView source_element=s /> } />
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
