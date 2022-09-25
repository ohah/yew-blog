use web_sys::MouseEvent;
use yew::{ function_component, html, classes, Callback, use_state, Children};
use yew_router::{components::Link, prelude::{use_history, History}};
use crate::{router::root::{RootRoute, WriteRoute}, store::{github_auth::{GithubUser, GithubAuth}, blog::Blog}, components::dropdown::{DropdownChildren, Button, Parent}};
use crate::components::dropdown::Dropdown;
use crate::components::transition::Transition;
use crate::store::toast::ToastStatus;
use yewdux::prelude::{use_store, Dispatch};

use crate::components::modal;
use modal::Modal;

use crate::components::github_login;
use github_login::GitHubLogin;

use crate::store::theme::{ Theme, ThemeConfig };

#[function_component(Header)]
pub fn header() -> Html {
	const HOVER_CSS: &str =
		"dark:hover:text-slate-200 group hover:text-slate-900 duration-200 ease-in-out cursor-pointer";
	let (_, dispatch) = use_store::<Theme>();
	let (user_state, _) = use_store::<GithubUser>();
  let is_login = use_state(|| false);
  let dropdown = use_state(||false);
  let _dropdown = use_state(||false);

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

  let tshow = use_state(||false);
	html! {
    <header class="dark:bg-gradient-to-r from-[#1a0540] to-[#200a51] bg-white h-[60px] flex items-center border-b shadow-b dark:border-slate-500/60 sticky top-0 left-0 z-[9999]">
      <div class="flex justify-between align-center w-full lg:max-w-screen-lg m-auto items-center">
        <div class="px-2 group">
          <Link<RootRoute> to={RootRoute::Home}>
            <p class="yg-jalnan text-xl group-hover:scale-110 block ease-in-out duration-200 group-hover:text-slate-700 dark:group-hover:text-slate-100 ">{ "Ohah의 공간" } </p>
            <p class="text-[3px] group-hover:text-slate-700 dark:group-hover:text-slate-100"> {"개발자의 주절주절"} </p>
          </Link<RootRoute>>
        </div>
        <div class="flex items-center pr-3 S-CoreDream-4Regular"> 
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
              class="text-2xl group px-2"
              onclick={toggle_login_modal.clone()}
            >
              <i class="hover:text-slate-700 dark:hover:text-slate-200 ri-login-box-line"></i>
            </button>
            } else {
              <div class="relative inline-flex justify-center items-center text-left">
                <button
                  class="text-2xl group w-6 h-6"
                  onclick={
                    let dropdown = dropdown.clone();
                    Callback::from(move|e:MouseEvent|{
                      e.prevent_default();
                      dropdown.set(!*dropdown);
                    })
                  }
                >
                  <img 
                    class="w-full h-full rounded-full"
                    src={format!("{}", user_state.avatar_url)} 
                  />
                </button>
                <Transition 
                  show={*dropdown}
                  enter="transition ease-in-out duration-300 transform"
                  enter_from="scale-y-0"
                  enter_to="scale-1"
                  leave="transition ease-in-out duration-300 transform"
                  leave_from="scale-y-1"
                  leave_to="scale-y-0"
                >
                  <div class="origin-top-right absolute top-5 right-0 w-36 rounded-md shadow-lg bg-white dark:bg-slate-800 dark:shadow-none shadow dark:text-slate-300 text-gray-700 ring-1 ring-black ring-opacity-5 focus:outline-none" tabindex="-1">
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
                  </div>
                </Transition>
              </div>
            }
            <button
              class="text-2xl group px-2"
            >
              <a href="https://github.com/ohah/" target="_blank"><i class="hover:text-slate-700 dark:hover:text-slate-200 ri-github-fill"></i></a>
            </button>
            <Dropdown<Parent>>
              <Dropdown<Button>> 
                <div>{"Test"}</div>
              </Dropdown<Button>>
              <Dropdown<Button>> 
                <div>{"Test2"}</div>
              </Dropdown<Button>>
              // <div> {"그냥"} </div>
              // <Transition 
              //   enter="transition ease-in-out duration-300 transform"
              //   enter_from="scale-y-0"
              //   enter_to="scale-1"
              //   leave="transition ease-in-out duration-300 transform"
              //   leave_from="scale-y-1"
              //   leave_to="scale-y-0"
              // >
              //   <div>
              //     <input 
              //       autofocus={true}
              //       class="flex flex-grow font-sans block text-sm w-full py-2 px-3 ring-1 ring-slate-900/10 text-slate-500 rounded-lg dark:bg-slate-800 dark:ring-0 dark:highlight-white/5 dark:text-slate-400 focus:outline-none"
              //       value="끼야으아"
              //     />
              //   </div>
              // </Transition>
            </Dropdown<Parent>>
            // <Dropdown>
            //   <div> {"왔냐~"} </div>
            //   <div> {"씪빵이다"} </div>
            // </Dropdown>
            // <Dropdown<String>>
            //   <div> {"무야호"} </div>
            //   <Transition 
            //     show={*_dropdown}
            //     enter="transition ease-in-out duration-300 transform"
            //     enter_from="scale-y-0"
            //     enter_to="scale-1"
            //     leave="transition ease-in-out duration-300 transform"
            //     leave_from="scale-y-1"
            //     leave_to="scale-y-0"
            //   >
            //     <div>
            //       <input 
            //         autofocus={true}
            //         class="flex flex-grow font-sans block text-sm w-full py-2 px-3 ring-1 ring-slate-900/10 text-slate-500 rounded-lg dark:bg-slate-800 dark:ring-0 dark:highlight-white/5 dark:text-slate-400 focus:outline-none"
            //         value="끼야으아"
            //       />
            //     </div>
            //   </Transition>
            // </Dropdown<String>>
            <button
              class="text-2xl group px-2"
              onclick={alert}
            >
              {"경고!"}
            </button>
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
          <GitHubLogin />
        </Modal>
      }
    </header>
  }
}