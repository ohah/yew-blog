use std::result;

use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use yew::{ html, function_component, Properties, Callback, use_state, UseStateHandle, use_state_eq, use_node_ref, use_effect_with_deps, use_effect };
use web_sys::{KeyboardEvent, HtmlInputElement, MouseEvent, HtmlElement, Element, FocusEvent};
use itertools::Itertools;
use web_sys::Event;

#[derive(Clone, Properties, PartialEq, Debug)]
pub struct SearchInputProps {
	pub default_value:Option<String>,
	pub onchange:Callback<Event>,
	pub list:Vec<String>
}

#[function_component(SearchInput)]
pub fn search_input(SearchInputProps { default_value, onchange, list }: &SearchInputProps) -> Html {
	let select_input = use_node_ref();
	let dropdown = use_state(|| false);
	let show_list = use_state(||vec![]);
	let select = {
		let select_input = select_input.clone();
		let show_list = show_list.clone();
		Callback::from(move |e:MouseEvent| {
			e.prevent_default();
			let target = e.target().unwrap().unchecked_into::<HtmlElement>();
			let value = target.text_content().unwrap_or_default();
			let input = select_input.cast::<HtmlInputElement>().expect("render을 가져오지 못함");
			log::info!("value : {:?}", value);
			input.set_value(value.as_str());
			// show_list.set(vec![]);
		})
	};
	let keyevent = {
		let list = list.clone();
		let show_list = show_list.clone();
		Callback::from(move|e:KeyboardEvent| {
			let input = e.target().unwrap().unchecked_into::<HtmlInputElement>();
			let value = input.clone().value();
			let result = list.iter().filter(|&row| {
				let row = row.clone().to_lowercase();
				let value = value.clone().to_lowercase();
				row.contains(value.as_str())
			}).map(|row|{
				row.clone().to_string()
			}).collect_vec();
			log::info!("result {:?}", result);
			show_list.set(result);
		})
	};
	let focusout = {
		let show_list = show_list.clone();
		Callback::from(move|e:FocusEvent|{
			// show_list.set(vec![]);
		})
	};
	// let auto_list = show_list.clone().to_vec();
	html! {
		<div 
			class="flex flex-col relative flex-grow"
		>
			<input 
				ref={select_input}
				onkeyup={keyevent}
				onfocusout={focusout}
				class="flex flex-grow font-sans block text-sm w-full py-2 px-3 ring-1 ring-slate-900/10 text-slate-500 rounded-lg dark:bg-slate-800 dark:ring-0 dark:highlight-white/5 dark:text-slate-400 focus:outline-none" 
			/> 
			<div class={format!("scrollbar-thin dark:scrollbar-thumb-gray-900 dark:scrollbar-track-gray-800 scrollbar-thumb-gray-400 scrollbar-track-gray-300 origin-top-right max-h-2xl absolute top-7 right-0 mt-2 w-full rounded-md shadow-lg bg-white dark:bg-slate-900 dark:shadow-none shadow dark:text-slate-400 text-gray-700 ring-1 ring-black ring-opacity-5 focus:outline-none z-[100] {}", "test")} tabindex="-1">
				<div class="py-1 divide-y-2 dark:divide-slate-700">
					{
						for show_list.iter().enumerate().map(move|(i, value)| {
							let value = value.clone();
							html! {
								<li 
									class="flex w-full cursor-pointer dark:hover:bg-neutral-900 hover:bg-gray-200 text-left px-4 py-2 text-sm space-x-2 last:rounded-b first-rounded-t"
									onclick={select.clone()}
									key={i}
								> {value} </li> 
							}
						})
					}
				</div>
			</div>
		</div>
	}
}