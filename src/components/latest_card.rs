use quad_rand::ChooseRandom;
use serde::{Deserialize};
use yew::{ html, Properties, function_component, classes, Html };
use crate::router::root::RootRoute;
use yew_router::{components::Link};

#[derive( Clone, PartialEq, Deserialize, Debug, Default, Properties)]
pub struct LatestProps {
	#[serde(rename="createdAt")]
	pub created_at:String,
	pub description:String,
	#[serde(rename="seoTitle")]
	pub seo_title:String,
	pub title:String,
	pub tag:String,
	#[serde(rename="updatedAt")]
	pub updated_at:String,
	pub avatar_url:String,
	pub name:String,
}

#[function_component(LatestCard)]
pub fn latset_card(LatestProps { title,seo_title,tag,created_at,updated_at,description, avatar_url, name }:&LatestProps) -> Html {
	let mut color = vec!["border-l-[#b4b4b4]", "border-l-[#22c55e]", "border-l-[#0ea5e9]", "border-l-[#8b5cf6]", "border-l-[#ec4899]", "border-l-[#ff5722]"];
	ChooseRandom::shuffle(&mut color);
	let x: usize = quad_rand::gen_range(0, 6);
	const DEFAULT: &str =
		"KanbanCard px-2 min-h-[200px] py-4 mx-2 shadow rounded-[3px] my-2 dark:bg-neutral-800 relative border-b-[#b4b4b4] border-r-[#b4b4b4] bg-white dark:border-b-[#100322] dark:border-t-[#100322] dark:border-r-[#100322] dark:border dark:border-l-4 border-l-4";
	let tag = tag.as_str().split(",");
	html! {
		<div class={classes!(DEFAULT, color[x])}>
			<div class="text-sm font-bold space-y-3 w-full h-full flex flex-col absolute inset-0 px-2 py-2">
				<div class="flex justify-between flex-none">
					<Link<RootRoute> to={RootRoute::View {seo_title : seo_title.to_string()}}>
						<h2 class="flex-grow line-clamp-1 hover:text-clip hover:line-clamp-3 ease-in-out trasition duration-100 space-x-2 dark:hover:text-white dark:hover:bg-black hover:bg-gray-300 hover:bg-opacity-75 z-50 hover:rounded"> 
							<span>{ title } </span>
						</h2>	
					</Link<RootRoute>>
					<div class="w-10 flex flex-none justify-center space-x-1 items-center">
						<img class="rounded-full w-5 h-5" src={avatar_url.clone()} />
					</div>
				</div>
				<div class="overflow-hidden line-clamp-5 flex-grow"> { description } </div>
				<div class="flex flex-wrap gap-y-[5px] gap-x-[5px]">
				{
					tag.map(|row| {
						html!{
							<div class="text-ellipsis overflow-hidden inline-flex bg-slate-200 text-gray-600 dark:bg-slate-600 dark:text-gray-100 inline-flex px-1 py-0.5 text-xs rounded"> { row } </div>
						}
					}).collect::<Html>()
				}
				</div>
				<div class="datetime text-xs flex justify-end w-full flex-col space-y-2 flex-none">
					<div class="flex justify-end w-full"> {created_at} </div>
				</div>
			</div>
		</div>
	}
}