use wasm_bindgen::JsCast;
use yew::{ html, function_component, Properties, Callback,  UseStateHandle, use_state_eq, use_node_ref, use_effect_with_deps};
use web_sys::{KeyboardEvent, HtmlInputElement, MouseEvent, };
use itertools::Itertools;
use web_sys::Event;

use crate::store::{blog::Blog, toast::ToastStatus};

#[derive(Properties, PartialEq)]
pub struct TagProps {
	pub default_value:Option<String>,
	pub list:Vec<String>,
	pub remove:Callback<usize>
}

#[function_component(Tag)]
pub fn tag(props:&TagProps) -> Html {
	fn remove(remove:Callback<usize>, i:usize) -> Callback<MouseEvent>{
		Callback::from(move|e:MouseEvent| {
			e.prevent_default();
			remove.emit(i);
		})
	}
	html! {
		{
			for props.list.iter().enumerate().map(|(i, _tag)| {
				let _tag = _tag.clone();
				let _remove = props.remove.clone();
				html! {
					<div
						class="flex-none text-ellipsis text-xs overflow-hidden inline-flex bg-slate-200 text-gray-600 dark:bg-slate-600 dark:text-gray-100 inline-flex px-1 py-0.5 rounded gap-x-1 items-center" 
						key={i}
					> 
						<span class="inline-flex">{_tag}</span>
						<span 
							class="cursor-pointer hover:text-black dark:hover:text-white inline-flex items-center"
							onclick={remove(_remove, i)}
						>
							<i class="ri-close-fill"></i>
						</span>
					</div>
				}
			})
		}
	}

}


#[derive(Properties, PartialEq)]
pub struct TagInputProps {
	pub default_value:Option<String>,
	pub onchange:Callback<String>
}

#[function_component(TagInput)]
pub fn tag_input(TagInputProps { default_value, onchange }: &TagInputProps) -> Html {
	let _default_value = default_value
	.clone()
	.unwrap_or_default()
	.trim()
	.split(",")
	.map(|s| s.trim().to_string())
	.collect::<Vec<String>>();
	let tag = use_state_eq(||Vec::new());
	let input_box = use_node_ref();
	{
		let input_box = input_box.clone();
		let default_value = default_value.clone();
		let _default_value = _default_value.clone();
		let tag = tag.clone();
		use_effect_with_deps(move|default_value| {
			if !default_value.clone().unwrap_or_default().is_empty() {
				tag.set(_default_value);
			}
			|| ()
		}, default_value.clone())
	}

	let keydown = {
		let handle_onchange = onchange.clone();
		let tag = tag.clone();
		Callback::from(move |e:KeyboardEvent| {
			let input = e.target().unwrap().unchecked_into::<HtmlInputElement>();
			let value = input.clone().value().to_lowercase();
			let key = e.key();
			let key = key.as_str();
			let tag = tag.clone();
			// log::info!("{:?}", key.clone());
			let mut vec = tag.to_vec();
			if  key == "Backspace" && value.clone().is_empty() && vec.len() > 0 {
				e.prevent_default();
				vec.remove(vec.len() - 1);
				let set_vec = vec.clone().into_iter().unique().collect::<Vec<String>>();
				tag.set(set_vec);
				let emit = vec.clone().into_iter().unique().collect::<Vec<String>>().join(",");
				handle_onchange.emit(emit);
			} else if key == " " || key == "," || key == "Enter" {
				e.prevent_default();
				if tag.len() >= 5 {
					Blog::toast_message("태그는 5개 이상 등록할 수 없습니다", ToastStatus::DANGER, None);
				} else if !value.clone().as_str().trim().is_empty() {
					let mut vec = tag.clone().to_vec();
					vec.push(value.replace(",", "").trim().to_string());
					vec.dedup_by(|a, b| a == b);
					let set_vec = vec.clone().into_iter().unique().collect::<Vec<String>>();
					tag.set(set_vec);
					let emit = vec.clone().into_iter().unique().collect::<Vec<String>>().join(",");
					handle_onchange.emit(emit);
					input.clone().set_value("");
				}
			}
		})
	};

	let input_onchange = {
		let handle_onchange = onchange.clone();
		Callback::from(move | event: Event | {
			let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
			handle_onchange.emit(value);
		})
	};

	fn remove (tag:UseStateHandle<Vec<String>>, index:usize, handle_onchange:Callback<String>) -> Callback<MouseEvent> {
		Callback::from(move|e:MouseEvent| {
			e.prevent_default();
			let mut vec = tag.to_vec();
			vec.remove(index);
			let set_vec = vec.clone().into_iter().unique().collect::<Vec<String>>();
			tag.set(set_vec);
			let emit = vec.clone().into_iter().unique().collect::<Vec<String>>().join(",");
			handle_onchange.clone().emit(emit);
		})
	}

	let remove = {
		let tag = tag.clone();
		let handle_onchange = onchange.clone();
		Callback::from(move|index|{
			let mut vec = tag.to_vec();
			vec.remove(index);
			let set_vec = vec.clone().into_iter().unique().collect::<Vec<String>>();
			tag.set(set_vec);
			let emit = vec.clone().into_iter().unique().collect::<Vec<String>>().join(",");
			handle_onchange.emit(emit);
		})
	};

	let tag = tag.clone().to_vec();
	html! {
		<div 
			class="flex font-sans text-sm py-2 px-3 items-center w-full gap-x-1 dark:bg-slate-800 dark:ring-0 dark:highlight-white/5 dark:text-slate-400 text-slate-500"
		>
			<Tag
				list={tag.clone().to_vec()}
				remove={remove}
			/>
			<input 
				class="bg-transparent flex flex-grow ring-1 ring-slate-900/10 focus:outline-none flex flex-grow tag-input" 
				onkeydown={keydown}
				onchange={input_onchange}
				ref={input_box}
			/>
		</div>
	}
}