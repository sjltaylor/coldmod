use crate::{filter_state::FilterState, heatmap_filter::HeatmapFilter};
use coldmod_msg::web::Msg;
use heatmap_ui::HeatMapUI;
use leptos::*;

mod coldmod_d;
mod controls_ui;
mod filter_state;
mod heatmap_filter;
mod heatmap_ui;

#[component]
fn App(cx: Scope) -> impl IntoView {
    let rw_filters = create_rw_signal(cx, Option::<HeatmapFilter>::None);

    let sender = coldmod_d::connect(move |msg| match msg {
        Msg::HeatMapAvailable(heat_map) => rw_filters.set(Some(HeatmapFilter {
            filter_state: FilterState::default(),
            heatmap: heat_map,
        })),
        Msg::HeatMapChanged(ref heatmap_delta) => {
            rw_filters.update(|f| f.as_mut().unwrap().update(heatmap_delta));
        }
        _ => {}
    });

    let heat_srcs_memo = create_memo(cx, move |_| match rw_filters.get() {
        Some(heatmap) => Some(heatmap.heat_srcs()),
        None => None,
    });

    create_effect(cx, move |_| {
        if let Some(heat_srcs) = heat_srcs_memo.get() {
            log!("HeatMapUI/count: {}", heat_srcs.len());

            let filterset = coldmod_msg::proto::FilterSet {
                trace_srcs: heat_srcs.into_iter().map(|hs| hs.trace_src).collect(),
            };
            sender.send(Msg::SetFilterSetInContext(filterset));
        }
    });

    provide_context(cx, rw_filters);
    provide_context(cx, heat_srcs_memo);

    return view! { cx,
        <main>
            <HeatMapUI />
        </main>
        <div class="coldmod">"Coldmod"</div>
    };
}

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    mount_to_body(|cx| view! { cx,  <App></App> });
}
