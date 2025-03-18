use crate::functions::auth::Login;
use leptos::{ev::SubmitEvent, prelude::*};

#[component]
pub fn login_page() -> impl IntoView {
    let email = RwSignal::new("".to_string());
    let password = RwSignal::new("".to_string());

    let login_action = ServerAction::<Login>::new();

    let submit_error = Memo::new(move |_| {
        login_action
            .value()
            .get()
            .and_then(|result| result.err())
            .map_or_else(
                || None,
                |err| match err {
                    ServerFnError::ServerError(s) => Some(s),
                    _ => Some("Something went wrong! Try again later".to_string()),
                },
            )
    });

    let handle_login = move |ev: SubmitEvent| {
        ev.prevent_default();

        let email = email.get_untracked();
        let password = password.get_untracked();

        login_action.dispatch(Login { email, password });
    };

    view! {
      <div class="hero flex-grow">
        <div class="hero-content flex-col lg:flex-row-reverse">
          <div class="text-center lg:text-left max-w-xl">
            <h1 class="text-5xl font-bold">"Login now!"</h1>
            <p class="hidden md:block py-6 text-lg text-base-content leading-relaxed">
              "Welcome back! Log in to start shortening your URLs effortlessly.
              New here? Signing up is fast and simple. Our service turns long URLs into short, 
              shareable links with real-time analytics and custom aliases for secure, 
              efficient link management."
            </p>
          </div>
          <div class="card flex-shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
            <form class="card-body" on:submit=handle_login>
              <label class="label grid">
                <span>"Your Email"</span>
                <input type="email" placeholder="john.doe@domain.com" class="input input-md" autocomplete="email" required bind:value=email />
              </label>
              <label class="label grid">
                <span>"Your Password"</span>
                <input type="password" placeholder="Enter your password" class="input input-md" autocomplete="current-password" required bind:value=password />
              </label>
              <div class="mt-3">
                <a href="/register" class="link link-primary">"Don't have an account?"</a>
              </div>
              <Show when=move || submit_error.get().is_some()>
                <div role="alert" class="alert alert-error alert-soft">
                  <span>{move || submit_error.get()}</span>
                </div>
              </Show>
              <div class="mt-3">
                <button type="submit" class="btn btn-primary btn-block">"Login"</button>
              </div>
            </form>
          </div>
        </div>
      </div>
    }
}
