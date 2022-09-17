use yew::{ Html, html };
use yew_router::Routable;
use yew_router::{ Switch };

#[path = "../pages/mod.rs"]
mod pages;
use pages::page_not_found::PageNotFound;
use pages::home::Home;
use pages::profile::Profile;
use pages::auth::Auth;
#[path = "../pages/blog/mod.rs"]
mod blog;
use blog::list::List;
use blog::write::Write;
use blog::view::View;
use blog::my_list::MyList;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum RootRoute {
	#[at("/")]
	Home,
	#[at("/Auth")]
	Auth,
	#[at("/profile")]
	Profile,
	#[at("/v/:page")] List {
		page: String,
	},
	#[at("/w/:id")] WriteRoute,
	#[at("/my/list/:page")] MyPageList {
		page: usize,
	},
	#[at("/my/cmt/:page")] MyPageCmt {
		page: usize,
	},
	#[at("/post/:seo_title")] View {
		seo_title: String,
	},
	#[not_found]
	#[at("/404")]
	NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum WriteRoute {
	#[not_found]
	#[at("/w/create")]
	Write,
	#[at("/w/:id")]
	ModifyWrite {
		id:usize,
	},
}

pub fn write_settings(route: &WriteRoute) -> Html {
	match route {
		WriteRoute::Write => html! {<Write />},
		WriteRoute::ModifyWrite { id }=> {
			let id = Some(id.clone());
			html! {<Write id={id} />}
		},
	}
}

/// 라우터
pub fn root_switch(routes: &RootRoute) -> Html {
	match routes {
		RootRoute::Home => html! { <Home /> },
		RootRoute::Profile => html! { <Profile /> },
		RootRoute::List { page } => {
			let page = format!("{}", page);
			html! { <List page={page} /> }
		},
		RootRoute::MyPageList { page } => {
			html! { <MyList page={page.clone()} /> }
		},
		RootRoute::MyPageCmt { page } => {
			html! { <MyList page={page.clone()} /> }
		},
		RootRoute::WriteRoute => html! {
			<Switch<WriteRoute> render={Switch::render(write_settings)} />
		},
		RootRoute::View { seo_title } => {
			let seo_title = format!("{}", seo_title);
			html! { <View seo_title={seo_title} /> }
		},
		RootRoute::Auth => html! { <Auth /> },
		RootRoute::NotFound => html! { <PageNotFound /> },
	}
}