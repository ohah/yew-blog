use yew::{ html, Children, Component, Context, Html, Properties, classes };
use crate::components::work_state::{ TagProps, WorkState };
use quad_rand::ChooseRandom;


#[derive(Properties, PartialEq)]
pub struct KanbanCardProps {
	#[prop_or_default]
	pub children: Children,
	pub class: Option<String>,
	pub title: Option<String>,
	pub description: Option<String>,
	pub epic: Option<TagProps>,
	pub number: Option<i32>,
	pub tag: Vec<&'static str>,
}

pub struct KanbanCard;

impl Component for KanbanCard {
	type Message = ();
	type Properties = KanbanCardProps;

	fn create(_ctx: &Context<Self>) -> Self {
		Self
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		let mut color = vec!["border-l-[#b4b4b4]", "border-l-[#22c55e]", "border-l-[#0ea5e9]", "border-l-[#8b5cf6]", "border-l-[#ec4899]", "border-l-[#ff5722]"];
		ChooseRandom::shuffle(&mut color);
		let x: usize = quad_rand::gen_range(0, 6);
		const DEFAULT: &str =
			"KanbanCard px-2 min-h-[200px] py-4 mx-2 shadow rounded-[3px] my-2 dark:bg-neutral-800 relative border-b-[#b4b4b4] border-r-[#b4b4b4] bg-white dark:border-b-[#100322] dark:border-t-[#100322] dark:border-r-[#100322] dark:border dark:border-l-4 border-l-4";
		let tag = ctx.props().tag.clone().into_iter();
		let ctx = ctx.clone();
		html! {
			<div class={classes!(Some(&ctx.props().class), DEFAULT, color[x])}>
				<div class="flex">
					<div class="text-sm font-bold space-y-3 grow w-full">
						<div class="flex justify-between">
							<h2 class="flex-grow line-clamp-1 hover:text-clip hover:whitespace-nowrap text-xs"> { ctx.props().title.clone().unwrap_or_default() } </h2>	
							<div class="w-10 flex flex-none justify-center">
								<img class="rounded-full w-5 h-5" src="https://avatars.githubusercontent.com/u/16170776?s=40&v=4" />
							</div>
						</div>
						<div class="overflow-hidden line-clamp-3"> { ctx.props().description.clone().unwrap_or_default() } </div>
						<div class="flex flex-wrap gap-y-[5px] gap-x-[5px]">
						{
							tag.map(|row| {
								html!{
									<div class="text-ellipsis overflow-hidden inline-flex bg-slate-600 text-gray-100 inline-flex px-1 py-0.5 text-xs rounded"> {row} </div>
								}
							}).collect::<Html>()
						}
						</div>
						<div>
							<WorkState state={ctx.props().epic.clone().unwrap_or(TagProps::None)}/>
						</div>
						<div class="text-xs">
							{ for ctx.props().children.iter() } 
						</div>
					</div>
				</div>
			</div>
		}
	}
}