


use dispatch::Dispatch;
use leptos::*;
use source::{SourceView, SourceViewProps};
mod dispatch;
mod events;
mod source;
mod websocket;

#[component]
fn TraceView(cx: Scope) -> impl IntoView {
    return view! {cx,
        <h1>"Trace"</h1>
    };
}

#[component]
fn Container(cx: Scope, dispatch: Dispatch) -> impl IntoView {
    let (_active_view, _set_active_view) = create_signal(cx, 0);

    provide_context(cx, dispatch);

    return view! { cx,
        <main>
            <SourceView />
        </main>
    };
}

fn main() {
    let dispatch = Dispatch::new();

    websocket::start(dispatch.clone());
    // TODO: handle reconnection if there's an error

    mount_to_body(|cx| view! { cx,  <Container dispatch=dispatch></Container> });
}
