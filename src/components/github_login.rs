use yew::{html, function_component};

#[function_component(GitHubLogin)]
pub fn github_login() -> Html {
  let client_id = if cfg!(debug_assertions) {
    // 개발모드
    "7e25b4c04d0672c94c7a"
  } else {
    // 배포모드
    "3191d85ec4a4726fb525"
  };
  html! {
    <div class="flex flex-col dark:bg-slate-800 bg-white dark:shadow-none shadow w-80 pb-10 px-5 rounded-lg space-y-2 modalIn">
      <h2 class="yg-jalnan py-3 t text-2xl"> {"로그인"} </h2>
      <a href={format!("https://github.com/login/oauth/authorize?client_id={}&scope=user",client_id)}>
        <button class="flex bg-black w-full rounded hover:bg-gray-900 items-center text-slate-400 border-slate-700"> 
          <i class="hover:text-slate-700 dark:hover:text-slate-200 text-[1.5rem] ri-github-fill border-r inline-flex w-10 h-10 justify-center items-center border-slate-500/50"></i> 
          <span class="py-2 px-2"> {"Github"} </span>
        </button>
      </a>
    </div>
  }
}