use web_sys::MouseEvent;
use yew::{Html, function_component, use_state, Callback, html};
use crate::components::transition::Transition;
use crate::components::card::Card;
use crate::components::tag_input::TagInput;

#[function_component(ProfileUpdate)]
pub fn profile_update() -> Html {
  let is_modal = use_state(||false);
  let toggle_modal = {
    let is_modal = is_modal.clone();
    Callback::from(move|e:MouseEvent| {
      e.prevent_default();
      is_modal.set(!*is_modal);
    })
  };
  let tag_changed = {
    Callback::from(move|value| {
    })
  };
  html!(
    <div> 
      <button 
        class="add-profile absolute top-0 right-0 h-[35px] px-2 inline-flex items-center justify-center text-xs rounded hover:bg-black hover:bg-opacity-75 z-[9999]"
        type="button"
        onclick={toggle_modal.clone()}
      > 
        {"추가"}
      </button>
      <Transition 
        show={*is_modal}
        enter="transition ease-in-out duration-300 transform"
        enter_from="scale-y-0"
        enter_to="scale-1"
        leave="transition ease-in-out duration-300 transform"
        leave_from="scale-y-1"
        leave_to="scale-y-0"
        class="bg-black bg-opacity-75 fixed inset-0 h-screen w-full z-[9999] flex items-center justify-center dark:text-slate-400"
      >
        <Card
          title="프로필 수정"
          class="w-[95vw]"
        > 
          <form class="flex flex-col gap-y-2">
            <div class="flex flex-row gap-x-2">
              <label 
                for="fr_date"
                class="flex flex-none px-5 items-center font-bold w-32" 
              >
                { "기간" }
              </label>
              <input 
                class="flex flex-grow font-sans block text-sm w-full py-2 px-3 ring-1 ring-slate-900/10 text-slate-500 rounded-lg dark:bg-slate-800 dark:ring-0 dark:highlight-white/5 dark:text-slate-400 focus:outline-none" 
                id="fr_date" 
                type="date" 
              />
              <label for="la_date"> {"~"} </label>
              <input 
                class="flex flex-grow font-sans block text-sm w-full py-2 px-3 ring-1 ring-slate-900/10 text-slate-500 rounded-lg dark:bg-slate-800 dark:ring-0 dark:highlight-white/5 dark:text-slate-400 focus:outline-none" 
                id="la_date" 
                type="date" 
              />
            </div>
            <div class="flex flex-row gap-x-2">
              <label 
                for="content"
                class="flex flex-none px-5 items-center font-bold w-32 min-h-32" 
              >
                { "내용" }
              </label>
              <textarea 
                id="content" 
                class="flex flex-grow font-sans block text-sm w-full py-2 px-3 ring-1 ring-slate-900/10 text-slate-500 rounded-lg dark:bg-slate-800 dark:ring-0 dark:highlight-white/5 dark:text-slate-400 focus:outline-none" 
              />
            </div>
            <div class="flex flex-row gap-x-2">
              <label 
                for="tag"
                class="flex flex-none px-5 items-center font-bold w-32" 
              >
                { "태그" }
              </label>
              <TagInput 
                onchange={tag_changed}
                default_value={"test"}
              />
            </div>
            <div class="flex flex-row gap-x-2">
              <label 
                for="state"
                class="flex flex-none px-5 items-center font-bold w-32" 
              >
                {"상태"}
              </label>
              <input 
                class="flex flex-grow font-sans block text-sm w-full py-2 px-3 ring-1 ring-slate-900/10 text-slate-500 rounded-lg dark:bg-slate-800 dark:ring-0 dark:highlight-white/5 dark:text-slate-400 focus:outline-none" 
                id="state" 
                type="text" 
              />
            </div>
            <div class="flex justify-end gap-x-3">
              <button class="rounded px-3 py-2 bg-blue-500 text-white font-bold hover:bg-blue-600 duration-200 transition flex justify-center items-center space-x-2"> {"확인"} </button>
              <button class="rounded px-3 py-2 bg-blue-500 text-white font-bold hover:bg-blue-600 duration-200 transition flex justify-center items-center space-x-2"> {"취소"} </button>
            </div>
          </form>
        </Card>
      </Transition>
    </div>
  )
}