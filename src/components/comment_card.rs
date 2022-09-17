use comrak::{markdown_to_html, ComrakOptions};
use web_sys::{MouseEvent,  Element};
use yew::{ html, function_component,use_state, Callback,use_node_ref, use_effect_with_deps };
use yew_router::prelude::{use_history, use_route, History};
use yewdux::prelude::use_store;
use crate::router::root::RootRoute;
use crate::store::blog::{DeletePayload, Blog};
use crate::{store::github_auth::GithubUser};

use super::comment::CommentListResponse;
use crate::components::comment_write::CommentWrite;
use crate::components::comment_reply_card::CommentReplyCard;
use crate::components::modal::Modal;

#[function_component(CommentCard)]
pub fn comment_card(CommentListResponse {id,wr_id,parent_id,content,avatar_url,name,created_at,updated_at,reply }:&CommentListResponse) -> Html {
	let (user, _) = use_store::<GithubUser>();
	let active_cmt = use_state(||false);
	let route = use_route::<RootRoute>().unwrap();
	let history = use_history().unwrap();
	let active_modify = use_state(|| false);
	let delete_modal = use_state(||false);
	let reply = reply.clone().unwrap();
	let render = use_node_ref();
	
	let activate = {
		let active_cmt = active_cmt.clone();
		Callback::from(move | e: MouseEvent| {
			e.prevent_default();
			active_cmt.set(true);
		})
	};

	{
		let render = render.clone();
		let active_modify = active_modify.clone();
		use_effect_with_deps(move|(render, content)| {
			if *active_modify == false {
				let content = content.clone();
				let render = render.clone();
				let render = render.cast::<Element>().expect("render을 가져오지 못함");
				let text = markdown_to_html(content.as_str(), &ComrakOptions::default());
				render.set_inner_html(text.as_str());
			}
			|| ()
		}, (render.clone(), content.clone()));
	}

	{
		let active_modify = active_modify.clone();
		use_effect_with_deps(move|history| {
			let listener = history.listen(move|| {
				active_modify.set(false);
			});
			move || { std::mem::drop(listener);}
		}, history.clone());
	}
	
	let modify_show = {
    let active_modify = active_modify.clone();
    Callback::from(move |e:MouseEvent| {
      e.prevent_default();
			active_modify.set(!*active_modify);
    })
  };
	let delete = {
    let route = route.clone();
		let history = history.clone();
		let id = id.clone();
		let delete_modal = delete_modal.clone();
    Callback::from(move |e:MouseEvent| {
      e.prevent_default();
      Blog::comment_remove( DeletePayload {
        id: id
      }, history.clone(), route.clone());
			delete_modal.set(false);
    })
  };

	let delete_close = {
    let delete_modal = delete_modal.clone();
    Callback::from(move |e:MouseEvent| {
      e.prevent_default();
      delete_modal.set(false);
    })
  };
	let delete_modal_open = {
		let delete_modal = delete_modal.clone();
		{
			Callback::from(move |e:MouseEvent| {
				e.prevent_default();
				delete_modal.set(true);
			})
		}
	};
	html! {
		<div class="rounded dark:bg-neutral-800 relative bg-white dark:border-[#100322] dark:border relative flex flex-col text-sm border-[#b4b4b4] border">
			<section class="flex flex-col gap-y-2 py-4">
				<header class="flex text-xs px-2 justify-between">
					<div class="group flex items-center gap-x-2 font-bold">
						<img src={format!("{}", avatar_url)} class="rounded-full w-5 h-5"/>
						<span> {name} </span>
						<span class="datetime"> {created_at} </span>
						<span class="rounded border dark:border-slate-600/50 px-1 py-0.5"> {"Owner"} </span>
					</div>
					<div class="flex gap-x-2">
						<button
							class="inline-flex items-center gap-x-1 dark:hover:text-white hover:text-black transition duration-75 ease-in-out"
							onclick={modify_show}
						>
							<i class="ri-delete-bin-line"></i>
							<span> {"수정하기"} </span>
						</button>
						<button 
							class="inline-flex items-center gap-x-1 dark:hover:text-white hover:text-black transition duration-75 ease-in-out"
							onclick={delete_modal_open}
						>
							<i class="ri-delete-bin-line"></i>
							<span> {"삭제하기"} </span>
						</button>
					</div>
				</header>
				if *delete_modal == true {
					<Modal
						is_close={delete_close.clone()}
					>
						<div class="flex flex-col dark:bg-slate-800 bg-white dark:shadow-none shadow w-80 py-5 px-5 rounded-lg space-y-2 modalIn gap-y-3">
							<h2 class="yg-jalnan t text-2xl"> {"삭제하기"} </h2>
							<div>
								{"해당 댓글을 삭제하시겠습니까?"}
							</div>
							<div class="flex justify-end">
								<div class="flex gap-x-2">
									<button
										class="inline-flex items-center text-white bg-pink-800 px-2 py-1 rounded hover:bg-pink-900 trasition duration-200 ease-in-out"
										onclick={delete}
									>
										<span> {"확인"} </span>
									</button>
									<button
										class="inline-flex items-center text-black bg-gray-300 px-2 py-1 rounded hover:bg-gray-400 trasition duration-200 ease-in-out"
										onclick={delete_close.clone()}
									>
										<span> {"취소"} </span>
									</button>
								</div>
							</div>
						</div>
					</Modal>
				}
				if *active_modify == true {
					<CommentWrite
						wr_id={wr_id.clone()}
						content={content.clone()}
						id={id.clone()}
					/>
				} else {
					<div 
						class="px-2 prose prose-sm max-w-none dark:prose-invert"
						ref={render}
					>
						{content}
					</div>
				}
				<footer class="flex justify-between items-center">
					<div class="px-2">
						// <button class="w-6 h-6 rounded-full border dark:border-slate-500/80 hover:bg-gray-100 dark:hover:bg-slate-600">
						// 	<i class="ri-emotion-line"></i>
						// </button>
					</div>
					<div class="px-2 text-xs">
						{ format!("{}개의 대댓글", reply.len()) }
					</div>
				</footer>
			</section>
			{
				match reply {
					reply => {
						let reply = reply.clone();
						if reply.len() > 0 {
							html! { 
								{ 
									for reply.iter().enumerate().map(|(i, row)| {
										let row = row.clone();
										html! {
											<CommentReplyCard 
												key={i} 
												..row 
											/>
										}
									})
								}
							}
						} else {
							html! {<></>}
						}
					}
					_ => html! {<></>}
				}
			}
			if *active_cmt == true {
				<CommentWrite
					wr_id = {wr_id.clone()}
					parent_id = {id.clone()}
				/>
			} else if !user.name.is_empty() {
				<div class="px-4 py-2 flex gap-x-4 items-center text-xs dark:bg-neutral-700 rounded-b border-t border-t-[#b4b4b4] dark:border-t-[#100322] w-full">
					<div class="flex flex-none ">
						<img class="w-5 h-5 rounded-full" src={format!("{}", user.avatar_url)} />
					</div>
					<textarea 
						onclick={activate}
						class="flex flex-grow dark:bg-[#1c1b1e] w-full h-8 rounded p-2 bg-gray-100 focus:outline-none text-xs focus:ring-0 outline outline-none resize-none overflow-hidden"
						placeholder="댓글을 작성하세요"
					/>
				</div>
			}
		</div>
	}
}