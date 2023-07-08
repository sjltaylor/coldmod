use crate::filter_state::FilterState;
use leptos::*;

#[component]
pub fn ControlsUI(cx: Scope) -> impl IntoView {
    let filter_state = FilterState::default();
    let keys = filter_state.keys();
    let groups = [keys[0..1].to_vec(), keys[1..keys.len()].to_vec()];

    let rw_filters = create_rw_signal(cx, filter_state);

    let buttons_classes = move || {
        let mut cx = vec!["buttons"];
        if rw_filters.get().is_ascending() {
            cx.push("ascending");
        } else {
            cx.push("descending");
        }
        cx.join(" ")
    };

    return view! {cx,
    <div class="area controls">
        <div class="container controls">
            <div class={buttons_classes}>
            {groups.map(|group| {
                    view! {cx,
                    <div class="button-group">
                        {group.into_iter().map(|key| {
                            let (is_on,w_is_on) = create_slice(cx, rw_filters,
                                |filter_state| {
                                    filter_state.get(key)
                                },
                                move |filter_state, _: ()| {
                                    filter_state.toggle(key);
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
    w_is_on: SignalSetter<()>,
) -> impl IntoView {
    let label_class = label.clone().to_lowercase();

    let class_name = move || {
        if is_on.get() {
            return format!("toggle-button on {}", label_class);
        }
        return format!("toggle-button off");
    };

    return view! {cx,

    <div class={class_name} on:click=move |_| w_is_on.set(())>
        <div class="toggle-button-label">{label}</div>
    </div> };
}
