use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore::rt::Event;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct RequestArgs<'a> {
    method: &'a str,
    url: &'a str,
}

#[component]
pub fn App<G: Html>(cx: Scope) -> View<G> {
    let name = create_signal(cx, String::new());
    let method = create_signal(cx, String::new());
    let new_request = create_signal(cx, String::new());

    let do_request = move |e: Event| {
        e.prevent_default();
        spawn_local_scoped(cx, async move {
            let get_method = || {
                if method.get().is_empty() {
                    "GET".to_string()
                } else {
                    method.get().as_ref().clone()
                }
            };
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = invoke(
                "request",
                to_value(&RequestArgs {
                    method: &get_method(),
                    url: &name.get(),
                })
                .unwrap(),
            )
            .await;

            log(&new_msg.as_string().unwrap());

            new_request.set(new_msg.as_string().unwrap());
        })
    };

    view! { cx,
        main(class="container") {
            // div(class="row") {
            //     a(href="https://tauri.app",target="_blank") {
            //         img(src="public/tauri.svg",class="logo tauri",alt="Tauri logo")
            //     }
            //     a(href="https://sycamore-rs.netlify.app",target="_blank") {
            //         img(src="public/sycamore.svg",class="logo sycamore",alt="Sycamore logo")
            //     }
            // }
            // p {
            //     "Click on the Tauri and Sycamore logos to learn more."
            // }
            // p {
            //     "Recommended IDE setup: "
            //     a(href="https://code.visualstudio.com/",target="_blank") {
            //         "VS Code"
            //     }
            //     " + "
            //     a(href="https://github.com/tauri-apps/tauri-vscode",target="_blank") {
            //         "Tauri"
            //     }
            //     " + "
            //     a(href="https://github.com/rust-lang/rust-analyzer",target="_blank") {
            //         "rust-analyzer"
            //     }
            // }
            form(class="row",on:submit=do_request) {
                select(id="method-input",bind:value=method) {
                    option(value="GET"){"GET"}
                    option(value="POST"){"POST"}
                    option(value="DELETE"){"DELETE"}
                    option(value="PUT"){"PUT"}
                    option(value="PATCH"){"PATCH"}
                }
                input(id="greet-input",bind:value=name,placeholder="Enter a name...")
                button(type="submit") {
                    "Send"
                }
            }
            p {
                b {
                    (new_request.get())
                }
            }
        }
    }
}
