use crate::dispatch::Dispatch;
use crate::events;
use leptos::*;

#[component]
pub fn SourceView(cx: Scope) -> impl IntoView {
    let dispatch = use_context::<Dispatch>(cx).expect("no dispatch");
    let (source_data, _) = create_signal(cx, Option::<String>::None);

    let (sender, _) = dispatch.channel;

    sender
        .try_send(events::AppEvent::ColdmodMsg(
            coldmod_msg::web::Event::RequestSourceData,
        ))
        .expect("failed emit hydrate event");

    return view! {cx,
        <Show
            when=move || !source_data().is_none()
            fallback=|cx| view! { cx, <div>"No Data..."</div> }>
            <code>"data"</code>
        </Show>
    };
}
