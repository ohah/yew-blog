use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::{ function_component, html, use_effect_with_deps, use_state, Properties };
use crate::{components::{card::Card, list_card::{Datas, ListCard}}, store::{toast::{ToastStatus}, blog::Blog}};
use crate::components::my_pagination::MyPagination;

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct MyListResponse {
  pub data:Vec<Datas>,
  pub page:usize,
  #[serde(rename="totalPage")]
  pub total_page:usize,
  #[serde(rename="totalRows")]
  pub total_rows:usize,
  #[serde(rename="pageRows")]
  pub page_rows:usize,
}

#[derive(Properties, PartialEq, Clone)]
pub struct MyListProps {
  pub page:usize,
}
#[function_component(MyList)]
pub fn my_list(MyListProps{ page }:&MyListProps) -> Html {
  let lists = use_state(||MyListResponse::default());
  gloo_utils::document().set_title("Ohah의 블로그");
  {
    let lists = lists.clone();
    let url = format!("/api/profile/list/{}", page.clone());
    use_effect_with_deps(move |page| {
      wasm_bindgen_futures::spawn_local(async move {        
        let token = Blog::get_token();
        if token.is_empty() {
          Blog::toast_message("정상적인 로그인 상태가 아닙니다", ToastStatus::DANGER, None);
        } else {
          let fetched_list  = Request::get(url.as_str())
            .header("accept", "application/json")
            .header("Authorization", token.as_str())
            .header("Access-Control-Allow-Origin", "no-cors")
            .send()
            .await
            .unwrap()
            .json::<MyListResponse>()
            .await;
          match fetched_list {
            Ok(fetched_list) => {
              lists.set(fetched_list);
            }
            Err(err) => {
              log::info!("Update: {:?}", err);
              Blog::toast_message("알 수 없는 오류가 발생헀습니다.", ToastStatus::DANGER, None);
            }
          };
        }
      });
      || ()
    }, page.clone());
  }
  
	html! {
    <div class="lg:max-w-screen-lg px-2 my-3 space-y-3 dark:divide-slate-600">
      <Card 
        title="목록"
        >
        <div class="grid grid-cols-3 gap-x-4">
          <div class="bg-gray-100 dark:bg-slate-700 min-h-[220px] rounded">
            { 
              for lists.data.iter().enumerate().filter(|&(i, _)| i % 3 == 0 ).map(|(i, row)| {
                let row = row.clone();
                html! {
                  <ListCard 
                    key={i}
                    ..row 
                  />
                }
              })
            }
          </div>
          <div class="bg-gray-100 dark:bg-slate-700 min-h-[220px] rounded">
            { 
              for lists.data.iter().enumerate().filter(|&(i, _)| i % 3 == 1 ).map(|(i, row)| {
                let row = row.clone();
                html! {
                  <ListCard 
                    key={i}
                    ..row 
                  />
                }
              })
            }
          </div>
          <div class="bg-gray-100 dark:bg-slate-700 min-h-[220px] rounded">
            { 
              for lists.data.iter().enumerate().filter(|&(i, _)| i % 3 == 2 ).map(|(i, row)| {
                let row = row.clone();
                html! {
                  <ListCard 
                    key={i}
                    ..row 
                  />
                }
              })
            }
          </div>
        </div>
      </Card>

      <MyPagination 
        page={lists.page}
        total_rows={lists.total_rows}
        total_page={lists.total_page}
        page_rows={lists.page_rows}
      />
    </div>
  }
}