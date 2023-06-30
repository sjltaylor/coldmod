use crate::dispatch::Dispatch;
use leptos::*;

#[component]
pub fn ControlsUI(cx: Scope) -> impl IntoView {
    let dispatch = use_context::<Dispatch>(cx).unwrap();

    return view! {cx,
    <div class="area controls">
        <div class="container controls">
            <div class="buttons">
                <ToggleButton label="COLD" />
                <ToggleButton label="P10" />
                <ToggleButton label="P40" />
                <ToggleButton label="ALL" />
            </div>
        </div>
    </div> };
}

#[component]
pub fn ToggleButton(cx: Scope, #[prop(into)] label: String) -> impl IntoView {
    let (is_on, w_is_on) = create_signal(cx, false);

    let class_name = move || {
        if is_on() {
            return "toggle-button on";
        }
        return "toggle-button off";
    };

    return view! {cx,
    <div class={class_name} on:click=move |_| w_is_on.update(|v| *v = !*v)>
        <div class="toggle-button-label">{label}</div>
    </div> };
}
