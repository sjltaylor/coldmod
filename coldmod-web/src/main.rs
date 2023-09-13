use std::collections::HashSet;

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

#[component]
fn App(cx: Scope, path: String) -> impl IntoView {
    let rw_filters = create_rw_signal(cx, Option::<HeatmapFilter>::None);
    let (ignore_list, w_ignore_list) = create_signal(cx, IgnoreList::new());
    let (mod_client_connected, w_mod_client_connected) = create_signal(cx, false);

    let heat_srcs_memo = create_memo(cx, move |_| match rw_filters.get() {
        Some(heatmap) => Some(heatmap.heat_srcs()),
        None => None,
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
        Msg::SrcMessage(src_message::PossibleSrcMessage::SrcIgnore(ignore_key)) => {
            w_ignore_list.update(|set| {
                set.insert(ignore_key.key);
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
