use std::{ops::Deref};

use gloo_net::http::Request;
use serde::{Serialize, Deserialize};
use wasm_bindgen::{JsCast, prelude::wasm_bindgen};
use web_sys::{FocusEvent, HtmlInputElement, HtmlTextAreaElement, Event};
use yew::{ function_component, html,  Callback, use_state, use_effect_with_deps, Properties };
use yew_router::prelude::{ use_history, use_route };

use crate::components::tag_input::TagInput;
use crate::components::category_input::CategoryInput;
use crate::{router::root::{RootRoute}, store::{blog::{Blog, WritePayload }, toast::{ToastStatus}}};

#[derive(Serialize, Deserialize)]
pub struct Parameter {
  id:i32,
}

#[derive(Default, Clone, Debug)]
pub struct Data {
  pub content:String,
	pub tag:String,	
	pub title:String,
	pub member:String,
	pub category:String,
}

#[wasm_bindgen(module = "/src/vscode/editor.js")]
extern "C" {
  fn editor_init(element:&str, value:&str);
}

#[derive(Properties, PartialEq, Clone)]
pub struct WriteProps {
  pub id:Option<usize>,
}

#[derive(Properties, PartialEq, Clone, Deserialize, Serialize, Default, Debug)]
pub struct ModifyResponse {
  pub category:String,
  pub content:String,
  #[serde(rename="createdAt")]
  pub created_at:String,
  pub description:String,
  pub id:usize,
  pub member:String,
  #[serde(rename="seoTitle")]
  pub seo_title:String,
  pub tag:String,
  pub title:String,
  #[serde(rename="updatedAt")]
  pub updated_at:String,
}

#[function_component(Write)]
pub fn wrtie( WriteProps { id }:&WriteProps) -> Html {
  let payload = use_state(Data::default);
  let route = use_route::<RootRoute>().unwrap();
  let history = use_history().unwrap();
  let title_changed = {
    let payload = payload.clone();
    Callback::from(move|e:Event| {
      let value = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
      let mut data = payload.deref().clone();
      data.title = value;
      payload.set(data);
    })
  };

  let category_changed = {
    let payload = payload.clone();
    Callback::from(move|value| {
      log::info!("{:?}", value);
      let mut data = payload.deref().clone();
      data.category = value;
      payload.set(data);
    })
  };

  let content_changed = {
    let payload = payload.clone();
    Callback::from(move|e:Event| {
      let value = e.target().unwrap().unchecked_into::<HtmlTextAreaElement>().value();
      let mut data = payload.deref().clone();
      data.content = value;
      // log::info!("textarea : {:?}", data.content);
      payload.set(data);
    })
  };

  let tag_changed = {
    let payload = payload.clone();
    Callback::from(move|value| {
      // log::info!("tag_changed {:?}", value);
      let mut data = payload.deref().clone();
      data.tag = value;
      payload.set(data);
    })
  };

  {
    let id = id.clone();
    let id = match id {
      Some(id) => id,
      None => 0
    };
    let url = format!("/api/write/{}", id.clone());
    let payload = payload.clone();
    use_effect_with_deps(move |_| {
      if id > 0 { // 여기 문제
        wasm_bindgen_futures::spawn_local(async move {
          let fetched_list  = Request::get(url.as_str())
          .header("accept", "application/json")
          .header("Access-Control-Allow-Origin", "no-cors")
          .send()
          .await
          .unwrap()
          .json::<ModifyResponse>()
          .await
          .unwrap();

          // log::info!("id {:?}", fetched_list);
          let mut data = payload.deref().clone();

          data.category = fetched_list.category;
          data.title = fetched_list.title;
          data.tag = fetched_list.tag;
          data.content = fetched_list.content;
          editor_init("#content", data.content.as_str());

          payload.set(data);
        });
      } else {
        editor_init("#content", "");
      }
    || ()
    }, ());
  }
  
  let onsubmit = {
    let payload = payload.clone();
    let id = id.clone();
    let route = route.clone();
    let history = history.clone();
    Callback::from(move |e: FocusEvent| {
      e.prevent_default();
      if payload.title.is_empty() {
        Blog::toast_message("제목을 입력하세요", ToastStatus::DANGER, None);
      } else if payload.category.is_empty() {
        Blog::toast_message("카테고리를 입력하세요", ToastStatus::DANGER, None);
      } else if payload.content.is_empty() {
        Blog::toast_message("내용을 입력하세요", ToastStatus::DANGER, None);
      } else if payload.tag.is_empty() {
        Blog::toast_message("태그를 입력하세요", ToastStatus::DANGER, None);
      } else {
        Blog::write(WritePayload {
          content:payload.content.to_string(),
          tag:payload.tag.to_string(),
          title:payload.title.to_string(),
          category:payload.category.to_string(),
          id:id,
        }, history.clone(), route.clone());
      }
    })
  };
	html! {
    <form 
      onsubmit={onsubmit}
      class="flex flex-col space-y-3 py-2"
      style="height:calc(100vh - 60px)"
    >
      <div class="flex flex-none space-x-1">
        <label 
          for="title"
          class="flex flex-none px-5 items-center font-bold w-32" 
        >
          { "제목" }
        </label>
        <input 
          id="title" 
          onchange={title_changed}
          value={payload.title.to_string()}
          class="flex flex-grow font-sans block text-sm w-full py-2 px-3 ring-1 ring-slate-900/10 text-slate-500 rounded-lg dark:bg-slate-800 dark:ring-0 dark:highlight-white/5 dark:text-slate-400 focus:outline-none" 
        />
      </div>
      <div class="flex flex-none space-x-1">
        <label 
          for="title"
          class="flex flex-none px-5 items-center font-bold w-32" 
        >
          { "카테고리" }
        </label>
        <CategoryInput 
          onchange={category_changed.clone()}
          default_value={payload.category.to_string()}
        />
      </div>
      <div class="flex-grow flex w-full">
        <div class="flex flex-col flex-auto overflow-hidden" id="content"></div>
        <textarea 
          onchange={content_changed} 
          data-id="#content" 
          class="hidden ring-1 ring-slate-900/10 text-slate-500 rounded-lg dark:bg-slate-800 dark:ring-0 dark:highlight-white/5 dark:text-slate-400 focus:outline-none w-full"
          value={payload.content.to_string()}
        />
      </div>
      <div class="flex flex-none">
        <label 
          for="title"
          class="flex flex-none px-5 items-center font-bold w-32" 
        >
          { "태그" }
        </label>
        <TagInput 
          onchange={tag_changed}
          default_value={payload.tag.to_string()}
        />
      </div>
      <div class="flex flex-none">
        <button 
          type="submit"
          class="w-full rounded px-3 py-2 bg-blue-500 text-white font-bold hover:bg-blue-600 duration-200 transition flex justify-center items-center space-x-2" 
        >
          <i class="ri-edit-box-line"></i>
          <span>{ "보내기" }</span>
        </button>
      </div>
    </form>
  }
}