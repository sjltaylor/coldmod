use std::collections::{HashMap, HashSet};

use crate::{filter_state::FilterState, heatmap_filter::HeatmapFilter};
use coldmod_msg::{
    proto::{mod_command::Command, src_message, ModCommand, SendSrcInfo},
    web::Msg,
};
use heatmap_ui::HeatMapUI;

use base64::{engine::general_purpose, Engine as _};
use leptos::*;

mod coldmod_d;
mod controls_ui;
mod filter_state;
mod heatmap_filter;
mod heatmap_ui;

type IgnoreList = HashSet<String>;
type RemoveList = Memo<HashSet<String>>;

#[component]
fn App(cx: Scope, path: String) -> impl IntoView {
    let rw_filters = create_rw_signal(cx, Option::<HeatmapFilter>::None);
    let (ignore_list, w_ignore_list) = create_signal(cx, IgnoreList::new());
    let (src_available_list, w_src_available_list) =
        create_signal(cx, Option::<HashSet<String>>::None);
    let (mod_client_connected, w_mod_client_connected) = create_signal(cx, false);
    let (src_refs_by_key, w_src_refs_by_key) = create_signal(cx, HashMap::<String, u32>::new());

    let heat_srcs_memo = create_memo(cx, move |_| match rw_filters.get() {
        Some(heatmap) => Some(heatmap.heat_srcs()),
        None => None,
    });

    let removable_memo = create_memo(cx, move |_| {
        let srcs_available = src_available_list.get(); // map Some or None
        let heat_srcs = heat_srcs_memo.get(); // map to Some or None

        match (srcs_available, heat_srcs) {
            (Some(srcs_available), Some(heat_srcs)) => {
                return srcs_available
                    .intersection(
                        &heat_srcs
                            .iter()
                            .filter_map(|hs| {
                                if hs.trace_count == 0 {
                                    Some(hs.trace_src.key.clone())
                                } else {
                                    None
                                }
                            })
                            .collect(),
                    )
                    .cloned()
                    .collect::<HashSet<String>>();
            }
            (_, _) => return HashSet::<String>::new(),
        }
    });

    let sender = coldmod_d::connect(path, move |msg, sender| match msg {
        Msg::HeatMapAvailable(heat_map) => rw_filters.set(Some(HeatmapFilter {
            filter_state: FilterState::default(),
            heatmap: heat_map,
        })),
        Msg::HeatMapChanged(ref heatmap_delta) => {
            rw_filters.update(|f| f.as_mut().unwrap().update(heatmap_delta));
        }
        Msg::ModCommandClientAvailable => {
            log!("ModCommandClientAvailable");
            w_mod_client_connected.set(true);
            let command = Some(Command::SendSrcInfo(SendSrcInfo {}));
            let msg = Msg::RouteModCommand(ModCommand { command });
            sender.send(msg)
        }
        Msg::ModCommandClientUnavailable => {
            log!("ModCommandClientUnavailable");
            w_mod_client_connected.set(false);
        }
        Msg::SrcMessage(src_message::PossibleSrcMessage::SrcIgnore(src_ignore)) => {
            w_ignore_list.update(|set| {
                set.insert(src_ignore.key);
            });
        }
        Msg::SrcMessage(src_message::PossibleSrcMessage::SrcAvailable(src_available)) => {
            w_src_available_list.set(Some(src_available.keys.into_iter().collect()));
        }
        Msg::SrcMessage(src_message::PossibleSrcMessage::SrcRefs(src_refs)) => {
            w_src_refs_by_key.update(|map| {
                map.insert(src_refs.key, src_refs.count);
            });
        }
        _ => log!("unhandled msg: {:?}", msg),
    });

    let sender = sender.clone();

    provide_context(cx, rw_filters);
    provide_context(cx, heat_srcs_memo);
    provide_context(cx, mod_client_connected);
    provide_context(cx, sender);
    provide_context(cx, ignore_list);
    provide_context(cx, removable_memo);
    provide_context(cx, src_refs_by_key);

    return view! { cx,
        <main>
            <HeatMapUI />
        </main>
        <div class="coldmod">"Coldmod"</div>
    };
}

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let location = document().location().unwrap();
    let mut path = location.pathname().unwrap();

    if !path.starts_with("/connect/") {
        // generate 32 random b ytes and base64 encode them
        let mut buf = [0u8; 32];
        let crypto = window().crypto().unwrap();
        crypto
            .get_random_values_with_u8_array(&mut buf[..])
            .expect("cryto not to fail");

        path = format!(
            "/connect/web-{}",
            general_purpose::URL_SAFE_NO_PAD.encode(buf)
        );

        window()
            .history()
            .unwrap()
            .push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&path))
            .unwrap();
    };

    mount_to_body(|cx| view! { cx,  <App path></App> });
}
