use std::time::Duration;

use js_sys;
use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::Window;
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

fn main() {
    let (s, r) = async_channel::unbounded::<String>();
    stream::start(s);
    mount_to_body(|cx| view! { cx,  <Volume recv=r></Volume> });
}
