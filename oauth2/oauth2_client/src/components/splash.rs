#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::Link;
use fermi::*;

#[inline_props]
pub fn SplashPage(cx: Scope) -> Element {
    cx.render(rsx! {"Splash Page"})
}
