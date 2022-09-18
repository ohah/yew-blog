use wasm_bindgen::JsCast;
use yew::{ html, function_component, Properties, Callback, use_state, UseStateHandle, use_state_eq };
use web_sys::{KeyboardEvent, HtmlInputElement, MouseEvent, HtmlElement};
use itertools::Itertools;
use web_sys::Event;

#[derive(Properties, PartialEq)]
pub struct TagInputProps {
	pub default_value:Option<String>,
	pub onchange:Callback<Event>
}

#[function_component(TagInput)]
pub fn tag_input(props: &TagInputProps) -> Html {
	let tag = use_state_eq(||vec![]);
	let keyup = {
		let tag = tag.clone();
		Callback::from(move |e:KeyboardEvent| {
			e.prevent_default();
			let value = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
			let key = e.key();
			let key = key.as_str();
			let tag = tag.clone();
			// log::info!("키 입력, {:?}", key);
			if key == " " || key == "," {
				log::info!("태그 분리, {:?}", key);
				log::info!("state, {:?}", tag);
				// let mut array = tag_state.split(",");
				// let e = e.clone();
				let mut vec = tag.to_vec();
				vec.push(value.replace(",", "").trim().to_string());
				vec.dedup_by(|a, b| a == b);
				let vec = vec.clone().into_iter().unique().collect();
				// tag_state.push(value);
				tag.set(vec);
				// tag = Vec::new();
				// tag_state.set(vec);
				// let value = value.trim();
				// let tag = if tag_state.len() == 0 {
				// 	format!("{}",value)
				// } else {
				// 	let tag = tag_state.trim().get(0..).unwrap().to_string();
				// 	format!("{},{}", tag, value)
				// };
				// let tag  = tag.split(",").unique().join(",");
				// tag_state.set(tag);
				e.target().unwrap().unchecked_into::<HtmlInputElement>().set_value("");
				// e.target().unwrap().unchecked_into::<HtmlInputElement>().focus();
			}
		})
	};
	fn remove (tag:UseStateHandle<Vec<String>>, index:usize) -> Callback<MouseEvent> {
		Callback::from(move|e:MouseEvent| {
			e.prevent_default();
			let mut vec = tag.to_vec();
			vec.remove(index);
			tag.set(vec);
		})
	};
	// let remove= {
	// 	let tag = tag.clone();
	// 	Callback::from(move|e:MouseEvent| {
	// 		let value = e.target().unwrap().unchecked_into::<HtmlElement>().get;
	// 		let index = tag.iter().position(|x| *x == value).unwrap();
	// 		let mut vec = tag.to_vec();
	// 		vec.remove(index);
	// 		tag.set(vec);
	// 	})
	// };
	html! {
		<div 
			class="flex font-sans text-sm py-2 px-3 items-center w-full gap-x-1 dark:bg-slate-800 dark:ring-0 dark:highlight-white/5 dark:text-slate-400 text-slate-500"
		>
			{
				for tag.iter().enumerate().map(|(i, _tag)| {
					let _tag = _tag.clone();
					let tag = tag.clone();
					html! {
						<div
							class="flex-none text-ellipsis text-xs overflow-hidden inline-flex bg-slate-200 text-gray-600 dark:bg-slate-600 dark:text-gray-100 inline-flex px-1 py-0.5 rounded gap-x-1 items-center" 
							key={i}
						> 
							<span class="inline-flex">{_tag}</span>
							<span 
								class="cursor-pointer hover:text-black dark:hover:text-white inline-flex items-center"
								onclick={remove(tag, i)}
							>
								<i class="ri-close-fill"></i>
							</span>
						</div>
					}
				})
			}
			<input 
				class="bg-transparent flex flex-grow ring-1 ring-slate-900/10 focus:outline-none flex flex-grow" 
				onkeyup={keyup}
			/>
		</div>
	}
}