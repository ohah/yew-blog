use serde::{Serialize, Deserialize};
use yew::{ function_component, html, use_effect_with_deps, use_effect };
use yew_router::prelude::{use_location, Location, use_history, History};
use yewdux::prelude::{use_store, Dispatch};

use crate::{store::github_auth::{GithubAuth, GithubUser}};
use crate::router::root::RootRoute;

#[derive(Serialize, Deserialize)]
struct AuthQuery {
  code:String
}


#[function_component(Auth)]
pub fn auth() -> Html {
  let location = use_location().unwrap();
  let code = location.query::<AuthQuery>().unwrap().code;
  let dispatch = Dispatch::<GithubAuth>::new();
  let ( user_state, _) = use_store::<GithubUser>();
  {
    let dispatch = dispatch.clone();
    use_effect_with_deps(move |_| {
      dispatch.clone().get().get_user(code);
      || ()
    }, ());
  }
  {
    let user_state = user_state.clone();
    let history = use_history().unwrap();
    use_effect(move || {
      if user_state.access_token != "".to_string() && user_state.name != "".to_string() {
        history.push(RootRoute::Home);
      }
      || ()
    });
  }
	html! {
    <>
      // {format!("{:?}", href)}
      // {format!("{:?}", location.query)}
      // {format!("{:?}", location.route())}
    </>
  }
}