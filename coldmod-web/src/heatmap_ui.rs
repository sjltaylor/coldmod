use crate::{coldmod_d::Sender, controls_ui::ControlsUI, IgnoreList, RemoveList};
use coldmod_msg::{
    proto::{mod_command::Command, IgnoreCommand, ModCommand, RemoveCommand},
    web::{HeatSrc, Msg},
};
use leptos::*;

#[component]
pub fn HeatMapUI(cx: Scope) -> impl IntoView {
    let heat_srcs_memo = use_context::<Memo<Option<Vec<HeatSrc>>>>(cx).unwrap();

    return view! {cx,
        <Show
            when=move || heat_srcs_memo.get().is_some()
                fallback=|cx| view! { cx, <NoDataUI /> }>
                <div class="container heatmap">
                    <ControlsUI />
                    <ul class="container heatmap data">
                        <For
                            each=move || heat_srcs_memo.get().unwrap()
                            key=|u| format!("{}-{}", u.trace_src.key, u.trace_count)
                            view=move |cx, s| view! {cx, <HeatSourceUI heat_src=s /> } />
                    </ul>
            </div>
        </Show>
    };
}

#[component]
pub fn HeatSourceUI(cx: Scope, heat_src: HeatSrc) -> impl IntoView {
    let mod_client_connected = use_context::<ReadSignal<bool>>(cx).unwrap();
    let ignore_list = use_context::<ReadSignal<IgnoreList>>(cx).unwrap();
    let remove_list = use_context::<RemoveList>(cx).unwrap();
    let sender = use_context::<Sender>(cx).unwrap();
    let (command, w_command) = create_signal::<Option<Command>>(cx, None);

    let key = heat_src.trace_src.key.clone();
    let is_ignored = move || ignore_list.get().contains(&key);
    let _key = heat_src.trace_src.key.clone();

    let ignore_classname = move || {
        let mut buffer: Vec<String> = vec!["container heat-src".to_string()];

        if is_ignored() {
            buffer.push("ignore".to_string());
        }

        buffer.join(" ")
    };

    let key = heat_src.trace_src.key.clone();

    create_effect(cx, move |_| match command.get() {
        Some(command) => {
            let msg = Msg::RouteModCommand(ModCommand {
                command: Some(command),
            });
            sender.send(msg);
        }
        _ => {}
    });

    let trace_src = heat_src.trace_src;

    let refs_view = move || {
        if mod_client_connected.get() {
            Some(view! {cx,
                <div class="heat-src-trace-count">REFS:123</div>
            })
        } else {
            None
        }
    };

    let controls_view = move || {
        let key = key.clone();
        if mod_client_connected.get() && !ignore_list.get().contains(&key) {
            let remove = remove_list.get().contains(&key);
            Some(view! {cx,
                <HeatSourceControlsUI key w_command remove />
            })
        } else {
            None
        }
    };

    return view! {cx,
        <li class="heat-src-row">
            <div class={ignore_classname}>
                <div class="heat-src-trace-count">TRACES:{heat_src.trace_count}</div>
                { refs_view }
                <div class="heat-src-fqn">{trace_src.key}</div>
                { controls_view }
            </div>
        </li>
    };
}

#[component]
pub fn HeatSourceControlsUI(
    cx: Scope,
    key: String,
    w_command: WriteSignal<Option<Command>>,
    remove: bool,
) -> impl IntoView {
    let key_clone = key.clone();

    let on_ignore = move |_| {
        w_command.set(Some(Command::Ignore(IgnoreCommand {
            key: key_clone.clone(),
        })));
    };

    let remove_button = move || {
        if remove {
            let key_clone = key.clone();
            let on_remove = move |_| {
                w_command.set(Some(Command::Remove(RemoveCommand {
                    key: key_clone.clone(),
                })));
            };

            Some(view! {cx, <div class="toggle-button" on:click=on_remove>Remove</div>})
        } else {
            None
        }
    };

    return view! {cx,
        <div class="heat-src-controls button-group">
            <div class="toggle-button" on:click=on_ignore>Ignore</div>
            { remove_button }
        </div>
    };
}

#[component]
pub fn NoDataUI(cx: Scope) -> impl IntoView {
    return view! {cx, <div class="container heatmap nodata"></div> };
}
