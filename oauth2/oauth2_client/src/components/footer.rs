#![allow(non_snake_case)]
use crate::AUTH;
use dioxus::prelude::*;
use dioxus_router::Link;
use fermi::*;

#[inline_props]
pub fn FooterNavbar(cx: Scope) -> Element {
    cx.render(rsx! {
            ul {
            Link { to: "/", "Home"}
            br {}
            Link { to: "/profile", "Profile Page"}
        }
    })
}
