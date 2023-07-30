use coldmod_msg::web;
use console_error_panic_hook;
use dispatch::Dispatch;
use heatmap_ui::HeatMapUI;
use leptos::*;
use tracing_status_ui::TracingStatusUI;

mod controls_ui;
mod dispatch;
mod events;
mod filter_state;
mod heatmap_filter;
mod heatmap_ui;
mod tracing_status_ui;
mod websocket;

#[component]
fn App(cx: Scope, dispatch: Dispatch) -> impl IntoView {
    let (_active_view, _set_active_view) = create_signal(cx, 0);

    provide_context(cx, dispatch.clone());

    let (tracing_status, w_tracing_stats) = create_signal(cx, Option::<web::TracingStats>::None);

    provide_context(cx, tracing_status);

    dispatch.on_app_event(move |app_event| match app_event {
        events::AppEvent::ColdmodMsg(msg) => match msg {
            web::Msg::TracingStatsAvailable(tracing_stats) => {
                w_tracing_stats.set(Some(tracing_stats));
            }
            _ => {}
        },
        _ => {}
    });

    return view! { cx,
        <main>
            <TracingStatusUI />
            <HeatMapUI />
        </main>
        <div class="coldmod">"Coldmod"</div>
    };
}

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let dispatch = Dispatch::new();

    websocket::start(dispatch.clone());

    mount_to_body(|cx| view! { cx,  <App dispatch=dispatch></App> });
}
