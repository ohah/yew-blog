use yew::{Properties};
use yewdux::{store::Store, prelude::Dispatch};

#[derive(Clone, PartialEq, Copy)]
pub enum ToastStatus {
  SUCCESS,
  WARNING,
  INFO,
  DANGER,
}

#[derive(Default, Clone, PartialEq, Eq, Store, Properties, Debug)]
pub struct ToastMessage {
  pub message: String,
  pub status: String,
  pub timeout: usize,
  pub key:String,
}
#[derive(Default, Clone, PartialEq, Eq, Store, Properties, Debug)]
pub struct ToastChildren {
  pub children:Vec<ToastMessage>
}

impl ToastMessage {
  pub fn message(&self, message:&str, status:ToastStatus, timeout:Option<usize>) {
    let dispatch = Dispatch::<ToastChildren>::new();
    let mut _status:String = String::from("bg-blue-500");
    let timeout = {
      match timeout {
        Some(timeout) => timeout,
        _ => 3000,
      }
    };
    match status {
      ToastStatus::DANGER => _status = String::from("bg-red-500"),
      ToastStatus::SUCCESS => _status = String::from("bg-green-500"),
      ToastStatus::WARNING => _status = String::from("bg-yellow-500"),
      ToastStatus::INFO => _status = String::from("bg-blue-500"),
    }
    let state = dispatch.get().as_ref().to_owned();
    let x = quad_rand::gen_range(0, 100);
    let key = format!("{}-{}", message, x);
    let temp_data = vec![ToastMessage{
      message:message.to_string(),
      status:_status.clone(),
      timeout:timeout,
      key:key,
    }];
    let data = [state.children, temp_data].concat();
    dispatch.set(ToastChildren{ children:data } );
  }

  pub fn message_hide(&self, key:String) {
    let dispatch = Dispatch::<ToastChildren>::new();
    let state = dispatch.get().as_ref().to_owned();
    let mut children = state.clone().children;
    let index = children.clone().iter().position(|x| *x.key == key).unwrap();
    dispatch.set(ToastChildren{children: children.clone()});
    let result = children.clone().iter().enumerate().filter(|&(_, row )| row.clone().key == key ).count();
    if result == state.children.len() {
      dispatch.set(ToastChildren { children: vec![] })
    }
  }

  pub fn message_remove(&self, key:String) {
    let dispatch = Dispatch::<ToastChildren>::new();
    let state = dispatch.get().as_ref().to_owned();
    let mut children = state.clone().children;
    let index = children.iter().position(|x| *x.key == key).unwrap();
    children.remove(index);
    dispatch.set(ToastChildren{children: children});
  }
}