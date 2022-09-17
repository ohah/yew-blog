use gloo_net::{http::{Request}};
use serde::{ Serialize, Deserialize};
use yew_router::prelude::{AnyHistory, History};
use yewdux::{prelude::Dispatch, store::Store};

use crate::router::root::RootRoute;

use super::{github_auth::{GithubUser}, toast::{ToastMessage, ToastStatus}};

pub struct Blog;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct WritePayload {
	pub content:String,
	pub tag:String,	
	pub title:String,
	pub category:String,
	pub id:Option<usize>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeletePayload {
	pub id:usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct WriteCommentPayload {
	pub content:String,
	pub wr_id:usize,
	pub parent_id:Option<usize>,
	pub id:Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetWritePayload {
  id:String,
}

#[derive(Default, Clone, PartialEq, Eq, Serialize, Deserialize, Store)]
pub struct ResponseCode {
	pub success:bool,
	pub message:String,
	pub seo_title:Option<String>,
}

impl Blog {
	pub fn write(payload:WritePayload, history:AnyHistory, route:RootRoute) {
		let token = Blog::get_token();
		if token.is_empty() {
			Blog::toast_message("정상적인 로그인 상태가 아닙니다", ToastStatus::DANGER, None);
		} else { 
			wasm_bindgen_futures::spawn_local(async move {
				let response = Request::post("/api/write")
				.header("Authorization", token.as_str())
				.header("accept", "applcation/json")
				.header("Access-Control-Allow-Origin", "no-cors")
				.json(&payload)
				.expect("전송 오류")
				.send()
				.await
				.unwrap()
				.json::<ResponseCode>()
				.await;
				match response {
					Ok(response) => {
						let status = match response.success {
							true => ToastStatus::SUCCESS,
							false => ToastStatus::DANGER,
						};
						Blog::toast_message(response.message.as_str(), status, None);
						match response.seo_title {
							Some(seo_title) => {
								history.push(RootRoute::View { seo_title: seo_title } )
							}
							None => {
								log::error!("{:?}", "주소가 리턴되지 않았습니다")
							}
						}
					}
					Err(err) => {
						log::error!("{:?}", err);
						Blog::toast_message("알 수 없는 오류가 발생헀습니다.", ToastStatus::DANGER, None);
					}
				};
			});
		}
	}

	pub fn write_comment(payload:WriteCommentPayload) {
		let token = Blog::get_token();
		if token.is_empty() {
			Blog::toast_message("정상적인 로그인 상태가 아닙니다", ToastStatus::DANGER, None);
		} else {
			wasm_bindgen_futures::spawn_local(async move {
				let response = Request::post("/api/write_comment")
				.header("Authorization", token.as_str())
				.header("accept", "applcation/json")
				.header("Access-Control-Allow-Origin", "no-cors")
				.json(&payload)
				.expect("전송 오류")
				.send()
				.await
				.unwrap()
				.json::<ResponseCode>()
				.await;
				match response {
					Ok(response) => {
						let status = match response.success {
							true => ToastStatus::SUCCESS,
							false => ToastStatus::DANGER,
						};
						Blog::toast_message(response.message.as_str(), status, None);
					}
					Err(err) => {
						log::error!("{:?}", err);
						Blog::toast_message("알 수 없는 오류가 발생헀습니다.", ToastStatus::DANGER, None);
					}
				};
			});
		}
	}

	pub fn remove(payload:DeletePayload, history:AnyHistory, route:RootRoute) {
		let token = Blog::get_token();
		if token.is_empty() {
			Blog::toast_message("정상적인 로그인 상태가 아닙니다", ToastStatus::DANGER, None);
		} else {
			wasm_bindgen_futures::spawn_local(async move {
				let response = Request::post("/api/remove")
				.header("Authorization", token.as_str())
				.header("accept", "applcation/json")
				.header("Access-Control-Allow-Origin", "no-cors")
				.json(&payload)
				.expect("전송 오류")
				.send()
				.await
				.unwrap()
				.json::<ResponseCode>()
				.await;
				match response {
					Ok(response) => {
						let status = match response.success {
							true => ToastStatus::SUCCESS,
							false => ToastStatus::DANGER,
						};
						Blog::toast_message(response.message.as_str(), status, None);
						history.push(RootRoute::List { page: 1.to_string() });
					}
					Err(err) => {
						log::error!("{:?}", err);
						Blog::toast_message("알 수 없는 오류가 발생헀습니다.", ToastStatus::DANGER, None);
					}
				};
			});
		}
	}

	pub fn comment_remove(payload:DeletePayload, history:AnyHistory, route:RootRoute) {
		let token = Blog::get_token();
		if token.is_empty() {
			Blog::toast_message("정상적인 로그인 상태가 아닙니다", ToastStatus::DANGER, None);
		} else {
			wasm_bindgen_futures::spawn_local(async move {
				let response = Request::post("/api/remove_comment")
				.header("Authorization", token.as_str())
				.header("accept", "applcation/json")
				.header("Access-Control-Allow-Origin", "no-cors")
				.json(&payload)
				.expect("전송 오류")
				.send()
				.await
				.unwrap()
				.json::<ResponseCode>()
				.await;
				match response {
					Ok(response) => {
						let status = match response.success {
							true => ToastStatus::SUCCESS,
							false => ToastStatus::DANGER,
						};
						Blog::toast_message(response.message.as_str(), status, None);
						history.replace(route.clone());
					}
					Err(err) => {
						log::error!("{:?}", err);
						Blog::toast_message("알 수 없는 오류가 발생헀습니다.", ToastStatus::DANGER, None);
					}
				};
			});
		}
	}

	pub fn get_token() -> String {
		let dispatch = Dispatch::<GithubUser>::new();	
		let access_token = &dispatch.get().access_token;
		if access_token.clone().is_empty() {
			"".to_string()
		} else {
			format!("Bearer {}", access_token.clone())
		}
	}

	pub fn toast_message(message:&str, status:ToastStatus, timeout:Option<usize>) {
		let toast = Dispatch::<ToastMessage>::new();
		toast.get().message(message, status, timeout);
	}

}