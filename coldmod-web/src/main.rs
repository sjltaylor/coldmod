use console_error_panic_hook;
use dispatch::Dispatch;
use heatmap_ui::HeatMapUI;
use leptos::*;
use websocket::WS;

mod controls_ui;
mod dispatch;
mod events;
mod filter_state;
mod heatmap_filter;
mod heatmap_ui;
mod websocket;

#[component]
fn App(cx: Scope, ws: WS, dispatch: Dispatch) -> impl IntoView {
    let (_active_view, _set_active_view) = create_signal(cx, 0);

    provide_context(cx, dispatch.clone());
    provide_context(cx, ws);

    return view! { cx,
        <main>
            <HeatMapUI />
        </main>
        <div class="coldmod">"Coldmod"</div>
    };
}

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let dispatch = Dispatch::new();

    let ws = websocket::start(dispatch.clone());

    mount_to_body(|cx| view! { cx,  <App ws dispatch=dispatch></App> });
}
