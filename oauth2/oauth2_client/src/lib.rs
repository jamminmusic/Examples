#![allow(non_snake_case)]
#![allow(unused_imports)]
mod components;
mod router;

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use crate::router::router::AppRouter;
use dioxus::prelude::*;
use fermi::*;
use url::Url;

static AUTH: Atom<AuthStatus> = |_| AuthStatus {
    authenticated: false,
    ..Default::default()
};

#[derive(Debug, Default)]
struct AuthStatus {
    authenticated: bool,
    // Defined by specific login button press
    provider: Option<String>,
    // Defined by specific login button press
    flow: Option<String>,
}

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);

    // same as: cx.render(rsx! { app_router{},})
    // render the router passing down cx. cs is passed automagically.
    cx.render(rsx! {
        AppRouter {}
    })
}
