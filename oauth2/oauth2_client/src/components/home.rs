#![allow(non_snake_case)]
use crate::AUTH;
use dioxus::prelude::*;
use dioxus_router::Link;
use fermi::*;

#[inline_props]
pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {"Home Page"})
}
