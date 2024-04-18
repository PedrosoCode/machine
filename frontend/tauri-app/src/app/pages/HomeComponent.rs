use yew::function_component;
use yew::html;

#[function_component(HomeComponent)]
pub fn Home_Component() -> Html {
    html! {
        <div>
            <h1>{"This is the second page"}</h1>
            <Link<Route> to={Route::Home}>{"Go back to Home"}</Link<Route>>
        </div>
    }
}
