use leptos::prelude::*;

#[component]
pub fn Header1(text: String) -> impl IntoView {
    view! { <h1 class="text-2xl font-bold mb-5">{text}</h1> }
}

#[component]
pub fn Header2(text: String) -> impl IntoView {
    view! { <h2 class="text-xl font-bold mb-5">{text}</h2> }
}

#[component]
pub fn CardTitle(text: String) -> impl IntoView {
    view! { <h2 class="card-title">{text}</h2> }
}
