#![allow(non_snake_case)]
use crate::AUTH;
use dioxus::prelude::*;
use dioxus_router::{use_route, use_router, Link, Redirect, Route, Router};
use fermi::*;
use web_sys::window;

#[inline_props]
pub fn Login<'login>(
    cx: Scope<'login>,
    on_log_in: Option<EventHandler<'login>>,
    on_log_out: Option<EventHandler<'login>>,
) -> Element<'login> {
    let auth = use_read(cx, AUTH);

    cx.render(rsx! {
        button {
            "id": "spotify-login-button",
            "data-provider": "spotify",
            "data-authflow": "pkce",
            // depending on the value of `is_logged_in`, we will call a different event handler
            onclick: move |_| if auth.authenticated {
                match on_log_out {
                    Some(callback) => callback.call(()),
                    None => window().unwrap().location().assign("http://localhost:8080").unwrap()
                  }
            }
            else{
                match on_log_in {
                    Some(callback) => callback.call(()),
                    None => window().unwrap().location().assign("http://localhost:8080").unwrap()
                  }
            },
            if auth.authenticated {
                // if we are logged in, the button should say "Log Out"
                "Log Out"
            } else {
                // if we are not logged in, the button should say "Log In"
                "Log In"
            }
        }
    })
}

#[inline_props]
pub fn LoginCallback<'callback>(
    cx: Scope<'callback>,
    on_page_load: Option<EventHandler<'callback>>,
) -> Element<'callback> {
    cx.render(rsx! {
        body {
            onload: move |_| match on_page_load {
                Some(callback) => {
                    let auth_code = window().unwrap().location().search().unwrap();

                    callback.call(())
                }
                None => log::info!("failllleeeddd"),
            },
        }
    })
}
