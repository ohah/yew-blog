use std::ops::Deref;

use comrak::{markdown_to_html, ComrakOptions};
use serde::{Serialize, Deserialize};
use wasm_bindgen::JsCast;
use yew::{ html, Properties, function_component, classes, use_state, Callback, use_node_ref };
use yew_router::{prelude::{use_history, History, use_route}};
use crate::{store::{toast::ToastStatus, blog::{Blog, WriteCommentPayload}}, router::root::RootRoute};
use web_sys::{Event, FocusEvent, MouseEvent, Element, HtmlTextAreaElement};

#[derive(Serialize, Deserialize, Debug, Clone, Default, Properties, PartialEq)]
pub struct CommentWriteProps {
	pub wr_id:usize,
	pub parent_id:Option<usize>,
	pub id:Option<usize>,
	pub content:Option<String>
}

#[derive(Default, Clone)]
struct Payload {
  pub content:String,
}

#[function_component(CommentWrite)]
pub fn comment_write(CommentWriteProps { wr_id, parent_id, id, content }:&CommentWriteProps) -> Html {
	const DEFAULT: &str = "Cmt-Write w-full";
	const ACTIVE:&str = "bg-white border-t border-l border-r border-[#b4b4b4] dark:border-[#100322] dark:bg-neutral-700";
	const SCROLL:&str = "scrollbar-thin dark:scrollbar-thumb-slate-400 dark:scrollbar-track-gray-600 scrollbar-thumb-gray-400 scrollbar-track-gray-300 scrollbar-rounded";
	let payload = use_state(|| Payload {
		content: content.clone().unwrap_or_default()
	});

	let view_state = use_state(||false);
	let route = use_route::<RootRoute>().unwrap();
	let history = use_history().unwrap();
	let textarea = use_node_ref();
	let preview_element = use_node_ref();

	let content_changed = {
		let payload = payload.clone();
		Callback::from(move|e:Event|{
			let value = e.target().unwrap().unchecked_into::<HtmlTextAreaElement>().value();
      let mut data = payload.deref().clone();
      data.content = value;
      payload.set(data);
		})
	};
	let preview = {
		let payload = payload.clone();
		let view_state = view_state.clone();
		let preview_element = preview_element.clone();
		Callback::from(move|e:MouseEvent|{
			e.prevent_default();
			view_state.set(true);
			let preview_element = preview_element.cast::<Element>().expect("render을 가져오지 못함");
			let text = markdown_to_html(payload.content.as_str(), &ComrakOptions::default());
			preview_element.set_inner_html(text.as_str());
		})
	};

	let editor = {
		let view_state = view_state.clone();
		Callback::from(move|e:MouseEvent| {
			view_state.set(false);
		})
	};

	let onsubmit = {
		let payload = payload.clone();
		let id = id.clone();
		let wr_id = wr_id.clone();
		let parent_id = parent_id.clone();
		let history = history.clone();
		let route = route.clone();
		// let textarea = textarea.clone();
		Callback::from(move |e: FocusEvent| {
      e.prevent_default();
			payload.set(Payload::default());
      if payload.content.is_empty() {
				Blog::toast_message("내용을 입력하세요", ToastStatus::DANGER, None);
      } else {
				Blog::write_comment(WriteCommentPayload {
					content: payload.content.to_string(),
					wr_id: wr_id,
					parent_id: parent_id,
					id: id,
				});
				// let element = textarea.cast::<HtmlInputElement>().expect("not load textarea");
				// element.set_value("");
				// log::info!("route, {:?}", route);
				history.replace(route.clone());
      }
    })
	};

	html! {
		<form 
			onsubmit={onsubmit}
			class={classes!(DEFAULT)}
		>
			<section class="rounded dark:bg-neutral-700 relative bg-white dark:border-[#100322] dark:border relative flex flex-col text-sm border-[#b4b4b4] border">
				<header class="bg-gray-100 dark:bg-[#1c1b1e] rounded-t">
					<ul class="flex pt-2 px-2">
						<li 
							onclick={editor}
							class={classes!(if *view_state.clone() == false {ACTIVE}else{""}, "cursor-pointer","z-10", "py-2.5", "px-3.5", "rounded-t", "mb-px")}
						>
							{"쓰기"} 
						</li>
						<li 
							onclick={preview}
							class={classes!(if *view_state.clone() == true {ACTIVE}else{""}, "cursor-pointer","z-10", "py-2.5", "px-3.5", "rounded-t", "mb-px")}
						> 
							{"미리보기"} 
						</li>
					</ul>
				</header>
				<div class="-mt-[2px] bg-white p-2 border-t-[#b4b4b4] dark:border-[#100322] border-t dark:bg-neutral-700">
					<textarea
						ref={textarea}
						onchange={content_changed}
						class={classes!(if *view_state.clone() == false {""}else{"hidden"}, "dark:bg-[#1c1b1e]", "w-full", "h-28", "rounded", "p-2", "bg-gray-100", "focus:outline-none", "text-xs", "focus:ring-0", "outline", "outline-none", SCROLL)}
						value={payload.content.clone()}
					/>					
					<div
						ref={preview_element}
						class={classes!(if *view_state.clone() == true {""}else{"hidden"}, "prose", "max-w-none", "dark:prose-invert", "preview", "dark:bg-[#1c1b1e]", "w-full", "h-28", "rounded", "p-2", "bg-gray-100", "focus:outline-none", "text-xs", "overflow-y", SCROLL)}
					>
					</div>
				</div>
				<footer class="pb-2 px-2 flex justify-between items-center select-none">
					<div class="text-xs">{"마크다운 문법으로 작성 가능합니다"}</div>
					<button 
						type="submit"
						class="bg-green-500 text-white border-[#100322] border rounded py-1 px-2 hover:bg-green-600 transition ease-in-out duration-75">
						{"댓글 달기"}
					</button>
				</footer>
			</section>
		</form>
	}
}