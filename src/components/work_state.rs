use yew::{ html, Component, Context, Html, Properties, classes };

#[derive(PartialEq, Clone, Copy)]
pub enum TagProps {
	Bookmark,
	Work,
	Bug,
	Complete,
	Epic,
	Question,
	None,
}

impl TagProps {	
	fn get_icon(&self) -> &str {
		match self {
			TagProps::Bookmark => "ri-bookmark-fill",
			TagProps::Work => "ri-checkbox-fill",
			TagProps::Bug => "ri-bug-2-fill",
			TagProps::Complete => "ri-git-repository-line",
			TagProps::Epic => "ri-flashlight-fill",
			TagProps::Question => "ri-question-fill",
			TagProps::None => "",
		}
	}
}

#[derive(Properties, PartialEq)]
pub struct WorkStateProps {
	pub state: TagProps,
}

pub struct WorkState;

impl Component for WorkState {
	type Message = ();
	type Properties = WorkStateProps;

	fn create(_ctx: &Context<Self>) -> Self {
		Self
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		html! {
			<div class="">
				<i class={classes!(ctx.props().state.get_icon().to_string(), "text-xl", "dark:text-gray-200", "text-black")}></i>
			</div>
		}
	}
}