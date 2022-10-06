use serde::{Deserialize, Serialize};
use yew::{ html, Html, Properties, classes, function_component };
use crate::{components::{work_state::{ TagProps, WorkState }, profile_update::Position}, store::blog::Blog};
use quad_rand::ChooseRandom;

use crate::components::profile_update::ProfileUpdate;


#[derive(Default, Deserialize, Serialize, Debug, Clone, Properties, PartialEq)]
pub struct ProfileResponse {
	pub category:String,
  pub id:usize,
  pub tag:String,
  pub label:String,
  pub description:String,
  pub state:String,
  #[serde(rename="frDate")]
  pub fr_date:String,
  #[serde(rename="laDate")]
  pub la_date:String,
  #[serde(rename="createdAt")]
  pub created_at:String,
  #[serde(rename="updatedAt")]
  pub updated_at:String,
}


#[function_component(KanbanCard)]
pub fn kanban_card(ProfileResponse {id,tag,label,description,state,fr_date,la_date,created_at,updated_at, category } : &ProfileResponse) -> Html {
	let mut color = vec!["border-l-[#b4b4b4]", "border-l-[#22c55e]", "border-l-[#0ea5e9]", "border-l-[#8b5cf6]", "border-l-[#ec4899]", "border-l-[#ff5722]"];
	let value = ProfileResponse {id:id.clone(),tag:tag.clone(),label:label.clone(),description:description.clone(),state:state.clone(),fr_date:fr_date.clone(),la_date:la_date.clone(),created_at:created_at.clone(),updated_at:updated_at.clone(), category: category.clone() };
	ChooseRandom::shuffle(&mut color);
	let date = if fr_date.is_empty() && la_date.is_empty() {
		String::from("미정")
	} else {
		format!("{} ~ {}", fr_date, la_date)
	};
	let x: usize = quad_rand::gen_range(0, 6);
	let tag = tag.split(",");
	const DEFAULT: &str =
		"KanbanCard px-2 min-h-[200px] py-4 mx-2 shadow rounded-[3px] my-2 dark:bg-neutral-800 relative border-b-[#b4b4b4] border-r-[#b4b4b4] bg-white dark:border-b-[#100322] dark:border-t-[#100322] dark:border-r-[#100322] dark:border dark:border-l-4 border-l-4";
	html! {
		<div class={classes!(DEFAULT, color[x])}>
			<div class="flex">
				<div class="text-sm font-bold space-y-3 grow w-full">
					<div class="flex justify-between">
						<h2 class="flex-grow line-clamp-1 hover:text-clip hover:whitespace-nowrap text-xs"> {date} </h2>	
						<div class="w-10 flex flex-none justify-center">
							<img class="rounded-full w-5 h-5" src="https://avatars.githubusercontent.com/u/16170776?s=40&v=4" />
						</div>
					</div>
					<div class="overflow-hidden line-clamp-3"> { label } </div>
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
						<WorkState state={TagProps::None}/>
					</div>
					<div class="text-xs whitespace-pre-wrap">
						{description}
					</div>
				</div>
			</div>
			if Blog::is_admin() {
				<ProfileUpdate 
					position={Position::BottomRight}
					id={id.clone()}
					value={value.clone()}
				/>
			}
		</div>
	}
}