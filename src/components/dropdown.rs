use std::{rc::Rc};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{MouseEvent, Element };
use yew::{ html, html::{ChildrenRenderer}, {events::Event}, function_component, Children, Properties, html_nested, virtual_dom::{VChild, VNode}, Html, use_state, Callback, use_effect_with_deps, ChildrenWithProps, Component, Context, NodeRef };
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
	pub children: ChildrenWithProps<Transition>,
	// pub children: Children,
	pub button: Html,
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

pub struct Dropdown {
	is_show:bool,
	close_listener: Option<Closure<dyn Fn(Event)>>,
	body:Element,
	node: NodeRef,
}

pub enum Toggle {
	Hide,
	Show,
	Close,
}

impl Component for Dropdown {
	type Message = Toggle;
	type Properties = DropdownProps;

	fn create(ctx: &Context<Self>) -> Self {
		Self { 
			is_show: false,
			close_listener: None,
			body:gloo_utils::document().query_selector("body").expect("body element failed").unwrap(),
			node: NodeRef::default(),
		}
	}

	fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
		let outside = {
			ctx.link().callback(move|e:Event| {
				Toggle::Close
			})
		};
		match msg {
			Toggle::Hide => {
				self.is_show = false;
				if let Some(listener) = self.close_listener.take() {
					self.body.remove_event_listener_with_callback( "click", listener.as_ref().unchecked_ref()).unwrap();
					self.close_listener = None;
				}
				true
			}
			Toggle::Show => {
				self.is_show = true;
				let listener = Closure::<dyn Fn(Event)>::wrap({
					Box::new(move |e: Event| {
						outside.emit(e);
					})
				});
				let function = listener.as_ref().unchecked_ref();
				self.body.add_event_listener_with_callback("click", function).unwrap();
				self.close_listener = Some(listener);
				// log::info!("오픈.");
				// if let Some(node) = self.node.cast::<Element>().take() {
				// 	if let input = node.query_selector("input, textarea") {
				// 		match input {
				// 			Ok(input) => {
				// 				input.unwrap();
				// 			},
				// 			Err(err) => {}

				// 		}
				// 		// input.unwrap().dyn_ref::<HtmlInputElement>().unwrap().focus();
				// 	}
				// }
				true
			}
			Toggle::Close => {
				match self.is_show {
					true => {
						self.is_show = false;
						if let Some(listener) = self.close_listener.take() {
							self.body.remove_event_listener_with_callback( "click", listener.as_ref().unchecked_ref()).unwrap();
							self.close_listener = None;
						}
						true
					},
					false => {
						false
					}
				}
			}
		}
	}

	fn changed(&mut self, ctx: &Context<Self>) -> bool {
		true
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		let is_show = self.is_show;
		let bubble_stop = {
			Callback::from(move |e:MouseEvent| {
				e.stop_propagation();
			})
		};
		let toggle = {
			let is_show = is_show.clone();
			ctx.link().callback(move|e:MouseEvent| {
				e.stop_propagation();
				match is_show {
					true => Toggle::Hide,
					false => Toggle::Show
				}
			})
		};
		html! {
			<div 
				onclick={bubble_stop}
				class="relative"
				ref={self.node.clone()}
			>
				<div onclick={toggle}>{ctx.props().clone().button}</div>
				{
					for ctx.props().children.iter().map(|mut item| {
						let mut props = Rc::make_mut(&mut item.props);
						props.show = Some(is_show);
						item
					})
				}
			</div>
		}
	}

	fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
		if !first_render {
			return;
		}
	}

	fn destroy(&mut self, ctx: &Context<Self>) {
	}
}


// #[function_component(Dropdown)]
// // pub fn dropdown<T>(DropdownProps { children, is_active }: &DropdownProps<T>) -> Html where T: PartialEq + Display{
// pub fn dropdown(DropdownProps { children, header }: &DropdownProps) -> Html {
// 	let show = use_state(||false);
// 	children.iter().map(|mut item| {
// 		// let mut props = Rc::make_mut(&mut item);
// 		log::info!("{:?}", item);
// 	});
// 	let toggle = {
// 		let show = show.clone();
// 		Callback::from(move|e:MouseEvent| {
// 			show.set(!*show);
// 		})
// 	};
// 	let button_children = children.iter().filter(|row| {
// 		log::info!("row {:?}", row);
// 		true
// 	});
// 	// log::info!("button_children {:?}", button_children);
// 	html! {
// 		<div>

// 			<div onclick={toggle}>{header.clone()}</div>
// 			// {for children.iter()}
// 			<div class={format!("origin-top-right absolute top-7 right-0 mt-2 w-36 rounded-md shadow-lg bg-white dark:bg-slate-800 dark:shadow-none shadow dark:text-slate-300 text-gray-700 ring-1 ring-black ring-opacity-5 focus:outline-none {}", "")}>
// 				{ 
// 					for children.iter().map(|mut item| {
// 						// let mut props = Rc::make_mut(&mut item);
// 						{log::info!("{:?}", item)}
// 						// props.value = format!("item-{}", props.value);
// 						item
// 					})
// 				}	
// 			</div>
// 		</div>
// 	}
// }