use coldmod_msg::web;

use crate::dispatch::Dispatch;
use crate::events;
use leptos::*;

#[component]
pub fn TracingStatusUI(cx: Scope) -> impl IntoView {
    let dispatch = use_context::<Dispatch>(cx).unwrap();
    let (tracing_status, w_tracing_stats) = create_signal(cx, Option::<web::TracingStats>::None);
    let tracing_status_repr = move || {
        if tracing_status.get().is_none() {
            return "-".into();
        }
        format!("∑ trace_count:{}", tracing_status.get().unwrap().count)
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
