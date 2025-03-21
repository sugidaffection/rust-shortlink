use crate::{
    components::{
        guard::{AuthGuard, RedirectGuard},
        navbar::Navbar,
    },
    functions::auth::ValidateUserSession,
    pages::{
        auth::{login::LoginPage, register::RegisterPage},
        home::home::HomePage,
    },
};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

#[cfg(any(feature = "csr", feature = "hydrate"))]
use web_sys::window;

#[derive(Clone)]
pub struct AppState {
    pub check_session_action: ServerAction<ValidateUserSession>,
    pub is_signed_in: Memo<bool>,
}

#[component]
pub fn app() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    #[cfg(any(feature = "csr", feature = "hydrate"))]
    let default_theme = "business";

    // Apply the theme when the component mounts
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    if let Some(document) = window().and_then(|win| win.document()) {
        if let Some(html) = document.document_element() {
            html.set_attribute("data-theme", default_theme);
        }
    }

    let check_session_action = ServerAction::<ValidateUserSession>::new();

    let is_signed_in = Memo::new(move |v| match check_session_action.value().get() {
        Some(Ok(_)) => true,
        _ => false,
    });
    let is_checked = RwSignal::new(false);

    Effect::new(move || {
        if !is_checked.get() {
            check_session_action.dispatch(ValidateUserSession {});
            is_checked.set(true);
        }
    });

    provide_context(AppState {
        check_session_action,
        is_signed_in,
    });

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/shortlink.css"/>

        <Title text="Welcome to Leptos"/>

        <Router>
            <main class="bg-base-200 min-h-screen flex flex-col">
                <Navbar signed_in=is_signed_in />
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("/login") view=move || view! {
                        <RedirectGuard>
                            <LoginPage />
                        </RedirectGuard>
                    } />
                    <Route path=StaticSegment("/register") view=move || view! {
                        <RedirectGuard>
                            <RegisterPage />
                        </RedirectGuard>
                    } />
                    <Route path=WildcardSegment("any") view=NotFound />
                </Routes>
            </main>
            <footer class="bg-base-300 text-center py-4 text-gray-500">
                <p>"© 2025 ShortLink. All rights reserved."</p>
            </footer>
        </Router>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <div class="flex flex-col flex-grow justify-center items-center text-center">
        <h1 class="text-9xl font-bold text-primary">404</h1>
        <h2 class="text-3xl font-semibold mt-4">Page Not Found</h2>
        <p class="text-lg text-gray-500 mt-2">
            The page you are looking for does not exist or has been moved.
        </p>
        <a href="/" class="btn btn-primary mt-6">Go Home</a>
        </div>
    }
}
