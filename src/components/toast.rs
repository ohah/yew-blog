use yew::prelude::*;
use yewdux::{prelude::{use_store, Dispatch}};
use crate::store::toast::{ToastMessage, ToastChildren};
use crate::components::transition::Transition;

#[function_component(Message)]
pub fn message(ToastMessage{message,status,timeout,key }:&ToastMessage) -> Html {
  let dispatch = Dispatch::<ToastMessage>::new();
  let show = use_state(|| true);
  {
    let show = show.clone();
    let timeout = timeout.clone();
    use_effect_with_deps(move |_| {
      let timeout = timeout.clone() as u32;
      gloo::timers::callback::Timeout::new(timeout, move || {
        show.set(false);
      })
      .forget();
      || ()
    }, ());
  }
  
  let callback = {
    let dispatch = dispatch.clone();
    let key = key.clone();
    Callback::from(move|value| {
      dispatch.get().message_hide(key.clone());
    })
  };

  html! {
    <Transition 
      show={*show}
      enter="transition ease-in-out duration-300 transform"
      enter_from="translate-x-[200%]"
      enter_to="translate-x-0"
      leave="transition ease-in-out duration-300 transform"
      leave_from="translate-x-0"
      leave_to="translate-x-[200%]"
      callback={callback}
      class="z-[100000]"
    >
      <div class={classes!(status, "p-3", "rounded")}>
        {format!("{}", message)}
      </div>
    </Transition>
  }
}

#[function_component(Toast)]
pub fn toast() -> Html {
  let (state, _) = use_store::<ToastChildren>();
  html! {
    <div class={classes!("fixed", "bottom-10", "right-5", "space-y-5", "z-[10000000]", "text-white")}>
      { for state.children.iter().enumerate().map(|(i, row)| {
        let row = row.clone();
        html! {
          <Message 
            key={i}
            ..row 
          />
        }
      })}
    </div>
  }
}