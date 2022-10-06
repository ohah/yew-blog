use std::ops::Deref;

use wasm_bindgen::{JsCast};
use web_sys::{MouseEvent, FocusEvent, HtmlTextAreaElement, Event, HtmlInputElement};
use yew::{ function_component, use_state, Callback, html, Properties };
use yew_router::prelude::{use_route, use_history};
use crate::components::transition::Transition;
use crate::components::card::Card;
use crate::components::tag_input::TagInput;
use crate::router::root::RootRoute;
use crate::store::blog::{ProfilePayload, Blog};
use crate::store::toast::ToastStatus;

use super::kanban_card::ProfileResponse;

#[derive(Default, Clone, Debug)]
pub struct Payload {
  pub category:String,
  pub fr_date:String,
  pub la_date:String,
  pub tag:String,
  pub label:String,
  pub description:String,
  pub state:String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Position {
  TopRight,
  TopLeft,
  BottomLeft,
  BottomRight,
}

impl Position {
  fn as_str(&self) -> &'static str {
    match self {
      Position::TopRight => "top-0 right-0",
      Position::TopLeft => "top-0 left-0",
      Position::BottomLeft => "bottom-0 left-0",
      Position::BottomRight => "bottom-0 right-0",
    }
  }
}

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct ProfileUpadteProps {
  pub position:Option<Position>,
  pub id:Option<usize>,
  pub value:Option<ProfileResponse>
}

#[function_component(ProfileUpdate)]
pub fn profile_update(ProfileUpadteProps { position, id, value }:&ProfileUpadteProps) -> Html {
  let route = use_route::<RootRoute>().unwrap();
  let history = use_history().unwrap();
  let position = match position {
    Some(position) => position.clone().as_str(),
    None => "top-0 right-0"
  };
  let id = match id {
    Some(id) => Some(id.clone()),
    None => None
  };

  
  let is_modal = use_state(||false);
  let initial_value = if value.is_some() {
    let value = value.clone().unwrap();
    Payload {
      category: value.category,
      fr_date: value.fr_date,
      la_date: value.la_date,
      tag: value.tag,
      label: value.label,
      description: value.description,
      state: value.state,
    }
  } else {
    Payload::default()
  };

  let payload = use_state(||initial_value);  
  
  let toggle_modal = {
    let is_modal = is_modal.clone();
    Callback::from(move|e:MouseEvent| {
      e.prevent_default();
      is_modal.set(!*is_modal);
    })
  };
  let tag_changed = {
    let payload = payload.clone();
    Callback::from(move|value| {      
      let mut data = payload.deref().clone();
      data.tag = value;
      payload.set(data);
    })
  };
  let description_changed = {
    let payload = payload.clone();
    Callback::from(move|e:Event| {
      let value = e.target().unwrap().unchecked_into::<HtmlTextAreaElement>().value();
      let mut data = payload.deref().clone();
      data.description = value;
      payload.set(data);
    })
  };
  let fr_date_changed = {
    let payload = payload.clone();
    Callback::from(move|e:Event| {
      let value = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
      let mut data = payload.deref().clone();
      data.fr_date = value;
      payload.set(data);
    })
  };
  let la_date_changed = {
    let payload = payload.clone();
    Callback::from(move|e:Event| {
      let value = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
      let mut data = payload.deref().clone();
      data.la_date = value;
      payload.set(data);
    })
  };
  let category_changed = {
    let payload = payload.clone();
    Callback::from(move|e:Event| {
      let value = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
      let mut data = payload.deref().clone();
      data.category = value;
      payload.set(data);
    })
  };
  let state_changed = {
    let payload = payload.clone();
    Callback::from(move|e:Event| {
      let value = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
      let mut data = payload.deref().clone();
      data.state = value;
      payload.set(data);
    })
  };
  let label_changed = {
    let payload = payload.clone();
    Callback::from(move|e:Event| {
      let value = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
      let mut data = payload.deref().clone();
      data.label = value;
      payload.set(data);
    })
  };
  let onsubmit = {
		let payload = payload.clone();
    let is_modal = is_modal.clone();
    let route = route.clone();
    let history = history.clone();
		Callback::from(move |e: FocusEvent| {
      e.prevent_default();
      if payload.label.to_string().is_empty() { 
        Blog::toast_message("라벨을 입력해주세요", ToastStatus::DANGER, None);
      } else if payload.description.to_string().is_empty() {
        Blog::toast_message("내용을 입력해주세요", ToastStatus::DANGER, None);
      } else if payload.tag.to_string().is_empty() {
        Blog::toast_message("태그를 입력해주세요", ToastStatus::DANGER, None);
      } else if payload.state.to_string().is_empty() {
        Blog::toast_message("상태를 입력해주세요", ToastStatus::DANGER, None);
      } else {
        Blog::profile_write(ProfilePayload {
          category:payload.category.to_string(),
          description:payload.description.to_string(),
          tag:payload.tag.to_string(),
          label:payload.label.to_string(),
          state:payload.state.to_string(),
          fr_date:payload.fr_date.to_string(),
          la_date:payload.la_date.to_string(),
          id: id,
        }, is_modal.clone(), history.clone(), route.clone());
        payload.set(Payload::default());
      }
    })
	};

   
  html!(
    <div> 
      <button 
        class={format!("add-profile absolute {} h-[35px] px-2 inline-flex items-center justify-center text-xs rounded hover:bg-black hover:bg-opacity-75 z-[9999]", position)}
        type="button"
        onclick={toggle_modal.clone()}
      > 
        <i class="ri-add-fill"></i>
      </button>
      <Transition 
        show={*is_modal}
        enter="transition ease-in-out duration-300 transform"
        enter_from="scale-y-0"
        enter_to="scale-1"
        leave="transition ease-in-out duration-300 transform"
        leave_from="scale-y-1"
        leave_to="scale-y-0"
        class="bg-black bg-opacity-75 fixed inset-0 h-screen w-full z-[10000] flex items-center justify-center dark:text-slate-400"
      >
        <Card
          title="프로필 수정"
          class="w-[95vw]"
        > 
          <form 
            onsubmit={onsubmit}
            class="flex flex-col gap-y-2"
          >
            <select 
              class="flex flex-grow font-sans block text-sm w-full py-2 px-3 ring-1 ring-slate-900/10 text-slate-500 rounded-lg dark:bg-slate-800 dark:ring-0 dark:highlight-white/5 dark:text-slate-400 focus:outline-none" 
              id="state"
              autocomplete="off"
              required={true}
              onchange={category_changed}
            >
              <option 
                value="선택해주세요" 
                disabled={true}
                selected={if payload.category.to_string().is_empty() {true} else {false}}
              >
                {"선택해주세요"} 
              </option>
              <option 
                value="작업물" 
                selected={if payload.category.to_string() == "작업물".to_string() {true} else {false}}
              >
                {"작업물"} 
              </option>
              <option
                value="패치내역"
                selected={if payload.category.to_string() == "패치내역".to_string() {true} else {false}}
              >
                {"패치내역"}
              </option>
            </select>
            <div class="flex flex-row gap-x-2 items-center">
              <label 
                for="fr_date"
                class="flex flex-none px-5 items-center font-bold w-32" 
              >
                { "기간" }
              </label>
              <input 
                class="flex flex-grow font-sans block text-sm w-full py-2 px-3 ring-1 ring-slate-900/10 text-slate-500 rounded-lg dark:bg-slate-800 dark:ring-0 dark:highlight-white/5 dark:text-slate-400 focus:outline-none" 
                id="fr_date"
                autocomplete="off"
                type="text"
                value={payload.fr_date.to_string()}
                onchange={fr_date_changed}
              />
              <label for="la_date"> {"~"} </label>
              <input 
                class="flex flex-grow font-sans block text-sm w-full py-2 px-3 ring-1 ring-slate-900/10 text-slate-500 rounded-lg dark:bg-slate-800 dark:ring-0 dark:highlight-white/5 dark:text-slate-400 focus:outline-none" 
                id="la_date"
                autocomplete="off"
                type="text"
                value={payload.la_date.to_string()}
                onchange={la_date_changed}
              />
            </div>
            <div class="flex flex-row gap-x-2">
              <label 
                for="label"
                class="flex flex-none px-5 items-center font-bold w-32 min-h-32" 
              >
                { "라벨" }
              </label>
              <input 
                class="flex flex-grow font-sans block text-sm w-full py-2 px-3 ring-1 ring-slate-900/10 text-slate-500 rounded-lg dark:bg-slate-800 dark:ring-0 dark:highlight-white/5 dark:text-slate-400 focus:outline-none" 
                id="label"
                autocomplete="off"
                type="text"
                value={payload.label.to_string()}
                onchange={label_changed}
              />
            </div>
            <div class="flex flex-row gap-x-2">
              <label 
                for="description"
                class="flex flex-none px-5 items-center font-bold w-32 min-h-32" 
              >
                { "내용" }
              </label>
              <textarea 
                id="description" 
                autocomplete="off"
                class="flex flex-grow font-sans block text-sm w-full py-2 px-3 ring-1 ring-slate-900/10 text-slate-500 rounded-lg dark:bg-slate-800 dark:ring-0 dark:highlight-white/5 dark:text-slate-400 focus:outline-none" 
                value={payload.description.to_string()}
                onchange={description_changed}
              />
            </div>
            <div class="flex flex-row gap-x-2">
              <label 
                for="tag"
                class="flex flex-none px-5 items-center font-bold w-32" 
              >
                { "태그" }
              </label>
              <TagInput
                onchange={tag_changed}
                default_value={payload.tag.to_string()}
              />
            </div>
            <div class="flex flex-row gap-x-2">
              <label 
                for="state"
                autocomplete="off"
                class="flex flex-none px-5 items-center font-bold w-32" 
              >
                {"상태"}
              </label>
              <select 
                class="flex flex-grow font-sans block text-sm w-full py-2 px-3 ring-1 ring-slate-900/10 text-slate-500 rounded-lg dark:bg-slate-800 dark:ring-0 dark:highlight-white/5 dark:text-slate-400 focus:outline-none" 
                id="state"
                autocomplete="off"
                required={true}
                onchange={state_changed}
              >
                <option 
                  value="선택해주세요" 
                  disabled={true}
                  selected={if payload.state.to_string().is_empty() {true} else {false}}
                >
                  {"선택해주세요"} 
                </option>
                <option 
                  value="할일" 
                  selected={if payload.state.to_string() == "할일".to_string() {true} else {false}}
                >
                  {"할일"} 
                </option>
                <option
                  value="진행중"
                  selected={if payload.state.to_string() == "진행중".to_string() {true} else {false}}
                >
                  {"진행중"}
                </option>
                <option
                  value="완료"
                  selected={if payload.state.to_string() == "완료".to_string() {true} else {false}}
                >
                  {"완료"}
                </option>
              </select>
            </div>
            <div class="flex justify-end gap-x-3">
              <button type="submit" class="rounded px-3 py-2 bg-green-500 text-white font-bold hover:bg-green-600 duration-200 transition flex justify-center items-center space-x-2"> {"확인"} </button>
              <button 
                type="button" 
                onclick={toggle_modal.clone()}
                class="rounded px-3 py-2 bg-red-500 text-white font-bold hover:bg-red-600 duration-200 transition flex justify-center items-center space-x-2"> {"취소"} </button>
            </div>
          </form>
        </Card>
      </Transition>
    </div>
  )
}