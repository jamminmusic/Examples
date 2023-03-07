#![allow(non_snake_case)]
use crate::components::auth::Login;
use crate::router::router::BASE_URL;
use crate::AUTH;
use dioxus::prelude::*;
use dioxus_router::{Link, Redirect};
use fermi::*;
use getrandom::getrandom;
use reqwest;
use web_sys::window;

#[inline_props]
pub fn HeaderNavbar(cx: Scope) -> Element {
    let auth = use_read(cx, AUTH);

    cx.render(rsx! {
        ul {
            if auth.authenticated {
                rsx!{
                    Link { to: "/home", "Logo Goes Here"},
                    br {},
                    Login{ on_log_out: |_| println!("logged out")}
                }
            } else {
                rsx!{
                    Link { to: "/", "Logo Goes Here"},
                    br {},
                    Login{
                        on_log_in: |_| {
                            cx.spawn({
                                async move {
                                    // let document = window().expect("failed to get window").document().expect("failed to get document");
                                    // let element = document.get_element_by_id("spotify-login-button").expect("failed to get element");
                                    // let provider = element.get_attribute("provider").expect("failed to get provider");
                                    // let flow = element.get_attribute("flow").expect("failed to get flow");
                                    // let mut random_bytes = [0u8; 8];
                                    // getrandom(&mut random_bytes).expect("Failed to generate random bytes");
                                    // let id = u64::from_be_bytes(random_bytes);
                                    // let res =  reqwest::get(format!("{}/login?provider={}&flow={}&id={}", BASE_URL, provider, flow, id.to_string()))
                                    // .await.unwrap().text().await.unwrap();
                                    let res =  reqwest::get(format!("http://localhost:8000/login?provider=spotify&flow=pkce&id=1234")).await.unwrap().text().await.unwrap();
                                    log::info!("{:?}", res);
                                    window().unwrap().location().assign(res.as_str()).expect("error redirecting")
                                }
                            })
                        }
                    }
                }
            }
        }
    })
}
