use yew::{ html, Children, Component, Context, Html, Properties, classes};

#[derive(Properties, PartialEq)]
pub struct CardProps {
	#[prop_or_default]
	pub children: Children,
	pub class: Option<String>,
	pub title: Option<String>,
}

impl Card {
	fn minimize(&mut self) {
		self.minimize = true;
	}
	fn maximize(&mut self) {
		self.minimize = false;
	}
}

pub enum CardMsg {
	Minimize,
	Maximize,
}

pub struct Card {
	minimize: bool,
}

impl Component for Card {
	type Message = CardMsg;
	type Properties = CardProps;

	fn create(_ctx: &Context<Self>) -> Self {
		Self {
			minimize: false,
		}
	}

	fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			CardMsg::Minimize => {
				self.minimize();
				true
			}
			CardMsg::Maximize => {
				self.maximize();
				true
			}
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		const DEFAULT: &str =
			"card px-2 py-4 shadow rounded-[6px] my-2 dark:border-[#100322] border dark:bg-neutral-800 relative border-b-[#b4b4b4] border-r-[#b4b4b4] border-l-[#b4b4b4] overflow-hidden";
		let minimize = ctx.link().callback(|_| CardMsg::Minimize);
		let maximize = ctx.link().callback(|_| CardMsg::Maximize);

		html! {
			<div class={classes!(Some(&ctx.props().class), DEFAULT)}>
				<header class="h-[35px] border-b border-t rounded-t-[6px] flex justify-between w-full text-[11pt] absolute top-0 left-0 bg-[#ebebeb] border-[#d8d8d8] border-b-[#b4b4b4] dark:bg-[#1c1b1e] dark:border-[#100322] z-50">
					<div class="flex items-center space-x-[6px] ml-[6px] group absolute top-0 left-0 h-full">
						<button class="w-[11px] h-[11px] bg-[#ff5c5c] border border-[#e33e41] rounded-full text-[9pt] inline-flex items-center justify-center">
							<i class="ri-close-fill hidden group-hover:block text-black font-bold"></i>
						</button>
						<button
							onclick={maximize}
							class="w-[11px] h-[11px] bg-[#ffbd4c] border border-[#e09e3e] rounded-full text-[9pt] inline-flex items-center justify-center"
						>
							<i class="ri-add-fill hidden group-hover:block text-black font-bold"></i>
						</button>
						<button 
							onclick={minimize}
							class="w-[11px] h-[11px] bg-[#00ca56] border border-[#14ae46] rounded-full text-[9pt] inline-flex items-center justify-center
						">
							<i class="ri-subtract-fill hidden group-hover:block text-black font-bold"></i>
						</button>
					</div>
					<div class="w-full justify-center flex items-center text-xl font-bold">
						{ ctx.props().title.clone().unwrap_or_default() }
					</div>
				</header>
				<div class={classes!("mt-8", "text-sm", if self.minimize {"minimize"} else {"maximize"})}>
					{ for ctx.props().children.iter() }
				</div>
			</div>
		}
	}
}