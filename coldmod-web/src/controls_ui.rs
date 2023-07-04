use std::collections::HashMap;

use leptos::*;

#[component]
pub fn ControlsUI(cx: Scope) -> impl IntoView {
    let keys = vec![vec!["COLD"], vec!["P10", "P20", "P40", "P90", "HOT"]];
    let mut initial_filter_state = HashMap::<&str, bool>::new();
    for group in keys.iter() {
        for key in group {
            initial_filter_state.insert(key, false);
        }
    }

    let rw_filters = create_rw_signal(cx, initial_filter_state);

    create_effect(cx, move |_| {
        log!("filter changed: {:?}", rw_filters.get());
    });

    return view! {cx,
    <div class="area controls">
        <div class="container controls">
            <div class="buttons">
                {keys.into_iter().map(|group| {
                    view! {cx,
                    <div class="button-group">
                        {group.into_iter().map(|key| {
                            let (is_on,w_is_on) = create_slice(cx, rw_filters,
                                |h| {
                                    *h.get(key).unwrap()
                                },
                                |h, n| {
                                    h.insert(key, n);
                                })
                            ;

                            view! {cx,
                                <ToggleButton label=key.clone() is_on w_is_on />
                            }
                        }).collect_view(cx)}
                    </div>
                }}).collect_view(cx)}
            </div>
        </div>
    </div> };
}

#[component]
pub fn ToggleButton(
    cx: Scope,
    #[prop(into)] label: String,
    is_on: Signal<bool>,
    w_is_on: SignalSetter<bool>,
) -> impl IntoView {
    let class_name = move || {
        if is_on.get() {
            return "toggle-button on";
        }
        return "toggle-button off";
    };

    return view! {cx,

    <div class={class_name} on:click=move |_| w_is_on.set(!is_on.get())>
        <div class="toggle-button-label">{label}</div>
    </div> };
}
