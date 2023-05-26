use std::{cell::RefCell, rc::Rc};

use app_btn::*;
use events::*;
use leptos::*;

mod app_btn;
mod events;
mod stream;

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
fn SourceView(cx: Scope) -> impl IntoView {
    return view! {cx,
        <h1>"Source"</h1>
    };
}

#[component]
fn TraceView(cx: Scope) -> impl IntoView {
    return view! {cx,
        <h1>"Trace"</h1>
    };
}

#[component]
pub fn Select<F, W, IV>(
    /// The scope the component is running in
    cx: Scope,
    /// The components Show wraps
    children: Box<dyn Fn(Scope) -> Fragment>,
    /// A closure that returns a bool that determines whether this thing runs
    when: W,
    /// A closure that returns what gets rendered if the when statement is false
    fallback: F,
) -> impl IntoView
where
    W: Fn() -> bool + 'static,
    F: Fn(Scope) -> IV + 'static,
    IV: IntoView,
{
    let memoized_when = create_memo(cx, move |_| when());
    let prev_disposer = Rc::new(RefCell::new(None::<ScopeDisposer>));

    move || {
        if let Some(disposer) = prev_disposer.take() {
            disposer.dispose();
        }
        let (view, disposer) = cx.run_child_scope(|cx| match memoized_when.get() {
            true => children(cx).into_view(cx),
            false => fallback(cx).into_view(cx),
        });
        *prev_disposer.borrow_mut() = Some(disposer);
        view
    }
}

#[component]
fn Container(cx: Scope) -> impl IntoView {
    let (active_view, set_active_view) = create_signal(cx, 0);

    return view! { cx,
        <AppBtn label="Source" on:click=move |_| {set_active_view.update(|s| { *s = 0 })}></AppBtn>
        <AppBtn label="Tracing" on:click=move |_| { set_active_view.update(|s| { *s = 1 })}></AppBtn>
        <main>
            <Select
                when=move || active_view() == 1
                fallback=|cx| view! { cx, <SourceView /> }
            >
            <TraceView />
          </Select>
        </main>
    };
}

fn main() {
    //let (s, r) = async_channel::unbounded::<String>();
    // stream::start(s);

    mount_to_body(|cx| view! { cx,  <Container></Container> });
}
