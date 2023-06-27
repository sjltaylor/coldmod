use coldmod_msg::web;
use console_error_panic_hook;
use dispatch::Dispatch;
use heatmap_ui::{HeatMapUI, HeatMapUIProps};
use leptos::*;


mod dispatch;
mod events;
mod heatmap_ui;
mod source;
mod websocket;

#[component]
fn TracingStatus(cx: Scope) -> impl IntoView {
    let dispatch = use_context::<Dispatch>(cx).unwrap();
    let (tracing_status, w_tracing_stats) = create_signal(cx, Option::<web::TracingStats>::None);
    let tracing_status_repr = move || {
        if tracing_status().is_none() {
            return "-".into();
        }
        format!("TRACING: event count={}", tracing_status().unwrap().count)
    };

    dispatch.on_app_event(move |app_event| match app_event {
        events::AppEvent::ColdmodMsg(msg) => match msg {
            web::Msg::TracingStatsAvailable(tracing_stats) => {
                w_tracing_stats.set(Some(tracing_stats));
            }
            _ => {}
        },
        _ => {}
    });

    return view! {cx,
        <div class="container tracing-status">{tracing_status_repr}</div>
    };
}

#[component]
fn Container(cx: Scope, dispatch: Dispatch) -> impl IntoView {
    let (_active_view, _set_active_view) = create_signal(cx, 0);

    provide_context(cx, dispatch);

    return view! { cx,
        <TracingStatus />
        <main>
            <HeatMapUI />
        </main>
    };
}

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let dispatch = Dispatch::new();

    websocket::start(dispatch.clone());

    mount_to_body(|cx| view! { cx,  <Container dispatch=dispatch></Container> });
}
