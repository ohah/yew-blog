use yew::{ function_component, html };

#[function_component(PageNotFound)]
pub fn page_not_found() -> Html {
	html! {
    <section class="bg-primary py-[120px] relative z-10 flex justify-center">
      <div class="container">
        <div class="flex -mx-4">
          <div class="w-full px-4">
            <div class="mx-auto max-w-[400px] text-center">
              <h2 class="font-boldmb-2 text-[50px] sm:text-[80px] md:text-[100px] leading-none text-red-500">
                {"404"}
              </h2>
              <h4 class="text-slate-400 dark:text-slate-300 font-semibold text-[22px] leading-tight mb-3" >
                {"이런! 페이지를 찾을 수 없습니다"}
              </h4>
              <p class="text-lg text-slate-400 dark:text-slate-300 mb-8">
                {"이 페이지는 아마도 삭제 된것 같습니다"}
              </p>
              <a href="javascript:void(0)" class="text-base font-semibold text-slate-400 dark:text-slate-300 inline-block text-center border dark:border-slate-300/50 rounded-lg px-8 py-3 hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-primary transition">
                {"메인으로"}
              </a>
            </div>
          </div>
        </div>
      </div>
    </section>
  }
}