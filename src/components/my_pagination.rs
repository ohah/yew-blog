use yew::prelude::*;
use yew_router::{components::Link};
use crate::{router::root::RootRoute};

#[derive(Clone, Properties, PartialEq)]
pub struct MyPageProps {
  pub page:usize,
  pub total_rows:usize,
  pub page_rows:usize,
  pub total_page:usize,
}

#[function_component(MyPagination)]
pub fn my_pagination(MyPageProps {page,total_rows, page_rows, total_page }:&MyPageProps) -> Html {
  // let total_page = *total_page as i32;
  let mut prev_page = page.to_string().parse::<i32>().unwrap_or(0) - 1;
  let mut next_page = page.to_string().parse::<i32>().unwrap_or(0) + 1;
  if 0 >= prev_page {
    prev_page = 1;
  }
  if next_page >= *total_page as i32{
    next_page = *total_page as i32;
  }
  const STYLE:&str = "relative z-10 inline-flex justify-center items-center border bg-gray-50 w-10 h-10 text-sm font-medium text-indigo-600 focus:z-20 dark:border-slate-600 dark:bg-neutral-800 dark:text-gray-400 dark:hover:bg-slate-700 hover:bg-blue-100 duration-200 trasition";
  html! {
    if *total_rows <= 9 {
      <></>
    }else {
      <div class="w-full flex justify-center">
        <nav class="isolate inline-flex -space-x-px rounded-md shadow-sm " aria-label="Pagination">
          <Link<RootRoute> to={RootRoute::MyPageList {page : prev_page as usize}}>
            <div class={classes!(STYLE, "rounded-l")}>
              <span class="sr-only">{"Previous"}</span>
              <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                <path fill-rule="evenodd" d="M12.79 5.23a.75.75 0 01-.02 1.06L8.832 10l3.938 3.71a.75.75 0 11-1.04 1.08l-4.5-4.25a.75.75 0 010-1.08l4.5-4.25a.75.75 0 011.06.02z" clip-rule="evenodd" />
              </svg>
            </div>
          </Link<RootRoute>>
          {
            if *total_page as i32 <= 6 {
              // let len = *total_page as i32;
              // let page = (1..=*total_page).collect::<Vec<_>>();
              (1..=*total_page).collect::<Vec<_>>().into_iter().map(|i| {
                html!{
                  <Link<RootRoute> key={i} to={RootRoute::MyPageList {page : i as usize}}>
                    <span aria-current="page" class={classes!(STYLE, if page.clone() == i {"dark:bg-slate-700 bg-blue-100"} else {""})}> {i} </span>
                  </Link<RootRoute>>
                }
              }).collect::<Html>()
            } else {
              let total_page = (1..=*total_page).collect::<Vec<_>>();
              let first = (*page..= *page + 2).collect::<Vec<_>>();
              let len = total_page.len();
              let last = (len - 2..=len).collect::<Vec<_>>();
              let middle = vec![0];
              let total = [first, middle, last].concat();
              html! {
                total.into_iter().map(|i| {
                  html!{
                    if i == 0 {
                      <div key={i}>
                        <span aria-current="page" class={classes!(STYLE)}>{ "..." }</span>
                      </div>
                    } else {
                      <Link<RootRoute> key={i} to={RootRoute::MyPageList {page : i as usize}}>
                        <span aria-current="page" class={classes!(STYLE, if page.clone() == i {"dark:bg-slate-700 bg-blue-100"} else {""})}>{i} </span>
                      </Link<RootRoute>>
                    }
                  }
                }).collect::<Html>()
              }
            }
          }
          <Link<RootRoute> to={RootRoute::MyPageList {page : next_page as usize}}>
            <div class={classes!(STYLE, "rounded-r")}>
              <span class="sr-only">{"Next"}</span>
              <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                <path fill-rule="evenodd" d="M7.21 14.77a.75.75 0 01.02-1.06L11.168 10 7.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z" clip-rule="evenodd" />
              </svg>
            </div>
          </Link<RootRoute>>
        </nav>
      </div>
    }
  }
}
