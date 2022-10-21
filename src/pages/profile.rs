use gloo_net::http::Request;
use yew::{ function_component, html, use_state, use_effect_with_deps };
use yew_router::prelude::{use_history, History};
use crate::components::card;
use card::Card;
use crate::components::kanban_card::{self, ProfileResponse};
use crate::components::profile_update::ProfileUpdate;
use kanban_card::KanbanCard;


#[function_component(Profile)]
pub fn profile() -> Html {
  let work_list = use_state(||Vec::new());
  let history = use_history().unwrap();
  gloo_utils::document().set_title("Ohah의 블로그 - 프로필");
  {
    let work_list = work_list.clone();
    use_effect_with_deps(move |_| {
      let work_list = work_list.clone();
      wasm_bindgen_futures::spawn_local(async move {
        let fetched_list  = Request::get("api/profile_list")
        .header("accept", "application/json")
        .header("Access-Control-Allow-Origin", "no-cors")
        .send()
        .await
        .unwrap()
        .json::<Vec<ProfileResponse>>()
        .await
        .unwrap();
        work_list.set(fetched_list.clone());
      });
    || ()
    }, ());
  }

  {
    let history = history.clone();
    let work_list = work_list.clone();
    use_effect_with_deps(move |history| {
      let history = history.clone();
      let work_list = work_list.clone();
			let listener = history.listen(move || {
        let work_list = work_list.clone();
				wasm_bindgen_futures::spawn_local(async move {
          let fetched_list  = Request::get("api/profile_list")
          .header("accept", "application/json")
          .header("Access-Control-Allow-Origin", "no-cors")
          .send()
          .await
          .unwrap()
          .json::<Vec<ProfileResponse>>()
          .await
          .unwrap();
          work_list.set(fetched_list);
        });
      });
			move || { std::mem::drop(listener);}
		},history.clone());
  }

	html! {
    <article class="grid mt-2">
      <Card 
      title="소개"
      >        
        <p>{ "React, Vue, Typescript를 주력으로 개발하는 개발자 입니다" }</p>
        <p>{ "여러가지 웹기술에 관심이 많으며 특히 Wasm, Rust에 관심이 많습니다"} </p>
        <p>{ "이 페이지도 WebAssmbly로 제작되었습니다!"} </p>
      </Card>
      <Card 
        title="기술"
      > 
        <div class="space-y-3">
          <div class="flex flex-wrap gap-x-2 gap-y-2">
            <div class="yg-jalnan text-xs px-2 py-2 rounded-lg border-2 dark:border-slate-600 border-gray-400 flex items-center gap-x-1"> 
              <span> {"Javascript"} </span>
            </div>
            <div class="yg-jalnan text-xs px-2 py-2 rounded-lg border-2 dark:border-slate-600 border-gray-400 flex items-center gap-x-1"> 
              <span>{"Typescript"}</span>
            </div>
            <div class="yg-jalnan text-xs px-2 py-2 rounded-lg border-2 dark:border-slate-600 border-gray-400 flex items-center gap-x-1"> 
              <span>{"CSS3"}</span>
            </div>
            <div class="yg-jalnan text-xs px-2 py-2 rounded-lg border-2 dark:border-slate-600 border-gray-400 flex items-center gap-x-1">               
              <span>{"HTML"}</span>
            </div>
            <div class="yg-jalnan text-xs px-2 py-2 rounded-lg border-2 dark:border-slate-600 border-gray-400 flex items-center gap-x-1"> 
              <span>{"PHP"}</span>
            </div>
            <div class="yg-jalnan text-xs px-2 py-2 rounded-lg border-2 dark:border-slate-600 border-gray-400 flex items-center gap-x-1"> 
              <span>{"Rust"}</span>
            </div>
            <div class="yg-jalnan text-xs px-2 py-2 rounded-lg border-2 dark:border-slate-600 border-gray-400 flex items-center gap-x-1"> 
              <span>{"React"}</span>
            </div>
            <div class="yg-jalnan text-xs px-2 py-2 rounded-lg border-2 dark:border-slate-600 border-gray-400 flex items-center gap-x-1">               
              <span>{"Next.js"}</span>
            </div>
            <div class="yg-jalnan text-xs px-2 py-2 rounded-lg border-2 dark:border-slate-600 border-gray-400 flex items-center gap-x-1"> 
              <span>{"Vue.js(2,3)"}</span>
            </div>
            <div class="yg-jalnan text-xs px-2 py-2 rounded-lg border-2 dark:border-slate-600 border-gray-400 flex items-center gap-x-1"> 
              <span>{"Webpack"}</span>
            </div>
            <div class="yg-jalnan text-xs px-2 py-2 rounded-lg border-2 dark:border-slate-600 border-gray-400 flex items-center gap-x-1"> 
              <span>{"Vite"}</span>
            </div>
          </div>
          // <div class="space-y-1">
          //   <h2 class="font-semibold flex justify-between w-full">
          //     <span>{ "Javascript" }</span>
          //     <span>{ "75%" }</span>
          //   </h2>
          //   <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
          //     <div class="bg-blue-600 h-2.5 rounded-full relative" style="width: 75%"></div>
          //   </div>
          // </div>
          // <div class="space-y-1">
          //   <h2 class="font-semibold flex justify-between w-full">
          //     <span>{ "Typescript" }</span>
          //     <span>{ "76.5%" }</span>
          //   </h2>
          //   <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
          //     <div class="bg-blue-600 h-2.5 rounded-full relative" style="width: 76.5%"></div>
          //   </div>
          // </div>
          // <div class="space-y-1">
          //   <h2 class="font-semibold flex justify-between w-full">
          //     <span>{ "PHP" }</span>
          //     <span>{ "70%" }</span>
          //   </h2>
          //   <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
          //     <div class="bg-blue-600 h-2.5 rounded-full relative" style="width: 70%"></div>
          //   </div>
          // </div>
          // <div class="space-y-1">
          //   <h2 class="font-semibold flex justify-between w-full">
          //     <span>{ "Rust" }</span>
          //     <span>{ "50%" }</span>
          //   </h2>
          //   <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
          //     <div class="bg-blue-600 h-2.5 rounded-full relative" style="width: 50%"></div>
          //   </div>
          // </div>
        </div>
      </Card>
      // <Card 
      //   title="프레임워크 & 라이브러리"
      // >
        // <div class="space-y-3">
          // <div class="space-y-1">
          //   <h2 class="font-semibold flex justify-between w-full">
          //     <span>{ "React" }</span>
          //     <span>{ "81.5%" }</span>
          //   </h2>
          //   <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
          //     <div class="bg-blue-600 h-2.5 rounded-full relative" style="width: 81.5%"></div>
          //   </div>
          // </div>
          // <div class="space-y-1">
          //   <h2 class="font-semibold flex justify-between w-full">
          //     <span>{ "Vue" }</span>
          //     <span>{ "78.7%" }</span>
          //   </h2>
          //   <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
          //     <div class="bg-blue-600 h-2.5 rounded-full relative" style="width: 78.7%"></div>
          //   </div>
          // </div>
          // <div class="space-y-1">
          //   <h2 class="font-semibold flex justify-between w-full">
          //     <span>{ "Three.js" }</span>
          //     <span>{ "30%" }</span>
          //   </h2>
          //   <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
          //     <div class="bg-blue-600 h-2.5 rounded-full relative" style="width: 30%"></div>
          //   </div>
          // </div>
        // </div>
      // </Card>
      <Card 
        title="작업물"
      >
        <ProfileUpdate />
        <div class="grid md:grid-cols-3 gap-x-4 overlfow-x-auto md:overflow-x-hidden overflow-x-scroll scrollbar-thin dark:scrollbar-thumb-gray-900 dark:scrollbar-track-gray-800 scrollbar-thumb-gray-400 scrollbar-track-gray-300 snap-x relative">
          <div class="space-y-2 md:w-auto w-[92.5vw] snap-x snap-center"> 
            <h2 class="font-bold text-center md:text-left"> {"할 일"} </h2>
            <div class="w-full bg-gray-100 rounded-full h-2 dark:bg-gray-700"></div>
          </div>
          <div class="space-y-2 md:w-auto w-[92.5vw] snap-x snap-center"> 
            <h2 class="font-bold text-center md:text-left"> {"진행 중"} </h2>
            <div class="w-full bg-gray-100 rounded-full h-2 dark:bg-gray-700"></div>
          </div>
          <div class="space-y-2 md:w-auto w-[92.5vw] snap-x snap-center"> 
            <h2 class="font-bold text-center md:text-left"> {"완료"} </h2>
            <div class="w-full bg-gray-100 rounded-full h-2 dark:bg-gray-700"></div>
          </div>
          <div class="flex items-center w-full py-3 col-span-3 space-x-2 sticky left-0">
            <div class="sticky left-0 space-x-2 flex items-center">
              <button class="text-xl w-5 h-5 inline-flex items-center justify-center bg-gray-100 hover:bg-gray-200 rounded dark:bg-slate-700 dark:hover:bg-slate-800 ease-in-out duration-200"><i class="ri-arrow-down-s-fill"></i></button>
              <span> {"Ohah"} </span>
            </div>
          </div>
          <div class="bg-gray-100 dark:bg-slate-700 md:min-h-[300px] rounded overflow-y-scroll md:max-h-[700px] max-h-[450px] scrollbar-thin dark:scrollbar-thumb-gray-900 dark:scrollbar-track-gray-800 scrollbar-thumb-gray-400 scrollbar-track-gray-300">
            {
              for work_list.iter().enumerate().filter(|&(i, res)| res.clone().state == "할일".to_string() && res.clone().category == "작업물").map(|(i, row)| {
                let row = row.clone();
                html! {
                  <KanbanCard 
                    key={i}
                    ..row
                  />
                }
              })
            }
          </div>
          <div class="bg-gray-100 dark:bg-slate-700 md:min-h-[300px] rounded overflow-y-auto md:max-h-[700px] max-h-[450px] scrollbar-thin dark:scrollbar-thumb-gray-900 dark:scrollbar-track-gray-800 scrollbar-thumb-gray-400 scrollbar-track-gray-300">
            {
              for work_list.iter().enumerate().filter(|&(i, res)| res.clone().state == "진행중".to_string() && res.clone().category == "작업물").map(|(i, row)| {
                let row = row.clone();
                html! {
                  <KanbanCard 
                    key={i}
                    ..row
                  />
                }
              })
            }
          </div>
          <div class="bg-gray-100 dark:bg-slate-700 md:min-h-[300px] rounded overflow-y-auto md:max-h-[700px] max-h-[450px] scrollbar-thin dark:scrollbar-thumb-gray-900 dark:scrollbar-track-gray-800 scrollbar-thumb-gray-400 scrollbar-track-gray-300">
            {
              for work_list.iter().enumerate().filter(|&(i, res)| res.clone().state == "완료".to_string() && res.clone().category == "작업물").map(|(i, row)| {
                let row = row.clone();
                html! {
                  <KanbanCard 
                    key={i}
                    ..row
                  />
                }
              })
            }
          </div>
        </div>
      </Card>
      <Card 
        title="홈페이지 패치 내역"
      >        
        <ProfileUpdate />
        <div class="grid md:grid-cols-3 gap-x-4 overlfow-x-auto md:overflow-x-hidden overflow-x-scroll scrollbar-thin dark:scrollbar-thumb-gray-900 dark:scrollbar-track-gray-800 scrollbar-thumb-gray-400 scrollbar-track-gray-300 snap-x relative">
          <div class="space-y-2 md:w-auto w-[92.5vw] snap-x snap-center"> 
            <h2 class="font-bold text-center md:text-left"> {"할 일"} </h2>
            <div class="w-full bg-gray-100 rounded-full h-2 dark:bg-gray-700"></div>
          </div>
          <div class="space-y-2 md:w-auto w-[92.5vw] snap-x snap-center"> 
            <h2 class="font-bold text-center md:text-left"> {"진행 중"} </h2>
            <div class="w-full bg-gray-100 rounded-full h-2 dark:bg-gray-700"></div>
          </div>
          <div class="space-y-2 md:w-auto w-[92.5vw] snap-x snap-center"> 
            <h2 class="font-bold text-center md:text-left"> {"완료"} </h2>
            <div class="w-full bg-gray-100 rounded-full h-2 dark:bg-gray-700"></div>
          </div>
          <div class="flex items-center w-full py-3 col-span-3 space-x-2 sticky left-0">
            <div class="sticky left-0 space-x-2 flex items-center">
              <button class="text-xl w-5 h-5 inline-flex items-center justify-center bg-gray-100 hover:bg-gray-200 rounded dark:bg-slate-700 dark:hover:bg-slate-800 ease-in-out duration-200"><i class="ri-arrow-down-s-fill"></i></button>
              <span> {"Ohah"} </span>
            </div>
          </div>
          <div class="bg-gray-100 dark:bg-slate-700 md:min-h-[300px] rounded overflow-y-auto md:max-h-[700px] max-h-[450px] scrollbar-thin dark:scrollbar-thumb-gray-900 dark:scrollbar-track-gray-800 scrollbar-thumb-gray-400 scrollbar-track-gray-300">
            {
              for work_list.iter().enumerate().filter(|&(i, res)| res.clone().state == "할일".to_string() && res.clone().category == "패치내역").map(|(i, row)| {
                let row = row.clone();
                html! {
                  <KanbanCard 
                    key={i}
                    ..row
                  />
                }
              })
            }
          </div>
          <div class="bg-gray-100 dark:bg-slate-700 md:min-h-[300px] rounded overflow-y-auto md:max-h-[700px] max-h-[450px] scrollbar-thin dark:scrollbar-thumb-gray-900 dark:scrollbar-track-gray-800 scrollbar-thumb-gray-400 scrollbar-track-gray-300">
            {
              for work_list.iter().enumerate().filter(|&(i, res)| res.clone().state == "진행중".to_string() && res.clone().category == "패치내역").map(|(i, row)| {
                let row = row.clone();
                html! {
                  <KanbanCard 
                    key={i}
                    ..row
                  />
                }
              })
            }
          </div>
          <div class="bg-gray-100 dark:bg-slate-700 md:min-h-[300px] rounded overflow-y-auto md:max-h-[700px] max-h-[450px] scrollbar-thin dark:scrollbar-thumb-gray-900 dark:scrollbar-track-gray-800 scrollbar-thumb-gray-400 scrollbar-track-gray-300">
            {
              for work_list.iter().enumerate().filter(|&(i, res)| res.clone().state == "완료".to_string() && res.clone().category == "패치내역").map(|(i, row)| {
                let row = row.clone();
                html! {
                  <KanbanCard 
                    key={i}
                    ..row
                  />
                }
              })
            }
          </div>
        </div>
      </Card>
    </article>
  }
}