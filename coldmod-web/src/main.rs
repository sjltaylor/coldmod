use app_btn::*;
use dispatch::Dispatch;
use leptos::*;
use source::{SourceView, SourceViewProps};

mod app_btn;
mod dispatch;
mod events;
mod source;
mod websocket;

#[component]
fn Volume(cx: Scope, recv: async_channel::Receiver<String>) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let (msgs, set_msgs) = create_signal::<Vec<String>>(cx, vec![]);

    leptos::spawn_local(async move {
        while let Ok(msg) = recv.recv().await {
            set_msgs.update(|msgs| msgs.push(msg));
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
        <AppBtn label="Source" on:click=move |_| {set_active_view.update(|s| { *s = 0 })}></AppBtn>
        <AppBtn label="Tracing" on:click=move |_| { set_active_view.update(|s| { *s = 1 })}></AppBtn>
        <main>
            <Show
                when=move || active_view() == 1
                fallback=|cx| view! { cx, <SourceView /> }
            >
            <TraceView />
          </Show>
        </main>
    };
}

fn main() {
    //let (s, r) = async_channel::unbounded::<String>();
    // stream::start(s);
    let dispatch = Dispatch::new();

    websocket::start(&dispatch);
    // TODO: handle reconnection if there's an error

    mount_to_body(|cx| view! { cx,  <Container dispatch=dispatch></Container> });
}
