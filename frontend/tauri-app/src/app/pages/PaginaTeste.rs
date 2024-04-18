use yew::function_component;
use yew::html;

#[function_component(PaginaTeste)]
pub fn Pagina_Teste() -> Html {
    html! {
        <div>
            <h1>{"This is the second page"}</h1>
            <Link<Route> to={Route::Home}>{"Go back to Home"}</Link<Route>>
        </div>
    }
}
