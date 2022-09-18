use wasm_bindgen::JsCast;
use yew::{ html, function_component, Properties, Callback, use_state, UseStateHandle, use_state_eq, use_node_ref, use_effect_with_deps, use_effect };
use web_sys::{KeyboardEvent, HtmlInputElement, MouseEvent, HtmlElement, Element};
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
	let input_box = use_node_ref();
	let keyup = {
		let tag = tag.clone();
		Callback::from(move |e:KeyboardEvent| {
			e.prevent_default();
			let input = e.target().unwrap().unchecked_into::<HtmlInputElement>();
			let value = input.clone().value();
			let key = e.key();
			let key = key.as_str();
			let tag = tag.clone();
			if key == " " || key == "," {
				// log::info!("태그 분리, {:?}", key);
				// log::info!("state, {:?}", tag);
				let mut vec = tag.to_vec();
				vec.push(value.replace(",", "").trim().to_string());
				vec.dedup_by(|a, b| a == b);
				let vec = vec.clone().into_iter().unique().collect();
				tag.set(vec);
				input.clone().focus();
				log::info!("{:?}", input);
				input.clone().set_value("");
				gloo::timers::callback::Timeout::new(1000, move || {
					log::info!("실행");
					input.clone().focus();
				})
				.forget();
			}
		})
	};
	// {
	// 	let input = input.clone();
	// 	use_effect(move || {
	// 		let input = input.cast::<HtmlInputElement>().expect("rendering error");
	// 		input.focus();
	// 		log::info!("유즈이펙트");
	// 		|| ()
	// 	});ㅕ
	// }
	{
		let input_box = input_box.clone();
		use_effect_with_deps(move|(tag, input_box)| {
			let input_box = input_box.clone();
			let element = input_box.cast::<HtmlInputElement>().expect("render을 가져오지 못함");
			log::info!("태그변경될때마다");
			element.focus();
			|| ()
		}, (tag.clone(), input_box.clone()));
	}

	fn remove (tag:UseStateHandle<Vec<String>>, index:usize) -> Callback<MouseEvent> {
		Callback::from(move|e:MouseEvent| {
			e.prevent_default();
			let mut vec = tag.to_vec();
			vec.remove(index);
			tag.set(vec);
		})
	}

	let focus = {
		let input_box = input_box.clone();
		Callback::from(move |e:MouseEvent| {
			log::info!("포카스 이벤트 실행");
			input_box.cast::<HtmlInputElement>().expect("input 에러").focus();
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
							<span onclick={focus.clone()} class="inline-flex">{_tag}</span>
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
				class="bg-transparent flex flex-grow ring-1 ring-slate-900/10 focus:outline-none flex flex-grow tag-input" 
				onkeyup={keyup}
				ref={input_box}
			/>
		</div>
	}
}