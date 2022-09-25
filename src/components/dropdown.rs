use std::{rc::Rc, fmt::Display, process::Child};

use itertools::Itertools;
use serde::Deserialize;
use web_sys::MouseEvent;
use yew::{ html, html::{ChildrenRenderer}, create_portal, function_component, Children, Properties, html_nested, virtual_dom::{VChild, VNode}, Html, use_state, Callback, use_effect_with_deps, ChildrenWithProps };

use super::transition::Transition;

#[derive(Clone, derive_more::From, PartialEq, Debug)]
pub enum DropdownChildren {
	Transition(VChild<Transition>),
	// Button(VChild<Dropdown<Button>>),
	// Body(VChild<Dropdown<Body>>),
	// DropdownParent(ChildrenWithProps<Dropdown<Parent>>),
	Children(VNode)
}

pub enum Attr {
	Button,
	Body,
	Children, 
	Parent
}

impl Attr {
	fn as_str(&self) -> &'static str {
		match self {
			Attr::Button => "Button",
			Attr::Body => "Body",
			Attr::Children => "Children",
			Attr::Parent => "Parent"
		}
	}
}
impl DropdownChildren {
	// pub fn is_button(&self) -> bool {
	// 	match self {
	// 		Self::Button(child) => true,
	// 		_ => false,
	// 	}
	// }
	// pub fn is_transition(&self) -> bool {
	// 	match self {
	// 		Self::Transition(child) => true,
	// 		_ => false,
	// 	}
	// }
	// pub fn is_children(&self) -> bool {
	// 	match self {
	// 		Self::Children(child) => true,
	// 		_ => false,
	// 	}
	// }
	// pub fn is_body(&self) -> bool {
	// 	match self {
	// 		Self::Body(child) => true,
	// 		_ => false,
	// 	}
	// }
}

#[allow(clippy::from_over_into)]
impl Into<Html> for DropdownChildren {
	fn into(self) -> Html {
		match self {
			// Self::Button(child) => child.into(),
			// Self::Body(child) => child.into(),
			Self::Transition(child) => child.into(),
			// child.into(),
			Self::Children(child) => child.into(),
		}
	}

}

pub type Parent = usize;
pub type Button = u8;
pub type Body = u16;

#[derive(Clone, Properties, PartialEq)]
// pub struct DropdownProps<T> where T:PartialEq, {
pub struct DropdownProps {
	#[prop_or_default]
	// pub children: ChildrenRenderer<DropdownChildren>,
	pub children: Children,
	pub header: Html,
	// pub is_active: Option<T>,
}


pub fn type_of<T>(_: T) -> &'static str {
	let typename = std::any::type_name::<T>();
	match typename {
		"&core::option::Option<u8>" => "Button",
		"&core::option::Option<u18>" => "Body",
		_ => "Parent"
	}
}

#[function_component(Dropdown)]
// pub fn dropdown<T>(DropdownProps { children, is_active }: &DropdownProps<T>) -> Html where T: PartialEq + Display{
pub fn dropdown(DropdownProps {children, header }: &DropdownProps) -> Html {
	let show = use_state(||false);
	// let is_active = is_active.as_ref();
	// let attr = type_of(is_active);
	// log::info!("{:?}", attr);
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
	let button_children = children.iter().filter(|row| {
		log::info!("row {:?}", row);
		true
	});
	// log::info!("button_children {:?}", button_children);
	html! {
		<div>

			<div onclick={toggle}>{header.clone()}</div>
			// {for children.iter()}
			<div class={format!("origin-top-right absolute top-7 right-0 mt-2 w-36 rounded-md shadow-lg bg-white dark:bg-slate-800 dark:shadow-none shadow dark:text-slate-300 text-gray-700 ring-1 ring-black ring-opacity-5 focus:outline-none {}", "")}>
				{ 
					for children.iter().map(|mut item| {
						// let mut props = Rc::make_mut(&mut item);
						{log::info!("{:?}", item)}
						// props.value = format!("item-{}", props.value);
						item
					})
				}	
			</div>
		</div>
	}
}