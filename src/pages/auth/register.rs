use leptos::{ev::SubmitEvent, prelude::*};

use crate::functions::auth::Register;

#[component]
pub fn register_page() -> impl IntoView {
    let email = RwSignal::new("".to_string());
    let password = RwSignal::new("".to_string());
    let username = RwSignal::new("".to_string());

    let register_action = ServerAction::<Register>::new();

    let submit_error = Memo::new(move |_| {
        register_action
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

    let handle_register = move |ev: SubmitEvent| {
        ev.prevent_default();

        let username = username.get_untracked();
        let email = email.get_untracked();
        let password = password.get_untracked();

        register_action.dispatch(Register {
            username,
            email,
            password,
        });
    };

    view! {
      <div class="hero flex-grow">
        <div class="hero-content flex-col lg:flex-row-reverse">
          <div class="text-center lg:text-left max-w-xl">
            <h1 class="text-5xl font-bold">"Register now!"</h1>
            <p class="hidden md:block py-6 text-lg text-base-content leading-relaxed">
              "Sign up today and start creating shorter, more manageable links for your URLs.
              Our URL shortener service helps you share links efficiently across any platform,
              making your online sharing experience smoother and more professional."
            </p>
          </div>
          <div class="card flex-shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
            <form class="card-body" on:submit=handle_register>
            <label class="label grid">
                <span>"Your Username"</span>
                <input type="username" placeholder="johndoe" class="input input-md" autocomplete="username" required bind:value=username />
              </label>
              <label class="label grid">
                <span>"Your Email"</span>
                <input type="email" placeholder="john.doe@domain.com" class="input input-md" autocomplete="email" required bind:value=email />
              </label>
              <label class="label grid">
                <span>"Your Password"</span>
                <input type="password" placeholder="Enter your password" class="input input-md" autocomplete="new-password" required bind:value=password />
              </label>
              <div class="mt-3">
                <a href="/login" class="link link-primary">"Already have an account?"</a>
              </div>
              <Show when=move || submit_error.get().is_some()>
                <div role="alert" class="alert alert-error alert-soft">
                  <span>{move || submit_error.get()}</span>
                </div>
              </Show>
              <div class="mt-3">
                <button type="submit" class="btn btn-primary btn-block">"Register"</button>
              </div>
            </form>
          </div>
        </div>
      </div>
    }
}
