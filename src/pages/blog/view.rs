use web_sys::{Element, MouseEvent};
use gloo_net::http::Request;
use wasm_bindgen::prelude::{wasm_bindgen};
use yew::{ function_component, html, Properties, use_state, use_effect_with_deps,use_node_ref, Html, Callback };
use comrak::{markdown_to_html, ComrakOptions};
use yewdux::prelude::{use_store};
use crate::{components::list_card::Datas, store::{toast::{ToastStatus}, github_auth::GithubUser, blog::{Blog, DeletePayload}}, router::root::RootRoute, components::comment::Comment};
use crate::{router::root::{WriteRoute}};
use yew_router::{components::Link, prelude::{use_history,use_route}};
use crate::components::modal::Modal;

#[wasm_bindgen(module = "/src/vscode/highlight.js")]
extern "C" {
  fn highlightAll();
}

#[derive(Properties, PartialEq, Clone)]
pub struct ViewProps {
  pub seo_title:String,
}
#[function_component(View)]
pub fn view(ViewProps{ seo_title }:&ViewProps) -> Html {
  let views = use_state(||Datas::default());  
  let (member, _ ) = use_store::<GithubUser>();
  let route = use_route::<RootRoute>().unwrap();
  let render = use_node_ref();
  let delete_modal = use_state(||false);
  {
    let views = views.clone();
    let url = format!("/api/view/{}", seo_title.clone());
    let render = render.clone();
    use_effect_with_deps(move |_| {
      wasm_bindgen_futures::spawn_local(async move {        
        let fetched_list  = Request::get(url.as_str())
          .header("accept", "application/json")
          .header("Access-Control-Allow-Origin", "no-cors")
          .send()
          .await
          .unwrap()
          .json::<Datas>()
          .await;
        match fetched_list {
          Ok(fetched_list) => {
            views.set(fetched_list.clone());
            let render = render.cast::<Element>().expect("render을 가져오지 못함");
            let text = markdown_to_html(fetched_list.content.as_str(), &ComrakOptions::default());
            render.set_inner_html(text.as_str());
            highlightAll();
          }
          Err(err) => {
            log::error!("{:?}", err);
            Blog::toast_message("알 수 없는 오류가 발생헀습니다.", ToastStatus::DANGER, None);
          }
        };
      });
      || ()
    }, seo_title.clone());
  }
  let delete = {
    let history = use_history().unwrap();
    let route = route.clone();
    let views = views.clone();
    Callback::from(move |e:MouseEvent| {
      e.prevent_default();
      Blog::remove( DeletePayload {
        id: views.id
      }, history.clone(), route.clone());
      // history.push(RootRoute::List { page: 1.to_string() });
    })
  };
	let delete_close = {
    let delete_modal = delete_modal.clone();
    Callback::from(move |e:MouseEvent| {
      e.prevent_default();
      delete_modal.set(false);
    })
  };

	let delete_modal_open = {
		let delete_modal = delete_modal.clone();
		{
			Callback::from(move |e:MouseEvent| {
				e.prevent_default();
				delete_modal.set(true);
			})
		}
	};


  // let views = views.clone();
  let member = member.clone();
  let tag = views.tag.as_str().split(",");
  let wr_id = views.id.clone();
	html! {
    <div class="lg:max-w-screen-lg px-2 mt-3 flex flex-col gap-y-5">
      <h2 class="text-2xl font-semibold yg-jalnan"> { format!("{}", views.title) } </h2>
      <section 
        class="prose max-w-none mx-auto w-full py-3 dark:prose-invert view-prose"
        ref={render}
      >
        <div class="flex justify-center items-center relative h-32 w-32 mx-auto">
          <svg class="absolute inset-0 animate-spin -ml-1 mr-3 w-full h-full text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          <div class="animate-pulse font-bold text-xs aboslute top-[50%] left-[50%]"> {"LOADING..."} </div>
        </div>
      </section>
      <div class="space-y-3">
        <div class="flex pt-3 flex-wrap gap-y-[5px] gap-x-[5px]">
        {
          tag.map(|row| {
            html!{
              <div class="text-ellipsis text-xs overflow-hidden inline-flex bg-slate-200 text-gray-600 dark:bg-slate-600 dark:text-gray-100 inline-flex px-1 py-0.5 rounded"> { row } </div>
            }
          }).collect::<Html>()
        }
        </div>
        <div class="flex justify-between">
          <div class="space-x-3 divide-x dark:divide-slate-600 flex text-xs flex items-center">
            <div class="group flex items-center space-x-2 font-bold">
              <img
                class="rounded-full w-6 h-6"
                src={format!("{}", views.avatar_url)}
              />
              <span>
                { format!("{}", views.name) }
              </span>
            </div>
            <span class="pl-3"> { format!("{}", views.created_at) } </span>
          </div>
          {
            if *member.name == views.name {
              html! {
                <div class="inline-flex items-center text-xs">
                  <Link<WriteRoute> to={WriteRoute::ModifyWrite {id : views.id }}>
                    <div class="flex items-center gap-x-1 hover:text-black dark:hover:text-white">
                      <i class="inline-flex items-center w-6 h-6 justify-center ri-edit-box-line"> </i>
                      <span> {"수정하기"} </span>
                    </div>
                  </Link<WriteRoute>>
                  <button
                    class="flex items-center gap-x-1 hover:text-black dark:hover:text-white"
                    onclick={delete_modal_open}
                  >
                    <i class="inline-flex items-center w-6 h-6 justify-center ri-delete-bin-line"></i>
                    <span> {"삭제하기"} </span>
                  </button>
                  if *delete_modal == true {
                    <Modal
                      is_close={delete_close.clone()}
                    >
                      <div class="flex flex-col dark:bg-slate-800 bg-white dark:shadow-none shadow w-80 py-5 px-5 rounded-lg space-y-2 modalIn gap-y-3">
                        <h2 class="yg-jalnan t text-2xl"> {"삭제하기"} </h2>
                        <div>
                          {"해당 댓글을 삭제하시겠습니까?"}
                        </div>
                        <div class="flex justify-end">
                          <div class="flex gap-x-2">
                            <button
                              class="inline-flex items-center text-white bg-pink-800 px-2 py-1 rounded hover:bg-pink-900 trasition duration-200 ease-in-out"
                              onclick={delete}
                            >
                              <span> {"확인"} </span>
                            </button>
                            <button
                              class="inline-flex items-center text-black bg-gray-300 px-2 py-1 rounded hover:bg-gray-400 trasition duration-200 ease-in-out"
                              onclick={delete_close.clone()}
                            >
                              <span> {"취소"} </span>
                            </button>
                          </div>
                        </div>
                      </div>
                    </Modal>
                  }
                </div>
              }
            } else {
              html! {
                <></>
              }
            }
          }
        </div>
      </div>
      <Comment
        wr_id = {wr_id}
      />
    </div>
  }
}