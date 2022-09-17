use std::{collections::HashMap};
use gloo_net::{http::{Request}};
use serde::{ Serialize, Deserialize };
use serde_json::{Value};
use yewdux::{store::{Store}, prelude::{Dispatch}};

use super::{toast::ToastStatus, blog::Blog};

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
  code:String,
	redirect_url:String,
}

#[derive(Default, Clone, PartialEq, Eq, Store, Serialize, Deserialize)]
pub struct GithubAuth {
	pub access_token:String,
	pub scope:String,
	pub token_type:String,
	#[serde(flatten)]
	pub extra: HashMap<String, Value>,
}

#[derive(Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Plan {
	pub name:String,
	pub collaborators:i32,
	pub private_repos:i32,
	pub space:i32,	
}
#[derive(Default, Clone, PartialEq, Eq, Store, Serialize, Deserialize)]
#[store(storage = "session", storage_tab_sync)]
pub struct GithubUser {
	pub login:String,
  pub id: i32,
  pub node_id:String,
  pub avatar_url:String,
  pub gravatar_id:String,
  pub url:String,
  pub html_url:String,
  pub followers_url:String,
  pub following_url:String,
  pub gists_url:String,
  pub starred_url:String,
  pub subscriptions_url:String,
  pub organizations_url:String,
  pub repos_url:String,
  pub events_url:String,
  pub received_events_url:String,
  pub site_admin: bool,
  pub name:String,
  pub company:String,
  pub blog:String,
  pub location:String,
  pub email:String,
  pub hireable: Option<String>,
  pub bio: Option<String>,
  pub twitter_username: Option<String>,
  pub public_repos: i32,
  pub public_gists: i32,
  pub followers: i32,
  pub following: i32,
  pub created_at:String,
  pub updated_at:String,
  pub private_gists: i32,
  pub total_private_repos: i32,
  pub owned_private_repos: i32,
  pub disk_usage: i32,
  pub collaborators: i32,
  pub two_factor_authentication: bool,
  pub plan: Plan,
	pub access_token:String,
	pub scope:String,
	pub token_type:String,
	#[serde(flatten)]
	pub extra: HashMap<String, Value>,
}

impl GithubAuth {
	pub fn get_user(&self, code:String) {
		let dispatch = Dispatch::<GithubUser>::new();	
		let href = gloo_utils::window().location().pathname().unwrap();
  	let orgin = gloo_utils::window().location().origin().unwrap();
		let redirect_url = format!("{}{}", orgin, href);
		wasm_bindgen_futures::spawn_local(async move {
			let response = Request::post("/api/Auth")
				.header("accept", "applcation/json")
				.header("Access-Control-Allow-Origin", "no-cors")
				.json(&Payload{
					code:code,
					redirect_url: redirect_url
				})
				.expect("전송 오류")
				.send()
				.await
				.unwrap()
				.json::<GithubUser>()
				.await;
				match response {
					Ok(response) => {
						dispatch.set(response);
					}
					Err(err) => {
						Blog::toast_message("로그인 과정에 문제가 있습니다.", ToastStatus::DANGER, None);
					}
				}
		});
	}
	pub fn logout(&self) {
		let dispatch = Dispatch::<GithubAuth>::new();	
		dispatch.set(GithubAuth::default());
		let dispatch = Dispatch::<GithubUser>::new();	
		dispatch.set(GithubUser::default());
	}
}