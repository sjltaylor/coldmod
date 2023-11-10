use std::collections::HashMap;

use crate::{coldmod_d::Sender, controls_ui::ControlsUI, IgnoreList, SrcAvailableList};
use coldmod_msg::{
    proto::{mod_command::Command, HeatSrc, IgnoreCommand, ModCommand, OpenCommand, RemoveCommand},
    web::Msg,
};
use leptos::*;

#[component]
pub fn HeatMapUI() -> impl IntoView {
    let heat_srcs_memo = use_context::<Memo<Option<Vec<HeatSrc>>>>().unwrap();

    return view! {
        <Show
            when=move || heat_srcs_memo.get().is_some()
                fallback=|| view! { <NoDataUI /> }>
                <div class="container heatmap">
                    <ControlsUI />
                    <ul class="container heatmap data">
                        <For
                            each=move || heat_srcs_memo.get().unwrap()
                            key=|u| format!("{}-{}", u.key, u.trace_count)
                            children=move |s| view! {<HeatSourceUI heat_src=s /> } />
                    </ul>
            </div>
        </Show>
    };
}

#[component]
pub fn HeatSourceUI(heat_src: HeatSrc) -> impl IntoView {
    let mod_client_connected = use_context::<ReadSignal<bool>>().unwrap();
    let ignore_list = use_context::<ReadSignal<IgnoreList>>().unwrap();
    let src_available_list = use_context::<SrcAvailableList>().unwrap();
    let src_refs_by_key = use_context::<ReadSignal<HashMap<String, u32>>>().unwrap();
    let sender = use_context::<Sender>().unwrap();
    let (command, w_command) = create_signal::<Option<Command>>(None);

    let key = heat_src.key.clone();
    let is_ignored = move || ignore_list.get().contains(&key);

    let ignore_classname = move || {
        let mut buffer: Vec<String> = vec!["container heat-src".to_string()];

        if is_ignored() {
            buffer.push("ignore".to_string());
        }

        buffer.join(" ")
    };

    create_effect(move |_| match command.get() {
        Some(command) => {
            let msg = Msg::RouteModCommand(ModCommand {
                command: Some(command),
            });
            sender.send(msg);
        }
        _ => {}
    });

    let key = heat_src.key.clone();
    let refs_view = move || {
        let key = key.clone();
        let maybe_ref = src_refs_by_key.get();
        let refs = maybe_ref.get(&key);

        if mod_client_connected.get() {
            let refs = if let Some(refs) = refs {
                format!("{}", refs)
            } else {
                "--".to_string()
            };
            Some(view! {
                <div class="heat-src-stat">REFS:{refs}</div>
            })
        } else {
            None
        }
    };

    let key = heat_src.key.clone();
    let controls_view = move || {
        let key = key.clone();
        if mod_client_connected.get() && !ignore_list.get().contains(&key) {
            let src_available = src_available_list.get().contains(&key);
            Some(view! {
                <HeatSourceControlsUI key w_command src_available />
            })
        } else {
            None
        }
    };

    let key = heat_src.key.clone();

    return view! {
        <li class="heat-src-row">
            <div class={ignore_classname}>
                <div class="heat-src-stat">TRACES:{heat_src.trace_count}</div>
                { refs_view }
                <div class="heat-src-fqn">{key}</div>
                { controls_view }
            </div>
        </li>
    };
}

#[component]
pub fn HeatSourceControlsUI(
    key: String,
    w_command: WriteSignal<Option<Command>>,
    src_available: bool,
) -> impl IntoView {
    let key_clone = key.clone();
    let open_button = move || {
        if src_available {
            let key_clone = key_clone.clone();
            let on_open = move |_| {
                w_command.set(Some(Command::Open(OpenCommand {
                    key: key_clone.clone(),
                })));
            };

            Some(view! {<div class="cm-button small" on:click=on_open>Open</div>})
        } else {
            None
        }
    };

    let key_clone = key.clone();
    let on_ignore = move |_| {
        w_command.set(Some(Command::Ignore(IgnoreCommand {
            key: key_clone.clone(),
        })));
    };

    let remove_button = move || {
        if src_available {
            let key_clone = key.clone();
            let on_remove = move |_| {
                w_command.set(Some(Command::Remove(RemoveCommand {
                    key: key_clone.clone(),
                })));
            };

            Some(view! {<div class="cm-button small" on:click=on_remove>Remove</div>})
        } else {
            None
        }
    };

    return view! {
        <div class="heat-src-controls button-group">
            { open_button }
            <div class="cm-button small" on:click=on_ignore>Ignore</div>
            { remove_button }
        </div>
    };
}

#[component]
pub fn NoDataUI() -> impl IntoView {
    return view! {<div class="container heatmap nodata"></div> };
}
