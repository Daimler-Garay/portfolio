use leptos::prelude::*;
use std::borrow::Cow;
use tailwind_fuse::*;

use crate::components::{Text, TextSize};

#[component]
pub fn NavBar(
    children: Children,
    #[prop(into, optional)] class: Option<Cow<'static, str>>,
) -> impl IntoView {
    view! {
        <nav class=tw_merge!(
            "flex gap-x-4 py-3 px-2 items-center",
            class
        )>{children()}</nav>
    }
}

#[component]
pub fn NavLink(
    children: Children,
    #[prop(into, optional)] active: bool,
    #[prop(into, optional)] to: Option<String>,
    #[prop(into, optional)] class: Option<Cow<'static, str>>,
) -> impl IntoView {
    // Just color change, no underline, no background
    let base_class = "transition-colors text-slate-600 dark:text-slate-300 hover:text-slate-900 dark:hover:text-white px-2 py-1";
    let active_class = if active {
        "text-black dark:text-white font-semibold"
    } else {
        ""
    };

    view! {
        <a href=to class=tw_merge!(base_class, active_class, class)>
            <Text size=TextSize::Md>{children()}</Text>
        </a>
    }
}
