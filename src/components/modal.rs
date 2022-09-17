use web_sys::MouseEvent;
use yew::{ html, create_portal, function_component, Children, Properties, Callback };

#[derive(Properties, PartialEq)]
pub struct ModalProps {
	#[prop_or_default]
	pub children: Children,
	pub is_close:Callback<MouseEvent>,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
	let modal_host = gloo_utils
		::document()
		.get_element_by_id("rust-yew-modal")
		.expect("a #rust-yew-modal element");
	create_portal(html! { 
		<div onclick={props.is_close.clone()} class="bg-black bg-opacity-75 fixed inset-0 h-screen w-full z-[9999] flex items-center justify-center dark:text-slate-400"> 
			<div
				onclick={Callback::from(move |e:MouseEvent|{
					e.stop_propagation();
				})}
			>
				{ for props.children.iter() }
			</div>
		</div>
	}, modal_host.into())
}