use yew::prelude::*;
use yewdux::{prelude::{use_store, Dispatch}};
use crate::store::toast::{ToastMessage, ToastChildren};

#[function_component(Message)]
pub fn message(ToastMessage{message,status,timeout,key,show }:&ToastMessage) -> Html {
  let dispatch = Dispatch::<ToastMessage>::new();
  let timer_id = use_state(|| None);
  let state = use_state(|| false);
  let timeout = timeout.clone();
  {
    let timer_id = timer_id.clone();
    let state = state.clone();
    let key = key.clone();
    let dispatch = dispatch.clone();
    use_effect_with_deps(move |_| {
      if *state.clone() == false {
        state.set(true);
        let timeout = timeout.clone() as u32;
        let id = {
          let timer_id = timer_id.clone();
          gloo::timers::callback::Timeout::new(timeout, move || {
            // log::info!("실행");
            state.set(false);
            timer_id.set(None);
            dispatch.get().message_hide(key);
          })
          .forget()
        };
        timer_id.set(Some(id));
      }
      || ()
    }, ());
  }
  
  let hide_class = if *timer_id.clone() == None {
    Some("toast-hide")
  } else {
    None
  };
  html! {
    <div class={classes!(hide_class, status, "toast", "p-3", "rounded")}>
      {format!("{}", message)}
    </div>    
  }
}

#[function_component(Toast)]
pub fn toast() -> Html {
  let (state, _) = use_store::<ToastChildren>();
  html! {
    <div class={classes!("fixed", "bottom-10", "right-5", "space-y-5", "z-[3000]", "text-white")}>
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