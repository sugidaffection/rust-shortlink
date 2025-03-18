use crate::app::AppState;
use leptos::prelude::*;
use leptos_router::components::Redirect;

#[component]
pub fn auth_guard(children: ChildrenFn) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let is_signed_in = app_state.is_signed_in;

    view! {
        <Suspense
            fallback= move || view!{ <div class="flex-grow">
                <span class="loading loading-spinner loading-xl"></span>
            </div> }
        >
                <Show
                    when=move || is_signed_in.get()
                    fallback=move || view! { <Redirect path="/login" /> }
                >
                    {children()}
            </Show>
        </Suspense>
    }
}

#[component]
pub fn redirect_guard(children: ChildrenFn) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let is_signed_in = app_state.is_signed_in;

    view! {
        <Suspense
            fallback= move || view!{ <div class="flex-grow">
                <span class="loading loading-spinner loading-xl"></span>
            </div> }
        >
                <Show
                    when=move || !is_signed_in.get()
                    fallback=move || view! { <Redirect path="/" /> }
                >
                    {children()}
            </Show>
        </Suspense>
    }
}
