use leptos::{ev::MouseEvent, html, prelude::*};

#[cfg(any(feature = "csr", feature = "hydrate"))]
use web_sys::window;

use crate::functions::short::CreateShortlink;

#[component]
pub fn home_page() -> impl IntoView {
    let long_url = RwSignal::new("".to_string());
    let input_element: NodeRef<html::Input> = NodeRef::new();

    let short_action = ServerAction::<CreateShortlink>::new();

    let handle_short_url = move |ev: MouseEvent| {
        ev.prevent_default();

        if long_url.get_untracked().is_empty() {
            if let Some(Some(input)) = input_element.try_get_untracked() {
                let _ = input.focus();
            }
            return;
        }

        short_action.dispatch(CreateShortlink {
            url: long_url.get_untracked(),
        });
    };

    view! {
        <div class="hero flex-grow">
            <div class="hero-content text-center">
                <div class="max-w-xl w-full">
                <h1 class="text-5xl font-bold text-primary">"Shorten Your Links Instantly"</h1>
                <p class="py-6 text-lg text-base-content">
                    "Enter your long URL below and get a short, trackable link in seconds!"
                </p>

                <div class="flex flex-col md:flex-row items-center gap-4 w-full">
                    <input
                    node_ref=input_element
                    type="text"
                    placeholder="Enter your URL..."
                    class="input w-full"
                    bind:value=long_url
                    required
                    />
                    <button class="btn btn-primary w-full md:w-auto" on:click=handle_short_url>"Shorten URL"</button>
                </div>

                <Suspense>
                    <Show when=move || short_action.value().get().and_then(|v| v.ok()).is_some()>
                        <DisplayResult result=short_action.value().get().and_then(|v| v.ok()) />
                    </Show>
                </Suspense>
                </div>
            </div>
        </div>

    }
}

#[component]
fn display_result(result: Option<String>) -> impl IntoView {
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    let result_for_copy = result.clone();
    let handle_copy = move |ev: MouseEvent| {
        ev.prevent_default();

        #[cfg(any(feature = "csr", feature = "hydrate"))]
        if let Some(navigator) = window()
            .and_then(|win| Some(win.navigator()))
            .and_then(|nav| Some(nav.clipboard()))
        {
            if let Some(url) = result_for_copy.clone() {
                let _ = navigator.write_text(&url);
            }
        }
    };
    view! {
        <div class="mt-4">
            <div class="flex items-center gap-4 bg-base-100 p-3 rounded-lg">
            <input
                type="text"
                class="input w-full"
                readonly
                value={result}
            />
            <button class="btn btn-accent" on:click=handle_copy>"Copy"</button>
            </div>
        </div>
    }
}
