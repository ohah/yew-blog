use web_sys::{Event, MouseEvent, FocusEvent, HtmlInputElement};
use yew::{function_component, html, classes, Callback, use_state, use_effect_with_deps,  use_node_ref};
use yew_router::{components::Link, prelude::{use_history, History, use_location, Location, use_route}, history};
use crate::{router::root::{RootRoute, WriteRoute}, store::{github_auth::{GithubUser, GithubAuth}, blog::{Blog}}};
use crate::components::{dropdown::{Dropdown}, transition::Transition};
use crate::store::toast::ToastStatus;
use yewdux::prelude::{use_store, Dispatch};

#[path = "../pages/blog/mod.rs"]
mod blog;
use blog::list::{SearchQuery};

use crate::components::modal;
use modal::Modal;
use crate::components::dialog::{Dialog, DialogType};

use crate::components::github_login;
use github_login::GitHubLogin;

use crate::store::theme::{ Theme, ThemeConfig };

#[function_component(Header)]
pub fn header() -> Html {
	const HOVER_CSS: &str =
  "dark:hover:text-slate-200 group hover:text-slate-900 duration-200 ease-in-out cursor-pointer";
  let location = use_location().unwrap();
  let route = use_route::<RootRoute>().unwrap();
  let history = use_history().unwrap();
	let (_, dispatch) = use_store::<Theme>();
  let query = use_state(||String::from(""));
  let search_input = use_node_ref();
  {
    let query_string = {
      let string = location.query::<SearchQuery>();
      match string {
        Ok(string) => {
          string.query
        },
        Err(err) => {
          String::from("")
        }
      }
    };
    let query = query.clone();
    use_effect_with_deps(move |_| {
      if !query_string.is_empty() {
        query.set(query_string);
      }
      || ()
    }, ());
  }
	let (user_state, _) = use_store::<GithubUser>();
  let is_login = use_state(|| false);
  let dropdown = use_state(||false);
  let _dropdown = use_state(||false);
  // let query = 
  let toggle_login_modal = {
    let is_login = is_login.clone();
    Callback::from(move |e:MouseEvent| {
      e.prevent_default();      
      is_login.set(!*is_login);
    })
  };

  let write = {
    let dropdown = dropdown.clone();
    let history = use_history().unwrap();
    Callback::from(move |e:MouseEvent| {
      e.prevent_default();
      dropdown.set(false);
      history.push(WriteRoute::Write);
    })
  };

  let dropdown_close = {
    let dropdown = dropdown.clone();
    Callback::from(move |e:MouseEvent| {
      e.prevent_default();
      dropdown.set(false);
    })
  };

  let logout = {
    let dispatch = Dispatch::<GithubAuth>::new();
    Callback::from(move |e:MouseEvent| {
      e.prevent_default();
      dispatch.get().logout();
    })
  };

  let alert = {
    Callback::from(move |e:MouseEvent| {
      e.prevent_default();
      Blog::toast_message("dasd", ToastStatus::SUCCESS, None);
    })
  };

  {		
    let query = query.clone();
    let search_input = search_input.clone();
    let location = location.clone();
		use_effect_with_deps(move|(history, search_input)| {
      let history = history.clone();
      let search_input = search_input.clone();
      let listener = history.clone().listen(move|| {
        // let location = history.clone().location();
        let query_string = {
          let string = location.query::<SearchQuery>();
          match string {
            Ok(string) => {
              string.query
            },
            Err(err) => {
              String::from("")
            }
          }
        };
        let input = search_input.cast::<HtmlInputElement>();
        query.set(query_string.clone());
        match input {
          Some(input) => {
            input.set_value(query_string.clone().as_str());
          },
          None => {},
        }
      });
			move || { std::mem::drop(listener);}
		}, (history.clone(), search_input.clone()));
	}
  

  let onsubmit = {
    let history = history.clone();
    let route = route.clone();
    let search_input = search_input.clone();
    let query = query.clone();
    Callback::from(move |e:FocusEvent|{  
      e.prevent_default();
      let input = search_input.cast::<HtmlInputElement>().expect("render을 가져오지 못함");
			let value = input.clone().value();
      if value.clone() == (*query).clone() {
        Blog::toast_message("동일한 검색어를 입력할 수 없습니다.", ToastStatus::WARNING, None);
      } else if !value.clone().is_empty() { 
        history.push_with_query(RootRoute::List { page: 1.to_string() }, SearchQuery {query: value.clone()});
        query.set(value.clone());
      } else {
        Blog::toast_message("검색어를 입력해주세요", ToastStatus::DANGER, None);
      }
    })
  };
  DialogType::Panel {
    a : 1, b : 3
  };
	html! {
    <header class="dark:bg-gradient-to-r from-[#1a0540] to-[#200a51] bg-white h-[60px] flex items-center border-b shadow-b dark:border-slate-500/60 sticky top-0 left-0 z-[9999]">
      <div class="flex justify-between align-center w-full lg:max-w-screen-lg m-auto items-center">
        <div class="px-2 group">
          <Link<RootRoute> to={RootRoute::Home}>
            <p class="yg-jalnan sm:text-xl text-sm group-hover:scale-110 block ease-in-out duration-200 group-hover:text-slate-700 dark:group-hover:text-slate-100 ">{ "Ohah" } </p>
            <p class="text-[3px] sm:block hidden group-hover:text-slate-700 dark:group-hover:text-slate-100"> {"개발자의 주절주절"} </p>
          </Link<RootRoute>>
        </div>
        <div class="flex items-center pr-3 S-CoreDream-4Regular sm:text-base text-sm"> 
          <ul class="flex items-center divide-x-2 dark:divide-slate-500/60 font-bold">            
            <li class={classes!("px-3", HOVER_CSS)}>
              <span class="hover:scale-110 block ease-in-out duration-200"><Link<RootRoute> to={RootRoute::Profile}>{ "프로필" }</Link<RootRoute>></span>
            </li>
            <li class={classes!("px-3", HOVER_CSS)}>
              <span class="hover:scale-110 block ease-in-out duration-200"><Link<RootRoute> to={RootRoute::List { page : 1.to_string()} }>{ "목록" }</Link<RootRoute>></span>
            </li>
          </ul>
          <div class="flex items-center justify-center">
            if user_state.clone().name == "" {
            <button
              class="sm:text-2xl text-sm group px-2"
              onclick={toggle_login_modal.clone()}
            >
              <i class="hover:text-slate-700 dark:hover:text-slate-200 ri-login-box-line"></i>
            </button>
            } else {
              <div class="relative inline-flex justify-center items-center text-left">
                <Dropdown
                  button={
                    let user_state = user_state.clone();
                    let html = html!{
                      <button class="sm:text-2xl text-sm group w-6 h-6">
                        <img 
                          class="w-full h-full rounded-full"
                          src={format!("{}", user_state.avatar_url)} 
                        />
                      </button>
                    };
                    html
                  }
                >
                  <Transition 
                    enter="transition ease-in-out duration-300 transform"
                    enter_from="scale-y-0"
                    enter_to="scale-1"
                    leave="transition ease-in-out duration-300 transform"
                    leave_from="scale-y-1"
                    leave_to="scale-y-0"
                    class="origin-top-right absolute top-7 right-0 mt-2 w-36 rounded-md shadow-lg bg-white dark:bg-slate-800 dark:shadow-none shadow dark:text-slate-300 text-gray-700 ring-1 ring-black ring-opacity-5 focus:outline-none"
                  >
                      <div class="py-1 divide-y-2 dark:divide-slate-700">
                        <button 
                          class="flex w-full dark:hover:bg-slate-900 hover:bg-gray-200 text-left px-4 py-2 text-sm space-x-2" tabindex="-1"
                          onclick={write}
                        >
                          <i class="ri-edit-box-line"></i>
                          <span>{"글 쓰기"}</span>
                        </button>
                        <div
                          onclick={dropdown_close}
                        >
                          <Link<RootRoute> to={RootRoute::MyPageList { page : 1} }>
                            <button class="flex w-full dark:hover:bg-slate-900 hover:bg-gray-200 text-left px-4 py-2 text-sm space-x-2" tabindex="-1">
                              <i class="ri-list-unordered"></i>
                              <span>{"내가 쓴 글"}</span>
                            </button>
                          </Link<RootRoute>>
                        </div>
                        <button class="flex w-full dark:hover:bg-slate-900 hover:bg-gray-200 text-left px-4 py-2 text-sm space-x-2" tabindex="-1">
                          <i class="ri-message-2-line"></i>
                          <span>{"내가 쓴 댓글"}</span>
                        </button>
                        <button 
                          class="flex w-full dark:hover:bg-slate-900 hover:bg-gray-200 text-left px-4 py-2 text-sm space-x-2" tabindex="-1"
                          onclick={logout}
                        >
                          <i class="ri-logout-box-r-line"></i>
                          <span>{"로그아웃"}</span>
                        </button>
                      </div>
                  </Transition>
                </Dropdown>
              </div>
            }
            <button
              class="text-2xl group px-2"
            >
              <a href="https://github.com/ohah/" target="_blank"><i class="group-hover:text-slate-700 group-dark:hover:text-slate-200 ri-github-fill"></i></a>
            </button>
            <Dropdown
              button={
                let html = html!{
                  <button class="text-2xl group w-6 h-6 hover:text-slate-700 dark:hover:text-slate-200" >
                    <i class="ri-search-line"></i>
                  </button>
                };
                html
              }
            >
              <Transition 
                enter="transition ease-in-out duration-300 transform origin-top-right"
                enter_from="scale-y-0"
                enter_to="scale-1"
                leave="transition ease-in-out duration-300 transform"
                leave_from="scale-y-1"
                leave_to="scale-y-0"
                class="origin-top-right absolute top-7 right-0 mt-2 rounded-md shadow-lg bg-white dark:bg-slate-800 dark:shadow-none shadow dark:text-slate-300 text-gray-700 ring-1 ring-black ring-opacity-5 focus:outline-none"
              >                
                <form 
                  class="flex relative w-full outline-0 focus:ring-0 focus:outline-0 ring-0"
                  onsubmit={onsubmit.clone()}
                >
                  <input 
                    ref={search_input}
                    type="text"
                    class="flex flex-grow font-sans block text-sm w-full py-2 pl-3 pr-10 ring-1 ring-slate-900/10 text-slate-500 rounded-lg dark:bg-slate-800 dark:ring-0 dark:highlight-white/5 dark:text-slate-400 focus:outline-none min-w-[13rem] focus:ring-none focus:ring-0 border dark:border-slate-600 border-gray-500"
                    value={(*query).clone()}
                    placeholder="검색어를 입력하세요"
                  />
                  <button class="inline-flex items-center justify-center w-full w-10 absolute top-0 right-0 h-full outline-0">
                    <i class="ri-search-line"></i>
                  </button>
                </form>
              </Transition>
            </Dropdown>
            // <button
            //   class="text-2xl group px-2"
            //   onclick={alert}
            // >
            //   {"경고!"}
            // </button>
            <button 
              class="text-2xl group px-2"
              onclick={dispatch.apply_callback(|_| ThemeConfig::Toggle)}>
              if dispatch.get().color == "dark".to_string() {
                <i class="hover:text-slate-700 dark:hover:text-slate-200 ri-sun-line ease-in-out transition duration-500"></i>
              } else {
                <i class="hover:text-slate-700 dark:hover:text-slate-200 ri-moon-line ease-in-out transition duration-500"></i>
              }
            </button>
          </div>
        </div>        
      </div>
      if *is_login == true {
        <Modal
          is_close={toggle_login_modal.clone()}
        >
          <Transition 
          show={*is_login}
          enter="transition ease-in-out duration-300 transform"
          enter_from="scale-y-0"
          enter_to="scale-1"
          leave="transition ease-in-out duration-300 transform"
          leave_from="scale-y-1"
          leave_to="scale-y-0"
          >
            <GitHubLogin />
          </Transition>
        </Modal>
      }
    </header>
  }
}