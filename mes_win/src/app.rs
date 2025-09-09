use crate::components::login::Login;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="app">
            <Login />
        </main>
    }
}
