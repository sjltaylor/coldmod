use leptos::*;

#[component]
pub fn AppBtn(cx: Scope, #[prop(into)] label: String) -> impl IntoView {
    return view! { cx,
        <div class="app-lug">
            <div class="app-lug-bg"></div>
            <div class="app-lug-label">{label}</div>
        </div>
    };
}
