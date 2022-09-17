use yew::{ html, Children, Component, Context, Html, Properties, classes };

#[derive(Properties, PartialEq)]
pub struct KanbanProps {
	#[prop_or_default]
	pub children: Children,
	pub class: Option<String>,
	pub title: Option<String>,
}

pub struct Kanban;

impl Component for Kanban {
	type Message = ();
	type Properties = KanbanProps;

	fn create(_ctx: &Context<Self>) -> Self {
		Self
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		const DEFAULT: &str =
			"Kanban px-2 py-4 shadow rounded-[6px] my-2 dark:border-[#100322] border dark:bg-neutral-800 relative border-b-[#b4b4b4] border-r-[#b4b4b4] border-l-[#b4b4b4]";
		html! {
			<div class={classes!(Some(&ctx.props().class), DEFAULT)}>
				<header class="h-[35px] border-b border-t rounded-t-[6px] flex justify-between w-full text-[11pt] absolute top-0 left-0 bg-[#ebebeb] border-[#d8d8d8] border-b-[#b4b4b4] dark:bg-[#1c1b1e] dark:border-[#100322]">
					<div class="flex items-center space-x-[6px] ml-[6px]">
						<button class="group w-[11px] h-[11px] bg-[#ff5c5c] border border-[#e33e41] rounded-full text-[9pt] inline-flex items-center justify-center">
							<i class="ri-close-fill hidden group-hover:block text-black font-bold"></i>
						</button>
						<button class="group w-[11px] h-[11px] bg-[#ffbd4c] border border-[#e09e3e] rounded-full text-[9pt] inline-flex items-center justify-center">
							<i class="ri-add-fill hidden group-hover:block text-black font-bold"></i>
						</button>
						<button class="group w-[11px] h-[11px] bg-[#00ca56] border border-[#14ae46] rounded-full text-[9pt] inline-flex items-center justify-center">
							<i class="ri-subtract-fill hidden group-hover:block text-black font-bold"></i>
						</button>
					</div>
					<div class="w-full justify-center flex items-center text-xl font-bold">
						{ ctx.props().title.clone().unwrap_or_default() }
					</div>
				</header>
				<div class="mt-8 text-sm">
					{ for ctx.props().children.iter() }
				</div>
			</div>
		}
	}
}