use coldmod_msg::web;
use leptos::*;

#[component]
pub fn TracingStatusUI(cx: Scope) -> impl IntoView {
    let tracing_status = use_context::<ReadSignal<Option<web::TracingStats>>>(cx).unwrap();

    let tracing_status_repr = move || {
        if tracing_status.get().is_none() {
            return "-".into();
        }
        format!("∑ trace_count:{}", tracing_status.get().unwrap().count)
    };

    return view! {cx,
        <div class="container tracing-status">{tracing_status_repr}</div>
    };
}
