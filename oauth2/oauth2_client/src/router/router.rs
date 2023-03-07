#![allow(non_snake_case)]
use crate::components::{
    auth::LoginCallback, footer::FooterNavbar, header::HeaderNavbar, home::Home, splash::SplashPage,
};
use crate::AUTH;
use dioxus::prelude::*;
use dioxus_router::{Link, Route, Router};
use fermi::*;
use web_sys::window;

pub const BASE_URL: &str = "localhost:8000";

pub fn AppRouter(cx: Scope) -> Element {
    let auth = use_read(cx, AUTH);

    cx.render(rsx! {
            Router {
            HeaderNavbar {}
            Route { to: "/", SplashPage{} }
            if auth.authenticated {
                rsx!{
                    Route { to: "/home", Home{} }
                    Route { to: "/callback",
                        LoginCallback{
                            on_page_load: |_| {
                                // take auth code out of location header
                                // subscribe to nats channel of id sent with auth request.
                                // publish auth code and state with nats publish
                                // unsubscribe
                            }
                        }
                    }
                    Route { to: "", "Err 404 Route Not Found" }
                    FooterNavbar {}
                }
            }
        }
    })
}
