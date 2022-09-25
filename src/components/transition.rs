use web_sys::MouseEvent;
use regex::Regex;
use yew::{ html, html::{ChildrenRenderer}, create_portal, function_component, Children, Properties, html_nested, virtual_dom::{VChild, VNode}, Html, use_state, Callback, UseStateHandle, use_effect_with_deps, classes };

#[derive(Properties, PartialEq)]
pub struct TransitionProps {
	pub show:Option<bool>,
	#[prop_or_default]
	pub children: Children,	
	pub enter:&'static str,
	pub enter_from:&'static str,
	pub enter_to:&'static str,
	pub leave:&'static str,
	pub leave_from:&'static str,
	pub leave_to:&'static str,
	pub class:Option<String>,
	pub callback:Option<Callback<String>>,
}

#[function_component(Transition)]
pub fn transition(TransitionProps {class, callback, children,enter,enter_from,enter_to,leave,leave_from,leave_to, show }: &TransitionProps) -> Html {
	let toggle = use_state(||false);
	let enter_timer = use_state(|| false);
	let leave_timer = use_state(|| false);
	let timer = use_state(||"enter");
	let end = use_state(||true);
	let regx = Regex::new(r"duration-((\[|)([0-9]+)(ms|)(\]|))").unwrap();
	let leave_ms = regx.clone().captures(leave).unwrap();
	let leave_ms = leave_ms.get(3).unwrap().as_str();
	let show = match show {
		Some(show) => show,
		None => &false
	};
	{
		let toggle = toggle.clone();
		let enter_timer = enter_timer.clone();
		let leave_timer = leave_timer.clone();
		let show = show.clone();
		let end = end.clone();
		let timer = timer.clone();
		use_effect_with_deps(move|show|{
			let show = show.clone();
			let toggle = toggle.clone();
			let enter_timer = enter_timer.clone();
			let leave_timer = leave_timer.clone();
			if show == true {
				timer.set("enter");
				leave_timer.clone().set(false);
				enter_timer.clone().set(true);
				gloo::timers::callback::Timeout::new(0, move || {
					toggle.set(true);
					let enter_timer = enter_timer.clone();
					enter_timer.clone().set(false);
				})
				.forget();
			} else if show == false  && *toggle == true {
				timer.set("leave");
				end.set(false);
				leave_timer.clone().set(true);
				enter_timer.clone().set(false);
				gloo::timers::callback::Timeout::new(0, move || {
					let leave_timer = leave_timer.clone();
					toggle.set(false);
					leave_timer.clone().set(false);
				})
				.forget();
			}
			|| ()
		}, show.clone());
	}

	{
		let callback = callback.clone();
		let end = end.clone();
		let leave_ms = leave_ms.clone().to_string().parse::<u32>().unwrap_or(300);
		use_effect_with_deps(move|timer| {
			let timer = timer.clone();
			if *timer == "leave" {
				gloo::timers::callback::Timeout::new(leave_ms, move || {
					end.set(true);
					match callback {
						Some(callback) => {
							callback.emit("".to_string());
						}
						None => {}
					}
				}).forget();
			}
			|| ()
		}, timer.clone());
	}

	let toggle_click = {
		let toggle = toggle.clone();
		Callback::from(move|e:MouseEvent|{
			e.prevent_default();
			toggle.set(!*toggle);
		})
	};

	let enter_animation = if *enter_timer == true && *leave_timer == false && show.clone() == true {
		Some(enter_from.clone())
	} else if *enter_timer == false && *leave_timer == false && show.clone() == true {
		Some(enter_to.clone())
	} else {
		None
	};
	let leave_animation = if *leave_timer == true && *enter_timer == false && show.clone() == false {
		Some(leave_from.clone())
	} else if *leave_timer == false && *enter_timer == false && show.clone() == false {
		Some(leave_to.clone())
	} else {
		None
	};
	let enter_class = if *enter_timer == true {
		Some(enter.clone())
	} else {
		Some(leave.clone())
	};

	html! {
		if *show == true || *end.clone() == false {
			<div class={classes!(enter_class, enter_animation, leave_animation.clone(), class)}>
				<div class={format!("{}", "")}>
					{ for children.iter() }	
				</div>
			</div>
		}
	}
}