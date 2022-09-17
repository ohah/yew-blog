use gloo_net::http::Request;
use serde::{Deserialize};
use yew::{ function_component, html, use_effect_with_deps, use_state, Properties,};
use crate::components::latest_card::{LatestProps, LatestCard};
use crate::store::blog::Blog;
use crate::store::toast::{ ToastStatus};
use crate::{components::card::Card};


#[derive( Clone, PartialEq, Deserialize, Debug, Default, Properties)]
pub struct Latest {
	pub cnt:usize,
	pub category:String,
	pub list:Vec<LatestProps>
}

#[function_component(Home)]
pub fn home() -> Html {
	let lists = use_state(||vec![Latest::default()]);
	{
		let lists = lists.clone();
		use_effect_with_deps(move |_| {
			wasm_bindgen_futures::spawn_local(async move {        
				let fetched_list = Request::get("/api/latest")
					.header("accept", "application/json")
					.header("Access-Control-Allow-Origin", "no-cors")
					.send()
					.await
					.unwrap()
					.json::<Vec<Latest>>()
					.await;
					match fetched_list {
						Ok(fetched_list) => {
							lists.set(fetched_list);
						}
						Err(err) => {							
							log::error!("{:?}", err);
							Blog::toast_message("알 수 없는 오류가 발생헀습니다.", ToastStatus::DANGER, None);
						}
					};
			});
			|| ()
		}, ());
	}
	html! {
    <article class="grid mt-2">
      <Card 
        title="안내"
      >        
				<p>{ "개인 개발자의 개발 블로그입니다" }</p>
      </Card>
      <Card 
        title="최신글"
      >
        <div class="grid grid-cols-3 gap-x-4">
					{ 
						for lists.iter().enumerate().map(|(i, row)| {
							let row = row.clone();
							html! {
								<div key={i} class="space-y-2"> 
									<h2 class="font-bold"> {row.category} </h2>
									<div class="w-full bg-gray-100 rounded-full h-2 dark:bg-gray-700"></div>
								</div>
							}
						})
					}
          <div class="flex items-center w-full py-3 col-span-3 space-x-2">
            <button class="text-xl w-5 h-5 inline-flex items-center justify-center bg-gray-100 hover:bg-gray-200 rounded dark:bg-slate-700 dark:hover:bg-slate-800 ease-in-out duration-200 "><i class="ri-arrow-down-s-fill"></i></button>
            <span> {format!("{}개의 인기 있는 카테고리", lists.len())} </span>
          </div>
					{ 
						for lists.iter().enumerate().map(|(i, row)| {
							let row = row.clone();
							html! {
								<div key={i} class="bg-gray-100 dark:bg-slate-700 min-h-[300px] max-h-[630px] overflow-y-auto rounded scrollbar-thin dark:scrollbar-thumb-gray-900 dark:scrollbar-track-gray-800">
									{
										for row.list.iter().enumerate().map(|(k, data)| {
											let data = data.clone();
											html!{ 
												<LatestCard 
													key={k}
													..data 
												/>
											}
										})
									}
								</div>
							}
						})
					}
        </div>
      </Card>
    </article>
  }
}