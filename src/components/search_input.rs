use wasm_bindgen::JsCast;
use yew::{ html, function_component, Properties, Callback, use_state, UseStateHandle, use_state_eq, use_node_ref, use_effect_with_deps, use_effect };
use web_sys::{KeyboardEvent, HtmlInputElement, MouseEvent, HtmlElement, Element};
use itertools::Itertools;
use web_sys::Event;

#[derive(Properties, PartialEq)]
pub struct SearchInputProps {
	pub default_value:Option<String>,
	pub onchange:Callback<Event>
}

#[function_component(SearchInput)]
pub fn search_input(props: &SearchInputProps) -> Html {
	
	html! {
		<div 
			class="flex font-sans text-sm py-2 px-3 items-center w-full gap-x-1 dark:bg-slate-800 dark:ring-0 dark:highlight-white/5 dark:text-slate-400 text-slate-500"
		>
		</div>
	}
}