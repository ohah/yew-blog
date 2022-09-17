use gloo_net::http::Request;
use serde::{Serialize, Deserialize};
use yew::{ html, Properties, function_component, classes,  use_state, use_effect_with_deps };
use yew_router::{prelude::{use_history, History}};
use crate::{store::{toast::ToastStatus, blog::Blog}};
use crate::components::comment_write::CommentWrite;
use crate::components::comment_card::CommentCard;

#[derive(Default, Deserialize, Serialize, Debug, Clone, Properties, PartialEq)]
pub struct CommentListResponse {
	pub id:usize,
	pub wr_id:usize,
	pub parent_id:Option<usize>,
	pub content:String,
	pub avatar_url:String,
	pub name:String,
	#[serde(rename="createdAt")]
	pub created_at:String,
	#[serde(rename="updatedAt")]
	pub updated_at:String,
	pub reply:Option<Vec<CommentListResponse>>,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq, Default, Debug)]
pub struct CommentProps {
	pub wr_id:usize
}
#[function_component(Comment)]
pub fn comment(CommentProps { wr_id }:&CommentProps) -> Html {
	let lists = use_state(||vec![]);
	let history = use_history().unwrap();
	{
		let lists = lists.clone();
		let url = format!("/api/comment_list/{}", wr_id.clone());
		use_effect_with_deps(move |wr_id| {
			if wr_id > &0 {
				wasm_bindgen_futures::spawn_local(async move {        
					let fetched_list  = Request::get(url.as_str())
						.header("accept", "application/json")
						.header("Access-Control-Allow-Origin", "no-cors")
						.send()
						.await
						.unwrap()
						.json::<Vec<CommentListResponse>>()
						.await;
					match fetched_list {
						Ok(fetched_list) => {
							lists.set(fetched_list);
						}
						Err(err) => {
							log::info!("Error: {:?}", err);
							Blog::toast_message("댓글을 불러오는 과정에서 오류가 발생헀습니다", ToastStatus::DANGER, None);
						}
					};
				});
			}
			|| ()
		}, wr_id.clone());
	}

  {
    let history = history.clone();
		let lists = lists.clone();
		let wr_id = wr_id.clone();
    use_effect_with_deps(move |(history, wr_id)| {
			let history = history.clone();
			let wr_id = wr_id.clone();
			let listener = history.listen(move || {
				let lists = lists.clone();
				let wr_id = wr_id.clone();
				let url = format!("/api/comment_list/{}", wr_id.clone());
				wasm_bindgen_futures::spawn_local(async move {        
					let fetched_list  = Request::get(url.as_str())
						.header("accept", "application/json")
						.header("Access-Control-Allow-Origin", "no-cors")
						.send()
						.await
						.unwrap()
						.json::<Vec<CommentListResponse>>()
						.await;
					match fetched_list {
						Ok(fetched_list) => {
							lists.set(fetched_list);
						}
						Err(err) => {
							log::info!("Error: {:?}", err);
							Blog::toast_message("댓글을 불러오는 과정에서 오류가 발생헀습니다.", ToastStatus::DANGER, None);
						}
					};
				});
			});

			move || { std::mem::drop(listener);}
		},(history.clone(), wr_id.clone()));
  }

	const DEFAULT: &str = "Cmt-List mb-8";
	html! {
		<div class={classes!(DEFAULT)}>
			<CommentWrite 
				wr_id = {wr_id.clone()}
			/>
			<section class="pt-4 flex flex-col gap-y-4">
				{ 
					for lists.iter().enumerate().map(|(i, row)| {
						let row = row.clone();
						html! {
							<CommentCard 
								key={i}
								..row 
							/>
						}
					})
				}
			</section>
		</div>
	}
}