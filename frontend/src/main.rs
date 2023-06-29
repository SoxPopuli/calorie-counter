use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeHello, catch)]
    pub async fn hello(name: String) -> Result<JsValue, JsValue>;
}

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,

    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
fn app() -> Html {

    html! {
        <BrowserRouter>
            <div style={"display: flex; flex-direction: row; gap: 1em;"}>
                <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
            </div>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!{ <h1>{ "Home" }</h1> },
        Route::NotFound => html!{ <h1>{ "404" }</h1> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

