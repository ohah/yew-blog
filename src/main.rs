use yew_router::prelude::*;
use yew::prelude::*;
use yew_router::{ Switch };

mod components;
use crate::components::header::Header;
mod router;
use router::root::{ RootRoute, root_switch };

use crate::components::toast::Toast;

mod store;
use store::theme::Theme;
use yewdux::prelude::use_store;

#[function_component(App)]
fn app() -> Html {
	let (theme, _) = use_store::<Theme>();
	html! {
		<main class={classes!(theme.clone().color.to_string(), "min-h-screen", "duration-200", "ease-in-out", "relative")}>
			<BrowserRouter>
				<div class="z-10 dark:bg-gradient-to-r from-[#1a0540] to-[#200a51] text-slate-500 dark:text-slate-400 min-h-screen w-full">
					<Header />
					<article class="lg:w-full lg:max-w-screen-lg md:mx-auto w-auto md:px-0 px-3.5 md:px-0 overflow-x-hidden">
						<Switch<RootRoute> render={Switch::render(root_switch)} />
					</article>
					<div class="absolute -z-10 w-full h-full top-0 left-0 flex justify-between items-center space-x-5 md:space-x-8 lg:space-x-14">
						<div class="w-1/3 h-full bg-gradient-to-t from-[#FFFFFF14] to-[#C4C4C400]"></div>
						<div class="w-1/3 h-full flex">
							<div class="w-1/2 h-full bg-gradient-to-b from-[#FFFFFF14] to-[#C4C4C400]"></div>
							<div class="w-1/2 h-full bg-gradient-to-t from-[#FFFFFF14] to-[#C4C4C400]"></div>
						</div>
						<div class="w-1/3 h-full bg-gradient-to-b from-[#FFFFFF14] to-[#C4C4C400]"></div>
					</div>
				</div>
				<Toast />
			</BrowserRouter>
			<div id="rust-yew-modal"></div>
		</main>
	}
}

fn main() {
	if cfg!(debug_assertions) {
		wasm_logger::init(wasm_logger::Config::default())		
	}
	yew::start_app::<App>();
}