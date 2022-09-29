use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use yew::{ html, function_component, Properties, Callback, use_state, use_state_eq, use_node_ref, use_effect_with_deps };
use web_sys::{KeyboardEvent, HtmlInputElement, MouseEvent, HtmlElement};
use itertools::Itertools;
use web_sys::Event;

use crate::store::{toast::ToastStatus, blog::Blog};

#[derive(Clone, Properties, PartialEq, Debug)]
pub struct CategoryInputProps {
	pub default_value:Option<String>,
	pub onchange:Callback<String>,
	// pub list:Vec<String>,
	// pub api:&str,
}

#[derive(Clone, Properties, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct SearchResponse {
	pub list:Vec<String>
}

#[function_component(CategoryInput)]
pub fn category_input(CategoryInputProps { default_value, onchange}: &CategoryInputProps) -> Html {
	let all_category = use_state(||SearchResponse::default());
	let select_input = use_node_ref();
	let dropdown = use_state(|| false);
	let show_list = use_state(||vec![]);
	let list_index = use_state_eq(||0);

	{
		let select_input = select_input.clone();
		let default_value = default_value.clone();
		use_effect_with_deps(move|default_value| {
			if !default_value.clone().unwrap_or_default().is_empty() {
				let input = select_input.cast::<HtmlInputElement>().expect("render을 가져오지 못함");
				input.set_value(default_value.clone().unwrap_or_default().as_str());
			}
			|| ()
		}, default_value.clone())
	}
	{
		let all_category = all_category.clone();
		use_effect_with_deps(move|_| {
			wasm_bindgen_futures::spawn_local(async move {
				let response = Request::get("/api/category")
				.header("accept", "applcation/json")
				.header("Access-Control-Allow-Origin", "no-cors")
				.send()
				.await
				.unwrap()
				.json::<SearchResponse>()
				.await;
				match response {
					Ok(response) => {
						all_category.set(response);
					}
					Err(err) => {
						log::error!("{:?}", err);
						Blog::toast_message("카테고리 목록을 가져오는데 실패하였습니다.", ToastStatus::DANGER, None);
					}
				};
			});
			|| ()
		}, ());
	}

	let select = {
		let select_input = select_input.clone();
		let dropdown = dropdown.clone();
		let onchange = onchange.clone();
		Callback::from(move |e:MouseEvent| {
			e.prevent_default();
			let target = e.target().unwrap().unchecked_into::<HtmlElement>();
			let value = target.text_content().unwrap_or_default();
			let input = select_input.cast::<HtmlInputElement>().expect("render을 가져오지 못함");
			input.set_value(value.as_str());
			onchange.emit(value.clone());
			dropdown.set(false);
		})
	};
	let keydown = {
		let list_index = list_index.clone();
		let show_list = show_list.clone();
		let dropdown = dropdown.clone();
		Callback::from(move|e:KeyboardEvent| {
			let key = e.key();
			let key = key.as_str();
			let input = e.target().unwrap().unchecked_into::<HtmlInputElement>();
			let value = input.clone().value();
			if !value.as_str().is_empty() {
				dropdown.set(true);
			}
			if key == "ArrowUp" {
				e.prevent_default();
				let index = if *list_index <= 0 {
					show_list.len() - 1
				} else {
					*list_index - 1
				};
				list_index.set(index);
			} else if key == "ArrowDown" {
				e.prevent_default();
				let index = if *list_index < show_list.len() - 1 {
					*list_index + 1
				} else {
					0
				};
				list_index.set(index);
			} else if key == "Enter" {
				e.prevent_default();
				dropdown.set(false);
			} else {
				list_index.set(0);
			}
		})
	};

	let keyup = {
		let dropdown = dropdown.clone();
		let show_list = show_list.clone();
		let list_index = list_index.clone();
		let all_category = all_category.clone();
		let onchange = onchange.clone();
		Callback::from(move|e:KeyboardEvent| {
			let key = e.key();
			let key = key.as_str();
			let input = e.target().unwrap().unchecked_into::<HtmlInputElement>();
			let value = input.clone().value();
			let result = all_category.list.iter().filter(|&row| {
				let row = row.clone().to_lowercase();
				let value = value.clone().to_lowercase();
				row.contains(value.as_str())
			}).map(|row|{
				row.clone().to_string()
			}).collect_vec();
			if result.len() > 0 {
				show_list.set(result.clone());
				dropdown.set(true);
			} else {
				show_list.set(vec![])
			}
			if key == "Enter" {
				let index = *list_index.clone();
				let value = &result[index];
				input.clone().set_value(value.clone().as_str());
				onchange.emit(value.clone());
				show_list.set(vec![]);
				dropdown.set(false);
			} else if key == "Backspace" && value.len() == 0 {
				show_list.set(vec![]);
				dropdown.set(false)
			}
		})
	};

	let list_open = {
		let show_list = show_list.clone();
		let all_category = all_category.clone();
		let dropdown = dropdown.clone();
		Callback::from(move|e:MouseEvent|{
			e.prevent_default();
			let list = all_category.list.to_vec();
			show_list.set(list);
			dropdown.set(true);
		})
	};

	let input_onchange = {
		let onchange = onchange.clone();
		Callback::from(move|e:Event| {
			let input = e.target().unwrap().unchecked_into::<HtmlInputElement>();
			let value = input.clone().value();
			if !value.as_str().is_empty() {
				onchange.emit(value);
			}
		})
	};
	let show_dropdown = if *dropdown == false {
		"hidden"
	}else {
		""
	};
	html! {
		<div 
			class="flex flex-col relative flex-grow"
		>
			<input 
				ref={select_input}
				onkeydown={keydown}
				onkeyup={keyup}
				onchange={input_onchange}
				class="flex flex-grow font-sans block text-sm w-full py-2 px-3 ring-1 ring-slate-900/10 text-slate-500 rounded-lg dark:bg-slate-800 dark:ring-0 dark:highlight-white/5 dark:text-slate-400 focus:outline-none" 
			/> 
			<div 
				class={format!("scrollbar-thin dark:scrollbar-thumb-gray-900 dark:scrollbar-track-gray-800 scrollbar-thumb-gray-400 scrollbar-track-gray-300 origin-top-right max-h-2xl absolute top-7 right-0 mt-2 w-full rounded-md shadow-lg bg-white dark:bg-slate-800 dark:shadow-none shadow dark:text-slate-400 text-gray-700 ring-1 ring-black ring-opacity-5 focus:outline-none z-[100] {}", show_dropdown)} 
				tabindex="-1"
			>
				<div class="divide-y-2 dark:divide-slate-700">
					{
						for show_list.iter().enumerate().map(move|(i, value)| {
							let value = value.clone();
							let active = if *list_index.clone() == i  {
								"dark:bg-neutral-900 bg-gray-200"
							} else {
								""
							};
							html! {
								<li
									class={format!("flex w-full cursor-pointer dark:hover:bg-neutral-900 hover:bg-gray-200 text-left px-4 py-2 text-sm space-x-2 last:rounded-b first:rounded-t {}", active)}
									onclick={select.clone()}
									key={i}
								> 
									{value}
								 </li> 
							}
						})
					}
				</div>
			</div>
			<div 
				class="absolute h-full w-8 inline-flex justify-center top-0 right-0 items-center cursor-pointer"
				onclick={list_open}
			>
				<i class="ri-arrow-down-s-line"></i>
			</div>
		</div>
	}
}