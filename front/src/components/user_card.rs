use crate::components::Button;
use axum::Json;
use dioxus::prelude::*;
use shared::models::{user::User, ButtonType};

#[inline_props]
pub fn UserCard(
    cx: Scope,
    // pub fn UserCard<'a>(
    //     cx: Scope<'a>,
    // user: &'a User,
    // on_edit: EventHandler<'a, MouseEvent>,
    // on_delete: EventHandler<'a, MouseEvent>,
) -> Element {
    let future = use_future(cx, (), |_| async move {
        reqwest::Client::new()
            .get("http://127.0.0.1:8000/health/check_user/")
            .send()
            .await
    });
    cx.render(match future.value() {
        // Some(Ok(Json(code))) => rsx! {
        //     div {
        //         format!("{:#?}", code)
        //     }
        // },
        Some(Ok(other)) => rsx! {
            div {
                format!("Unrecognized body: {:#?}", other)
            }
        },
        Some(Err(e)) => rsx! {
            div {
                e.to_string()
            }
        },
        None => rsx! {
            div {
                "No user found."
            }
        },
    })
}
