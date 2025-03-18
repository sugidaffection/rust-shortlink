use leptos::prelude::*;

#[component]
pub fn navbar(signed_in: Memo<bool>) -> impl IntoView {
    view! {
        <div class="navbar bg-base-300 shadow-sm px-4 sticky top-0 z-10">
        <div class="flex-1">
            <a href="/" class="btn btn-ghost text-xl">"ShortLink"</a>
        </div>
        <div class="flex gap-2">
            <ul class="menu menu-horizontal px-1">
                <li><a href="/">"Home"</a></li>
                <li><a>"Features"</a></li>
                <li><a>"Pricing"</a></li>
                <li><a>"Donate"</a></li>
            </ul>
        </div>
        <Show when=move || !signed_in.get()
            fallback=move || view! { <ProfileDropdown /> }
        >
        <div class="flex space-x-2">
            <a href="/login" class="btn btn-primary">"Sign In"</a>
            <a href="/register" class="btn btn-secondary">"Sign Up"</a>
        </div>
        </Show>

        </div>
    }
}

#[component]
fn profile_dropdown() -> impl IntoView {
    view! {
        <div class="dropdown dropdown-end">
          <div tabindex="0" role="button" class="btn btn-ghost btn-circle avatar">
            <div class="w-10 rounded-full">
              <img
                alt="Tailwind CSS Navbar component"
                src="https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp" />
            </div>
          </div>
          <ul
            tabindex="0"
            class="menu menu-sm dropdown-content bg-base-100 rounded-box z-1 mt-3 w-52 p-2 shadow">
            <li>
              <a class="justify-between">
                "Profile"
                <span class="badge">"New"</span>
              </a>
            </li>
            <li><a>"My Links"</a></li>
            <li><a>"Settings"</a></li>
            <li><a>"Logout"</a></li>
          </ul>
        </div>
    }
}
