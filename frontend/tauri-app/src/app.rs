use yew::prelude::*;
use yew_router::prelude::*;

// Definição dos módulos onde estão localizados seus componentes
mod pages {
    pub mod HomeComponent;
    pub mod PaginaTeste;
}

// Importação dos componentes a partir dos módulos definidos
use pages::HomeComponent::HomeComponent;
use pages::PaginaTeste::PaginaTeste;

// Definição das rotas usando enum com anotações para o yew_router
#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/pagina-teste")]
    TestPage,
}

// Componente principal que inclui o roteador
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

// Função switch para determinar qual componente renderizar com base na rota
fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <HomeComponent /> },
        AppRoute::TestPage => html! { <PaginaTeste /> },
    }
}
