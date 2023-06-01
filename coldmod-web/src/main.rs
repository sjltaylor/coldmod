use dispatch::Dispatch;
use leptos::*;
use source::{SourceView, SourceViewProps};

mod dispatch;
mod events;
mod source;
mod websocket;

use crossfire::mpmc;

#[component]
fn Volume(cx: Scope, dispatch: Dispatch) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let (msgs, set_msgs) = create_signal::<Vec<String>>(cx, vec![]);

    leptos::spawn_local(async move {
        while let Ok(msg) = dispatch.receive().await {
            set_msgs.update(|msgs| msgs.push(format!("{:?}", msg)));
        }
    });

    return view! { cx,
        <h1>{"COLDMOD"}</h1>
         <input name="volume" type="range" min="0" max="100" prop:value={count} on:input=move |e| {
             set_count.update(|count| *count = event_target_value(&e).parse::<i32>().expect("a range input had a non integer value"));
         } />{count}<br />
         <For
            each=msgs
            key=|msg| msg.clone()
            view=move |cx, msg| view! {cx, <div>{msg}</div> } />
    };
}

#[component]
fn TraceView(cx: Scope) -> impl IntoView {
    return view! {cx,
        <h1>"Trace"</h1>
    };
}

#[component]
fn Container(cx: Scope, dispatch: Dispatch) -> impl IntoView {
    let (active_view, set_active_view) = create_signal(cx, 0);

    provide_context(cx, dispatch);

    return view! { cx,
        <main>
            <SourceView />
        </main>
    };
}

fn main() {
    let dispatch = Dispatch::new();

    websocket::start(&dispatch);
    // TODO: handle reconnection if there's an error

    mount_to_body(|cx| view! { cx,  <Container dispatch=dispatch></Container> });
}
