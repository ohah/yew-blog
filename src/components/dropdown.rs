use std::{rc::Rc, fmt::Display, process::Child};

use itertools::Itertools;
use serde::Deserialize;
use web_sys::MouseEvent;
use yew::{ html, html::{ChildrenRenderer}, create_portal, function_component, Children, Properties, html_nested, virtual_dom::{VChild, VNode}, Html, use_state, Callback, use_effect_with_deps, ChildrenWithProps };

use super::transition::Transition;

#[derive(Clone, derive_more::From, PartialEq, Debug)]
pub enum DropdownChildren {
	Transition(VChild<Transition>),
	Button(VChild<Dropdown<Button>>),
	// DropdownParent(ChildrenWithProps<Dropdown<Parent>>),
	Children(VNode)
}

impl DropdownChildren {
	pub fn is_button(&self) -> bool {
		match self {
			Self::Button(child) => true,
			_ => false,
		}
	}
	pub fn is_transition(&self) -> bool {
		match self {
			Self::Transition(child) => true,
			_ => false,
		}
	}
	pub fn is_children(&self) -> bool {
		match self {
			Self::Children(child) => true,
			_ => false,
		}
	}
}

#[allow(clippy::from_over_into)]
impl Into<Html> for DropdownChildren {
	fn into(self) -> Html {
		match self {
			Self::Button(child) => child.into(),
			Self::Transition(child) => html!{
				<Transition 
					enter="transition ease-in-out duration-300 transform"
					enter_from="scale-y-0"
					enter_to="scale-1"
					leave="transition ease-in-out duration-300 transform"
					leave_from="scale-y-1"
					leave_to="scale-y-0"
				>
					<div> {"음?"} </div>
				</Transition>
			},
			// child.into(),
			Self::Children(child) => child.into(),
			// Self::DropdownParent(child) => child.into(),
		}
	}

}

pub type Button = bool;
pub type Parent = bool;

#[derive(Clone, Properties, PartialEq)]
pub struct DropdownProps<T> where T:PartialEq, {
	#[prop_or_default]
	pub children: ChildrenRenderer<DropdownChildren>,
	pub is_active: Option<T>,
}


pub fn type_of<T>(_: T) -> &'static str {
	std::any::type_name::<T>()
}

#[function_component(Dropdown)]
pub fn dropdown<T>(DropdownProps { children, is_active }: &DropdownProps<T>) -> Html where T: PartialEq + Display{
	let show = use_state(||false);
	// let is_active = is_active.as_ref();
	let sex:Parent = true;
	let _type = type_of(sex);
	log::info!("{:?}", _type);
	// let button = children.clone().into_iter().filter(|item| item.clone().is_button());
	// children.clone().into_iter().filter(|item| item.clone().is_transition()).map(|mut item| {
	// 	// item.props
	// });
	let toggle = {
		let show = show.clone();
		Callback::from(move|e:MouseEvent| {
			show.set(!*show);
		})
	};
	// log::info!("{:?}", button);
	html! {
		<div>
			{for children.iter().map(|row|{
				if row.clone().is_button() {
					html!{<>{row}</>}
				} else {
					html!{<> </>}
				}
			})}
			// <div 
			// 	onclick={toggle}
			// >
				// {
				// 	for children.clone().iter().filter(|item| item.clone().is_button()).map(|item| {
				// 		html! {
				// 			<div> {item.clone()} </div>
				// 		}
				// 		// item
				// 	})
				// }
			// </div>
			// {				
			// 	for children.clone().into_iter().filter(|item| item.clone().is_transition()).map(|mut item| {
					// let item = item as VChild<Transition>;
					// let mut props = Rc::make_mut(&mut item.props);
					// props.show = show;
			// 		item
			// 	})
			// }
			// if *show == true {
				// {
				// 	for children.clone().into_iter().filter(|item| item.clone().is_children()).map(|item| {
				// 		item
				// 	})
				// }
			// }
			// <div class={format!("origin-top-right absolute top-7 right-0 mt-2 w-36 rounded-md shadow-lg bg-white dark:bg-slate-800 dark:shadow-none shadow dark:text-slate-300 text-gray-700 ring-1 ring-black ring-opacity-5 focus:outline-none {}", "")}>
				// { 
				// 	for children.iter().map(|mut item| {
				// 		let mut props = Rc::make_mut(&mut item.props);
				// 		props.value = format!("item-{}", props.value);
				// 		item
				// 	})
				// }	
			// </div>
		</div>
	}
}