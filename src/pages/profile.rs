use yew::{ function_component, html };
use crate::components::card;
use crate::components::work_state::TagProps;
use card::Card;
use crate::components::kanban_card;
use kanban_card::KanbanCard;

#[function_component(Profile)]
pub fn profile() -> Html {
	html! {
    <article class="grid mt-2">
      <Card 
      title="소개"
      >        
        <p>{ "React, Vue, Typescript를 주력으로 개발하는 개발자 입니다" }</p>
        <p>{ "여러가지 웹기술에 관심이 많으며 특히 Wasm, Rust에 관심이 많습니다"} </p>
        <p>{ "이 페이지도 WebAssmbly로 제작되었습니다!"} </p>
      </Card>
      <Card 
        title="기술"
      > 
        <div class="space-y-3">
          <div class="flex flex-wrap gap-x-2 gap-y-2">
            <div class="yg-jalnan text-xs px-2 py-2 rounded-lg border-2 dark:border-slate-600 border-gray-400 flex items-center gap-x-1"> 
              <div class="inline-flex w-5 h-5">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32"><rect x="2" y="2" width="28" height="28" style="fill:#f5de19"/><path d="M20.809,23.875a2.866,2.866,0,0,0,2.6,1.6c1.09,0,1.787-.545,1.787-1.3,0-.9-.716-1.222-1.916-1.747l-.658-.282c-1.9-.809-3.16-1.822-3.16-3.964,0-1.973,1.5-3.476,3.853-3.476a3.889,3.889,0,0,1,3.742,2.107L25,18.128A1.789,1.789,0,0,0,23.311,17a1.145,1.145,0,0,0-1.259,1.128c0,.789.489,1.109,1.618,1.6l.658.282c2.236.959,3.5,1.936,3.5,4.133,0,2.369-1.861,3.667-4.36,3.667a5.055,5.055,0,0,1-4.795-2.691Zm-9.295.228c.413.733.789,1.353,1.693,1.353.864,0,1.41-.338,1.41-1.653V14.856h2.631v8.982c0,2.724-1.6,3.964-3.929,3.964a4.085,4.085,0,0,1-3.947-2.4Z"/></svg>
              </div>
              <span> {"Javascript"} </span>
            </div>
            <div class="yg-jalnan text-xs px-2 py-2 rounded-lg border-2 dark:border-slate-600 border-gray-400 flex items-center gap-x-1"> 
              <div class="inline-flex w-5 h-5">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32"><polygon points="2 16 2 30 16 30 30 30 30 16 30 2 16 2 2 2 2 16" style="fill:#007acc"/><path d="M24.564,14.884a3.485,3.485,0,0,1,1.751,1.009,4.611,4.611,0,0,1,.671.9c.009.036-1.209.853-1.947,1.311-.027.018-.133-.1-.253-.276a1.587,1.587,0,0,0-1.316-.791c-.849-.058-1.4.387-1.391,1.129a1.027,1.027,0,0,0,.12.524c.187.387.533.618,1.622,1.089,2,.862,2.862,1.431,3.4,2.24a4.063,4.063,0,0,1,.324,3.413,3.753,3.753,0,0,1-3.1,2.218,8.584,8.584,0,0,1-2.133-.022,5.145,5.145,0,0,1-2.849-1.484,4.947,4.947,0,0,1-.729-1.08,2.092,2.092,0,0,1,.258-.164c.124-.071.6-.342,1.04-.6l.8-.467L21,24.08A3.759,3.759,0,0,0,22.067,25.1a2.6,2.6,0,0,0,2.724-.138,1.217,1.217,0,0,0,.156-1.551c-.218-.311-.662-.573-1.924-1.12a6.93,6.93,0,0,1-2.636-1.622,3.692,3.692,0,0,1-.769-1.4,5.606,5.606,0,0,1-.049-1.787,3.413,3.413,0,0,1,2.871-2.658A7.092,7.092,0,0,1,24.564,14.884Zm-6.573,1.169L18,17.2H14.356V27.556H11.778V17.2H8.133V16.076a11.018,11.018,0,0,1,.031-1.156c.013-.018,2.231-.027,4.92-.022l4.893.013Z" style="fill:#fff"/></svg>
              </div>
              <span>{"Typescript"}</span>
            </div>
            <div class="yg-jalnan text-xs px-2 py-2 rounded-lg border-2 dark:border-slate-600 border-gray-400 flex items-center gap-x-1"> 
              <div class="inline-flex w-5 h-5">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32"><polygon points="5.902 27.201 3.656 2 28.344 2 26.095 27.197 15.985 30 5.902 27.201" style="fill:#1572b6"/><polygon points="16 27.858 24.17 25.593 26.092 4.061 16 4.061 16 27.858" style="fill:#33a9dc"/><polygon points="16 13.191 20.09 13.191 20.372 10.026 16 10.026 16 6.935 16.011 6.935 23.75 6.935 23.676 7.764 22.917 16.282 16 16.282 16 13.191" style="fill:#fff"/><polygon points="16.019 21.218 16.005 21.222 12.563 20.292 12.343 17.827 10.67 17.827 9.24 17.827 9.673 22.68 16.004 24.438 16.019 24.434 16.019 21.218" style="fill:#ebebeb"/><polygon points="19.827 16.151 19.455 20.29 16.008 21.22 16.008 24.436 22.344 22.68 22.391 22.158 22.928 16.151 19.827 16.151" style="fill:#fff"/><polygon points="16.011 6.935 16.011 8.855 16.011 10.018 16.011 10.026 8.555 10.026 8.555 10.026 8.545 10.026 8.483 9.331 8.342 7.764 8.268 6.935 16.011 6.935" style="fill:#ebebeb"/><polygon points="16 13.191 16 15.111 16 16.274 16 16.282 12.611 16.282 12.611 16.282 12.601 16.282 12.539 15.587 12.399 14.02 12.325 13.191 16 13.191" style="fill:#ebebeb"/></svg>
              </div>
              <span>{"CSS3"}</span>
            </div>
            <div class="yg-jalnan text-xs px-2 py-2 rounded-lg border-2 dark:border-slate-600 border-gray-400 flex items-center gap-x-1"> 
              <div class="inline-flex w-5 h-5">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32"><polygon points="5.902 27.201 3.655 2 28.345 2 26.095 27.197 15.985 30 5.902 27.201" style="fill:#e44f26"/><polygon points="16 27.858 24.17 25.593 26.092 4.061 16 4.061 16 27.858" style="fill:#f1662a"/><polygon points="16 13.407 11.91 13.407 11.628 10.242 16 10.242 16 7.151 15.989 7.151 8.25 7.151 8.324 7.981 9.083 16.498 16 16.498 16 13.407" style="fill:#ebebeb"/><polygon points="16 21.434 15.986 21.438 12.544 20.509 12.324 18.044 10.651 18.044 9.221 18.044 9.654 22.896 15.986 24.654 16 24.65 16 21.434" style="fill:#ebebeb"/><polygon points="15.989 13.407 15.989 16.498 19.795 16.498 19.437 20.507 15.989 21.437 15.989 24.653 22.326 22.896 22.372 22.374 23.098 14.237 23.174 13.407 22.341 13.407 15.989 13.407" style="fill:#fff"/><polygon points="15.989 7.151 15.989 9.071 15.989 10.235 15.989 10.242 23.445 10.242 23.445 10.242 23.455 10.242 23.517 9.548 23.658 7.981 23.732 7.151 15.989 7.151" style="fill:#fff"/></svg>
              </div>
              <span>{"HTML"}</span>
            </div>
            <div class="yg-jalnan text-xs px-2 py-2 rounded-lg border-2 dark:border-slate-600 border-gray-400 flex items-center gap-x-1"> 
              <div class="inline-flex w-5 h-5">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32"><defs><radialGradient id="a" cx="-16.114" cy="20.532" r="18.384" gradientTransform="translate(26.52 -9.307)" gradientUnits="userSpaceOnUse"><stop offset="0" stop-color="#fff"/><stop offset="0.5" stop-color="#4c6b96"/><stop offset="1" stop-color="#231f20"/></radialGradient></defs><ellipse cx="16" cy="16" rx="14" ry="7.365" style="fill:url(#a)"/><ellipse cx="16" cy="16" rx="13.453" ry="6.818" style="fill:#6280b6"/><path d="M18.725,18.2l.667-3.434a1.752,1.752,0,0,0-.372-1.719,2.929,2.929,0,0,0-2-.525H15.867l.331-1.7a.219.219,0,0,0-.215-.26h-1.6a.219.219,0,0,0-.215.177l-.709,3.646a2.051,2.051,0,0,0-.477-1.054,2.783,2.783,0,0,0-2.2-.807H7.7a.219.219,0,0,0-.215.177l-1.434,7.38a.219.219,0,0,0,.215.26H7.869a.219.219,0,0,0,.215-.177l.347-1.785h1.2a5.167,5.167,0,0,0,1.568-.2,3.068,3.068,0,0,0,1.15-.689,3.538,3.538,0,0,0,.68-.844l-.287,1.475a.219.219,0,0,0,.215.26h1.6a.219.219,0,0,0,.215-.177l.787-4.051h1.094c.466,0,.6.093.64.133s.1.165.025.569l-.635,3.265a.219.219,0,0,0,.215.26h1.62A.219.219,0,0,0,18.725,18.2ZM11.33,15.366a1.749,1.749,0,0,1-.561,1.092,2.171,2.171,0,0,1-1.315.321H8.742l.515-2.651h.921c.677,0,.949.145,1.059.266A1.181,1.181,0,0,1,11.33,15.366Z" style="fill:#fff"/><path d="M25.546,13.332a2.783,2.783,0,0,0-2.2-.807H20.255a.219.219,0,0,0-.215.177l-1.434,7.38a.219.219,0,0,0,.215.26h1.608a.219.219,0,0,0,.215-.177l.347-1.785h1.2a5.167,5.167,0,0,0,1.568-.2,3.068,3.068,0,0,0,1.15-.689,3.425,3.425,0,0,0,1.076-1.927A2.512,2.512,0,0,0,25.546,13.332Zm-1.667,2.034a1.749,1.749,0,0,1-.561,1.092A2.171,2.171,0,0,1,22,16.778H21.29l.515-2.651h.921c.677,0,.949.145,1.059.266A1.181,1.181,0,0,1,23.879,15.366Z" style="fill:#fff"/><path d="M10.178,13.908a1.645,1.645,0,0,1,1.221.338,1.34,1.34,0,0,1,.145,1.161,1.945,1.945,0,0,1-.642,1.223A2.361,2.361,0,0,1,9.454,17H8.476l.6-3.089ZM6.261,20.124H7.869l.381-1.962H9.627a4.931,4.931,0,0,0,1.5-.191,2.84,2.84,0,0,0,1.07-.642,3.207,3.207,0,0,0,1.01-1.808,2.3,2.3,0,0,0-.385-2.044,2.568,2.568,0,0,0-2.035-.732H7.7Z" style="fill:#000004"/><path d="M14.387,10.782h1.6L15.6,12.744h1.421a2.767,2.767,0,0,1,1.85.468,1.548,1.548,0,0,1,.305,1.516l-.667,3.434H16.89l.635-3.265a.886.886,0,0,0-.08-.76,1.121,1.121,0,0,0-.8-.2H15.37l-.822,4.228h-1.6Z" style="fill:#000004"/><path d="M22.727,13.908a1.645,1.645,0,0,1,1.221.338,1.34,1.34,0,0,1,.145,1.161,1.945,1.945,0,0,1-.642,1.223A2.361,2.361,0,0,1,22,17h-.978l.6-3.089ZM18.81,20.124h1.608l.381-1.962h1.377a4.931,4.931,0,0,0,1.5-.191,2.84,2.84,0,0,0,1.07-.642,3.207,3.207,0,0,0,1.01-1.808,2.3,2.3,0,0,0-.385-2.044,2.568,2.568,0,0,0-2.035-.732H20.244Z" style="fill:#000004"/></svg>
              </div>
              <span>{"PHP"}</span>
            </div>
            <div class="yg-jalnan text-xs px-2 py-2 rounded-lg border-2 dark:border-slate-600 border-gray-400 flex items-center gap-x-1"> 
              <div class="inline-flex w-5 h-5">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32"><defs><radialGradient id="a" cx="16" cy="16" r="14" gradientTransform="translate(-2.656 15.686) rotate(-30) scale(1 0.6)" gradientUnits="userSpaceOnUse"><stop offset="0" stop-color="#7d7d7d"/><stop offset="0.267" stop-color="#7e7c7a"/><stop offset="0.45" stop-color="#817871"/><stop offset="0.608" stop-color="#867162"/><stop offset="0.753" stop-color="#8d684c"/><stop offset="0.886" stop-color="#965c30"/><stop offset="1" stop-color="#a04f12"/></radialGradient></defs><path d="M15.124,5.3a.832.832,0,1,1,.832.832.832.832,0,0,1-.832-.832M5.2,12.834a.832.832,0,1,1,.832.832.832.832,0,0,1-.832-.832m19.856.039a.832.832,0,1,1,.832.832.832.832,0,0,1-.832-.832M7.605,14.013a.76.76,0,0,0,.386-1l-.369-.835H9.074v6.545H6.144a10.246,10.246,0,0,1-.332-3.911Zm6.074.161V12.245h3.458c.179,0,1.261.206,1.261,1.016,0,.672-.83.913-1.513.913ZM8.958,24.561a.832.832,0,1,1,.832.832.832.832,0,0,1-.832-.832m12.331.039a.832.832,0,1,1,.832.832.832.832,0,0,1-.832-.832m.257-1.887a.759.759,0,0,0-.9.584l-.418,1.949a10.246,10.246,0,0,1-8.545-.041l-.417-1.949a.758.758,0,0,0-.9-.583l-1.721.37a10.246,10.246,0,0,1-.89-1.049h8.374c.095,0,.158-.017.158-.1V18.928c0-.086-.063-.1-.158-.1H13.679V16.947h2.649a1.665,1.665,0,0,1,1.629,1.412c.105.413.336,1.757.494,2.187.157.483.8,1.447,1.482,1.447h4.323a10.246,10.246,0,0,1-.949,1.1Zm4.65-7.821a10.246,10.246,0,0,1,.022,1.779H25.167c-.105,0-.148.069-.148.172v.483c0,1.136-.641,1.384-1.2,1.447-.535.06-1.128-.224-1.2-.551a3.616,3.616,0,0,0-1.671-2.808c1.03-.654,2.1-1.619,2.1-2.911a3.293,3.293,0,0,0-1.608-2.7,4.562,4.562,0,0,0-2.2-.724H8.367A10.246,10.246,0,0,1,14.1,5.84l1.282,1.344a.758.758,0,0,0,1.073.025l1.434-1.372a10.246,10.246,0,0,1,7.015,5l-.982,2.217a.761.761,0,0,0,.386,1Zm2.448.036-.033-.343,1.011-.943a.421.421,0,0,0-.134-.676L28.2,12.483l-.1-.334.806-1.12a.421.421,0,0,0-.263-.636l-1.363-.222-.164-.306.573-1.257a.419.419,0,0,0-.382-.573l-1.383.048L25.7,7.819l.318-1.347a.421.421,0,0,0-.487-.487L24.183,6.3l-.266-.219L23.966,4.7a.421.421,0,0,0-.572-.383l-1.257.573-.306-.164-.222-1.363a.421.421,0,0,0-.636-.263l-1.121.806-.333-.1-.483-1.293a.421.421,0,0,0-.675-.135l-.943,1.012-.343-.033-.728-1.177a.421.421,0,0,0-.688,0l-.728,1.177-.343.033-.943-1.012a.421.421,0,0,0-.675.135L12.483,3.8l-.333.1L11.03,3.1a.421.421,0,0,0-.636.263l-.222,1.363-.306.164L8.608,4.317a.42.42,0,0,0-.572.383l.048,1.383L7.818,6.3,6.471,5.984a.421.421,0,0,0-.487.487L6.3,7.819l-.218.265L4.7,8.036a.422.422,0,0,0-.383.573L4.89,9.866l-.164.306-1.363.222a.421.421,0,0,0-.263.636l.806,1.12-.1.334-1.293.483a.421.421,0,0,0-.134.676l1.011.943-.033.343-1.177.728a.421.421,0,0,0,0,.688l1.177.728.033.343-1.011.943a.421.421,0,0,0,.134.675l1.293.483.1.334L3.1,20.972a.421.421,0,0,0,.264.636l1.363.222.164.307-.573,1.257a.421.421,0,0,0,.383.573l1.383-.048.219.266-.317,1.348a.42.42,0,0,0,.487.486L7.818,25.7l.266.218L8.035,27.3a.42.42,0,0,0,.572.382l1.257-.573.306.164.222,1.362a.421.421,0,0,0,.636.264l1.12-.807.334.1.483,1.292a.421.421,0,0,0,.675.134l.943-1.011.343.034.728,1.177a.422.422,0,0,0,.688,0l.728-1.177.343-.034.943,1.011a.421.421,0,0,0,.675-.134l.483-1.292.334-.1,1.12.807a.421.421,0,0,0,.636-.264l.222-1.362.306-.164,1.257.573a.42.42,0,0,0,.572-.382l-.048-1.384.265-.218,1.347.317a.42.42,0,0,0,.487-.486L25.7,24.183l.218-.266,1.383.048a.421.421,0,0,0,.382-.573l-.573-1.257.164-.307,1.363-.222a.42.42,0,0,0,.263-.636l-.806-1.12.1-.334,1.293-.483a.42.42,0,0,0,.134-.675l-1.011-.943.033-.343,1.177-.728a.421.421,0,0,0,0-.688Z" style="fill:url(#a)"/></svg>
              </div>
              <span>{"Rust"}</span>
            </div>
            <div class="yg-jalnan text-xs px-2 py-2 rounded-lg border-2 dark:border-slate-600 border-gray-400 flex items-center gap-x-1"> 
              <div class="inline-flex w-5 h-5">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32"><circle cx="16" cy="15.974" r="2.5" style="fill:#007acc"/><path d="M16,21.706a28.385,28.385,0,0,1-8.88-1.2,11.3,11.3,0,0,1-3.657-1.958A3.543,3.543,0,0,1,2,15.974c0-1.653,1.816-3.273,4.858-4.333A28.755,28.755,0,0,1,16,10.293a28.674,28.674,0,0,1,9.022,1.324,11.376,11.376,0,0,1,3.538,1.866A3.391,3.391,0,0,1,30,15.974c0,1.718-2.03,3.459-5.3,4.541A28.8,28.8,0,0,1,16,21.706Zm0-10.217a27.948,27.948,0,0,0-8.749,1.282c-2.8.977-4.055,2.313-4.055,3.2,0,.928,1.349,2.387,4.311,3.4A27.21,27.21,0,0,0,16,20.51a27.6,27.6,0,0,0,8.325-1.13C27.4,18.361,28.8,16.9,28.8,15.974a2.327,2.327,0,0,0-1.01-1.573,10.194,10.194,0,0,0-3.161-1.654A27.462,27.462,0,0,0,16,11.489Z" style="fill:#007acc"/><path d="M10.32,28.443a2.639,2.639,0,0,1-1.336-.328c-1.432-.826-1.928-3.208-1.327-6.373a28.755,28.755,0,0,1,3.4-8.593h0A28.676,28.676,0,0,1,16.71,5.995a11.376,11.376,0,0,1,3.384-2.133,3.391,3.391,0,0,1,2.878,0c1.489.858,1.982,3.486,1.287,6.859a28.806,28.806,0,0,1-3.316,8.133,28.385,28.385,0,0,1-5.476,7.093,11.3,11.3,0,0,1-3.523,2.189A4.926,4.926,0,0,1,10.32,28.443Zm1.773-14.7a27.948,27.948,0,0,0-3.26,8.219c-.553,2.915-.022,4.668.75,5.114.8.463,2.742.024,5.1-2.036a27.209,27.209,0,0,0,5.227-6.79,27.6,27.6,0,0,0,3.181-7.776c.654-3.175.089-5.119-.713-5.581a2.327,2.327,0,0,0-1.868.089A10.194,10.194,0,0,0,17.5,6.9a27.464,27.464,0,0,0-5.4,6.849Z" style="fill:#007acc"/><path d="M21.677,28.456c-1.355,0-3.076-.82-4.868-2.361a28.756,28.756,0,0,1-5.747-7.237h0a28.676,28.676,0,0,1-3.374-8.471,11.376,11.376,0,0,1-.158-4A3.391,3.391,0,0,1,8.964,3.9c1.487-.861,4.01.024,6.585,2.31a28.8,28.8,0,0,1,5.39,6.934,28.384,28.384,0,0,1,3.41,8.287,11.3,11.3,0,0,1,.137,4.146,3.543,3.543,0,0,1-1.494,2.555A2.59,2.59,0,0,1,21.677,28.456Zm-9.58-10.2a27.949,27.949,0,0,0,5.492,6.929c2.249,1.935,4.033,2.351,4.8,1.9.8-.465,1.39-2.363.782-5.434A27.212,27.212,0,0,0,19.9,13.74,27.6,27.6,0,0,0,14.755,7.1c-2.424-2.152-4.39-2.633-5.191-2.169a2.327,2.327,0,0,0-.855,1.662,10.194,10.194,0,0,0,.153,3.565,27.465,27.465,0,0,0,3.236,8.1Z" style="fill:#007acc"/></svg>
              </div>
              <span>{"React"}</span>
            </div>
            <div class="yg-jalnan text-xs px-2 py-2 rounded-lg border-2 dark:border-slate-600 border-gray-400 flex items-center gap-x-1"> 
              <div class="inline-flex w-5 h-5 dark:fill-white">
                <svg version="1.0" xmlns="http://www.w3.org/2000/svg" width="48.000000pt" height="48.000000pt" viewBox="0 0 48.000000 48.000000" preserveAspectRatio="xMidYMid meet">
                  <g transform="translate(0.000000,0.000000) scale(0.100000,-0.100000)" stroke="none">
                    <path d="M160 467 c-49 -16 -133 -102 -148 -153 -28 -94 -8 -169 63 -239 101 -102 229 -102 330 0 102 101 102 229 0 330 -70 71 -152 91 -245 62z m102 -232 c74 -94 91 -125 68 -125 -5 0 -40 39 -77 86 l-68 87 -5 -69 c-4 -52 -9 -69 -20 -69 -12 0 -16 17 -18 84 -2 46 0 91 3 98 11 29 42 5 117 -92z m74 90 c8 -21 -4 -67 -16 -60 -13 8 -13 75 0 75 6 0 13 -7 16 -15z"/>
                  </g>
                </svg>
              </div>
              <span>{"Next.js"}</span>
            </div>
            <div class="yg-jalnan text-xs px-2 py-2 rounded-lg border-2 dark:border-slate-600 border-gray-400 flex items-center gap-x-1"> 
              <div class="inline-flex w-5 h-5">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32"><path d="M24.4,3.925H30L16,28.075,2,3.925H12.71L16,9.525l3.22-5.6Z" style="fill:#41b883"/><path d="M2,3.925l14,24.15L30,3.925H24.4L16,18.415,7.53,3.925Z" style="fill:#41b883"/><path d="M7.53,3.925,16,18.485l8.4-14.56H19.22L16,9.525l-3.29-5.6Z" style="fill:#35495e"/></svg>
              </div>
              <span>{"Vue.js(2,3)"}</span>
            </div>
            <div class="yg-jalnan text-xs px-2 py-2 rounded-lg border-2 dark:border-slate-600 border-gray-400 flex items-center gap-x-1"> 
              <div class="inline-flex w-5 h-5">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32"><defs><radialGradient id="a" cx="-236.884" cy="-171.086" r="0.006" gradientTransform="matrix(2157.515, 0, 0, -2491.283, 511096.688, -426208.482)" gradientUnits="userSpaceOnUse"><stop offset="0" stop-color="#93b4e7"/><stop offset="0.593" stop-color="#b9d1f8"/><stop offset="1" stop-color="#9cb6e0"/></radialGradient></defs><path d="M16,2.043,3.9,9.032V23.011L16,30l12.106-6.989V9.032Z" style="fill:url(#a)"/><path d="M16,9.133,10,12.6v6.932L16,23l6-3.466V12.6Z" style="fill:#6f95db"/><path d="M16,2,3.869,9.037,16,15.642,28.131,9.08Zm0,14.548L3.869,22.981,16,29.974l12.088-7.037L16,16.548Z" style="fill:#fff;fill-opacity:0.100000001490116"/></svg>
              </div>
              <span>{"Webpack"}</span>
            </div>
            <div class="yg-jalnan text-xs px-2 py-2 rounded-lg border-2 dark:border-slate-600 border-gray-400 flex items-center gap-x-1"> 
              <div class="inline-flex w-auto h-[24px]">
                <svg viewBox="0 0 410 404" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M399.641 59.5246L215.643 388.545C211.844 395.338 202.084 395.378 198.228 388.618L10.5817 59.5563C6.38087 52.1896 12.6802 43.2665 21.0281 44.7586L205.223 77.6824C206.398 77.8924 207.601 77.8904 208.776 77.6763L389.119 44.8058C397.439 43.2894 403.768 52.1434 399.641 59.5246Z" fill="url(#paint0_linear)"/><path d="M292.965 1.5744L156.801 28.2552C154.563 28.6937 152.906 30.5903 152.771 32.8664L144.395 174.33C144.198 177.662 147.258 180.248 150.51 179.498L188.42 170.749C191.967 169.931 195.172 173.055 194.443 176.622L183.18 231.775C182.422 235.487 185.907 238.661 189.532 237.56L212.947 230.446C216.577 229.344 220.065 232.527 219.297 236.242L201.398 322.875C200.278 328.294 207.486 331.249 210.492 326.603L212.5 323.5L323.454 102.072C325.312 98.3645 322.108 94.137 318.036 94.9228L279.014 102.454C275.347 103.161 272.227 99.746 273.262 96.1583L298.731 7.86689C299.767 4.27314 296.636 0.855181 292.965 1.5744Z" fill="url(#paint1_linear)"/><defs><linearGradient id="paint0_linear" x1="6.00017" y1="32.9999" x2="235" y2="344" gradientUnits="userSpaceOnUse"><stop stop-color="#41D1FF"/><stop offset="1" stop-color="#BD34FE"/></linearGradient><linearGradient id="paint1_linear" x1="194.651" y1="8.81818" x2="236.076" y2="292.989" gradientUnits="userSpaceOnUse"><stop stop-color="#FFEA83"/><stop offset="0.0833333" stop-color="#FFDD35"/><stop offset="1" stop-color="#FFA800"/></linearGradient></defs></svg>
              </div>
              <span>{"Vite"}</span>
            </div>
          </div>
          // <div class="space-y-1">
          //   <h2 class="font-semibold flex justify-between w-full">
          //     <span>{ "Javascript" }</span>
          //     <span>{ "75%" }</span>
          //   </h2>
          //   <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
          //     <div class="bg-blue-600 h-2.5 rounded-full relative" style="width: 75%"></div>
          //   </div>
          // </div>
          // <div class="space-y-1">
          //   <h2 class="font-semibold flex justify-between w-full">
          //     <span>{ "Typescript" }</span>
          //     <span>{ "76.5%" }</span>
          //   </h2>
          //   <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
          //     <div class="bg-blue-600 h-2.5 rounded-full relative" style="width: 76.5%"></div>
          //   </div>
          // </div>
          // <div class="space-y-1">
          //   <h2 class="font-semibold flex justify-between w-full">
          //     <span>{ "PHP" }</span>
          //     <span>{ "70%" }</span>
          //   </h2>
          //   <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
          //     <div class="bg-blue-600 h-2.5 rounded-full relative" style="width: 70%"></div>
          //   </div>
          // </div>
          // <div class="space-y-1">
          //   <h2 class="font-semibold flex justify-between w-full">
          //     <span>{ "Rust" }</span>
          //     <span>{ "50%" }</span>
          //   </h2>
          //   <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
          //     <div class="bg-blue-600 h-2.5 rounded-full relative" style="width: 50%"></div>
          //   </div>
          // </div>
        </div>
      </Card>
      // <Card 
      //   title="프레임워크 & 라이브러리"
      // >
        // <div class="space-y-3">
          // <div class="space-y-1">
          //   <h2 class="font-semibold flex justify-between w-full">
          //     <span>{ "React" }</span>
          //     <span>{ "81.5%" }</span>
          //   </h2>
          //   <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
          //     <div class="bg-blue-600 h-2.5 rounded-full relative" style="width: 81.5%"></div>
          //   </div>
          // </div>
          // <div class="space-y-1">
          //   <h2 class="font-semibold flex justify-between w-full">
          //     <span>{ "Vue" }</span>
          //     <span>{ "78.7%" }</span>
          //   </h2>
          //   <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
          //     <div class="bg-blue-600 h-2.5 rounded-full relative" style="width: 78.7%"></div>
          //   </div>
          // </div>
          // <div class="space-y-1">
          //   <h2 class="font-semibold flex justify-between w-full">
          //     <span>{ "Three.js" }</span>
          //     <span>{ "30%" }</span>
          //   </h2>
          //   <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
          //     <div class="bg-blue-600 h-2.5 rounded-full relative" style="width: 30%"></div>
          //   </div>
          // </div>
        // </div>
      // </Card>
      <Card 
        title="작업물"
      >        
        <div class="grid md:grid-cols-3 gap-x-4 overlfow-x-auto md:overflow-x-hidden overflow-x-scroll scrollbar-thin dark:scrollbar-thumb-gray-900 dark:scrollbar-track-gray-800 scrollbar-thumb-gray-400 scrollbar-track-gray-300 snap-x relative">
          <div class="space-y-2 w-[92.5vw] snap-x snap-center"> 
            <h2> {"할 일"} </h2>
            <div class="w-full bg-gray-100 rounded-full h-2 dark:bg-gray-700"></div>
          </div>
          <div class="space-y-2 w-[92.5vw] snap-x snap-center"> 
            <h2> {"진행 중"} </h2>
            <div class="w-full bg-gray-100 rounded-full h-2 dark:bg-gray-700"></div>
          </div>
          <div class="space-y-2 w-[92.5vw] snap-x snap-center"> 
            <h2> {"완료"} </h2>
            <div class="w-full bg-gray-100 rounded-full h-2 dark:bg-gray-700"></div>
          </div>
          <div class="flex items-center w-full py-3 col-span-3 space-x-2 sticky left-0">
            <div class="sticky left-0 space-x-2 flex items-center">
              <button class="text-xl w-5 h-5 inline-flex items-center justify-center bg-gray-100 hover:bg-gray-200 rounded dark:bg-slate-700 dark:hover:bg-slate-800 ease-in-out duration-200"><i class="ri-arrow-down-s-fill"></i></button>
              <span> {"Ohah"} </span>
            </div>
          </div>
          <div class="bg-gray-100 dark:bg-slate-700 md:min-h-[300px] rounded overflow-y-scroll md:max-h-[700px] max-h-[450px] scrollbar-thin dark:scrollbar-thumb-gray-900 dark:scrollbar-track-gray-800 scrollbar-thumb-gray-400 scrollbar-track-gray-300">
            <KanbanCard 
              title="2022.08 ~"
              description="React 강의 영상 준비"
              tag={vec!["Typescript", "React 18", "Vite" ]}
              epic={TagProps::Epic}
            >
            </KanbanCard>
          </div>
          <div class="bg-gray-100 dark:bg-slate-700 md:min-h-[300px] rounded overflow-y-auto md:max-h-[700px] max-h-[450px] scrollbar-thin dark:scrollbar-thumb-gray-900 dark:scrollbar-track-gray-800 scrollbar-thumb-gray-400 scrollbar-track-gray-300">
            <KanbanCard 
              title="2021.07 ~"
              description="hwpjs(hwpx) 만들기"
              tag={vec!["typescript"]}
              epic={TagProps::Bug}
            >
              <p> {"공식 문서의 불친절함으로 무기한 보류 상태"} </p>
              <p> <a href="https://github.com/ohah/hwpjs" target="_blank"> <i class="text-2xl hover:text-slate-700 dark:hover:text-slate-200 ri-github-fill"> </i> </a> </p>
            </KanbanCard>
          </div>
          <div class="bg-gray-100 dark:bg-slate-700 md:min-h-[300px] rounded overflow-y-auto md:max-h-[700px] max-h-[450px] scrollbar-thin dark:scrollbar-thumb-gray-900 dark:scrollbar-track-gray-800 scrollbar-thumb-gray-400 scrollbar-track-gray-300">
            <KanbanCard 
              title="2022.04 ~ 2022.05"
              description="호텔리조트 홈페이지 개발"
              tag={vec!["javascript", "vue2"]}
              epic={TagProps::Complete}
            >
              <p> {"특이사항 : 프로젝트가 엎어짐"} </p>
            </KanbanCard>
            <KanbanCard 
              title="2022.03 ~ 2022.04"
              description="문화예술원 예약, 예술인 등록 개발"
              tag={vec!["php", "javascript"]}
              epic={TagProps::Complete}
            >
              <p> {"메인 작업 까지 추가 되어 작업하게 됨"} </p>
            </KanbanCard>
            <KanbanCard 
              title="2022.02 ~ 2022.03"
              description="과자 회사 이벤트 페이지 개발"
              tag={vec!["php", "javascript"]}
              epic={TagProps::Complete}
            >
              <p> {"이미 퍼블리싱된 화면과 모달창(PSD)이 있었으나 반응형에 맞지 않아 커스텀하는데 고생"} </p>
            </KanbanCard>
            <KanbanCard 
              title="2022.01"
              description="워드프레스 플러그인 기능 수정"
              tag={vec!["PHP", "javascript"]}
              epic={TagProps::Complete}
            >
            <div>
              <p>{"Vimeo, Udemy에서 나오는 영어자막을 번역 api를 활용하여 실시간 번역"}</p>
              <p>{"해당 영상에 띄워주는 크롬 익스텐션 개발"}</p>
            </div>
            </KanbanCard>
            <KanbanCard 
              title="2021.12 ~ 2022.01"
              description="체육관 대관 및 예약 시스템 개발"
              tag={vec!["PHP", "javascript"]}
              epic={TagProps::Complete}
            >
              <p> {"제작 도중 요구사항이 많이 바뀌었었음"} </p>
            </KanbanCard>
            <KanbanCard 
              title="2021.02 ~ 2021.03"
              description="한의원 홈페이지 닥톡과 연결"
              tag={vec!["PHP", "javascript"]}
              epic={TagProps::Complete}
            >
              <p> {"닥톡에서는 javascript, spring 예제코드만 제공하여 해당 부분 참고하여 php api 제작"} </p>
              <p> {"api 정의서가 명확하지 않아 닥톡에 메일 보내느라 작업기간이 오래 걸림"} </p>
            </KanbanCard>
            <KanbanCard 
              title="2021.02 ~ 2021.02"
              description="모바일 사이트, 명함 홈페이지 퍼블리싱"
              tag={vec!["Css", "javascript"]}
              epic={TagProps::Complete}
            >
            </KanbanCard>
            <KanbanCard 
              title="2020.05 ~ 2020.06"
              description="금 거래소 실시간 반영 차트"
              tag={vec!["IE", "javascript"]}
              epic={TagProps::Complete}
            >
              <p> {"IE 속도 이슈, 데이터 처리 이슈"} </p>
              <p> {"api, javascript 렌더링 부분 최적화"} </p>
            </KanbanCard>
            <KanbanCard 
              title="2020.02 ~ 2020.05"
              description="리액트 포인트 게임 퍼블리싱 오류 및 수정"
              tag={vec!["React"]}
              epic={TagProps::Complete}
            >
              <p> {"Class형 컴포넌트의 옛날 형태의 리액트"} </p>
              <p> {"document.createElement와 react.render를 react내에서 또 쓰는 난잡한 코딩의 형태였음"} </p>
            </KanbanCard>
            <KanbanCard 
              title="2020.02"
              description="IE 11 호환성 작업"
              tag={vec!["React"]}
              epic={TagProps::Complete}
            >
            </KanbanCard>
            <KanbanCard 
              title="2019.01 ~ 2019.07"
              description="노노그램 게임 개발"
              tag={vec!["javascript"]}
              epic={TagProps::Complete}
            >
              <p> {"IE, IOS, IPad, Android, Mobile, PC 등 대부분의 기기에서 호환성 테스트때문에 코드가 난잡해짐."} </p>
              <p> {"기존 내용보다 추가적인 요구사항이 더 많아 오래 걸렸던 건"} </p>
              <p> {"유명 유튜버가 해당 사이트에서 게임을 하는걸 우연히 본게 기억에 남는 작업물"} </p>
            </KanbanCard>
            <KanbanCard 
              title="2019.01 ~ 2019.02"
              description="실시간 코인 테이블 개발"
              tag={vec!["javascript"]}
              epic={TagProps::Complete}
            >
            <p> {"코인(바이낸스, 빗썸, 업비트)간의 시세차이, 현재 시세, 전일가, 거래가, 현재가 등을 각각의 api에 연결하여 종합하여 테이블에 보여주는 형태의 페이지 개발"} </p>
            </KanbanCard>
          </div>
        </div>
      </Card>
      <Card 
        title="홈페이지 패치 내역"
      >        
        <div class="grid md:grid-cols-3 gap-x-4 overlfow-x-auto md:overflow-x-hidden overflow-x-scroll scrollbar-thin dark:scrollbar-thumb-gray-900 dark:scrollbar-track-gray-800 scrollbar-thumb-gray-400 scrollbar-track-gray-300 snap-x">
          <div class="space-y-2 w-[92.5vw] snap-x snap-center"> 
            <h2> {"할 일"} </h2>
            <div class="w-full bg-gray-100 rounded-full h-2 dark:bg-gray-700"></div>
          </div>
          <div class="space-y-2 w-[92.5vw] snap-x snap-center"> 
            <h2> {"진행 중"} </h2>
            <div class="w-full bg-gray-100 rounded-full h-2 dark:bg-gray-700"></div>
          </div>
          <div class="space-y-2 w-[92.5vw] snap-x snap-center"> 
            <h2> {"완료"} </h2>
            <div class="w-full bg-gray-100 rounded-full h-2 dark:bg-gray-700"></div>
          </div>
          <div class="flex items-center w-full py-3 col-span-3 space-x-2 sticky left-0">
            <div class="sticky left-0 space-x-2 flex items-center">
              <button class="text-xl w-5 h-5 inline-flex items-center justify-center bg-gray-100 hover:bg-gray-200 rounded dark:bg-slate-700 dark:hover:bg-slate-800 ease-in-out duration-200"><i class="ri-arrow-down-s-fill"></i></button>
              <span> {"Ohah"} </span>
            </div>
          </div>
          <div class="bg-gray-100 dark:bg-slate-700 md:min-h-[300px] rounded overflow-y-auto md:max-h-[700px] max-h-[450px] scrollbar-thin dark:scrollbar-thumb-gray-900 dark:scrollbar-track-gray-800 scrollbar-thumb-gray-400 scrollbar-track-gray-300">
            <KanbanCard 
              title="2022.09.17 ~"
              description="글쓰기 에디터 버그"
              tag={vec!["Rust", "Wasm", "typescript", "mvmEditor"]}
              epic={TagProps::Bug}
            > 
              <p>{ "이미지 업로드 실패시 다음 이미지가 업로드가 되지 않음" }</p>
            </KanbanCard>
            <KanbanCard 
              title="미정"
              description="Toast 메시지 버그 수정"
              tag={vec!["Rust", "Wasm"]}
              epic={TagProps::Bug}
            >
              <p>{ "Tailwind 기반이므로 금방 작업이 될 것으로 예상" }</p>
            </KanbanCard>
            <KanbanCard 
              title="미정"
              description="유저 정보"
              tag={vec!["Rust", "Wasm"]}
              epic={TagProps::Bookmark}
            >
              <p>{ "아이디 클릭시 관련 드롭다운 컴포넌트 생성" }</p>
            </KanbanCard>
            <KanbanCard 
              title="미정"
              description="글 뷰페이지 추가 개발"
              tag={vec!["Rust", "Wasm"]}
              epic={TagProps::Question}
            > 
             <p> {"TOC 추가?"} </p>
             <p> {"글 좋아요, 이모지 추가"} </p>
            </KanbanCard>
          </div>
          <div class="bg-gray-100 dark:bg-slate-700 md:min-h-[300px] rounded overflow-y-auto md:max-h-[700px] max-h-[450px] scrollbar-thin dark:scrollbar-thumb-gray-900 dark:scrollbar-track-gray-800 scrollbar-thumb-gray-400 scrollbar-track-gray-300">
            <KanbanCard 
              title="2022.09.30 ~"
              description="모바일 반응형 개발"
              tag={vec!["Rust", "Wasm", "typescript"]}
              epic={TagProps::Bookmark}
            >
              <p>{ "Tailwind 기반이므로 금방 작업이 될 것으로 예상" }</p>
            </KanbanCard>
            <KanbanCard 
              title="2022.09.22 ~"
              description="검색 기능 추가"
              tag={vec!["Rust", "Wasm"]}
              epic={TagProps::Bookmark}
            >
              <p>{ "검색 컴포넌트 (헤더, 목록) 추가" }</p>
            </KanbanCard>
            <KanbanCard 
              title="2022.08 ~"
              description="러스트 SPA 블로그 만들기"
              tag={vec!["WebAssembly", "javascript", "rust", "php", "typescript"]}
              epic={TagProps::Complete}
            >
              <p>{ "기본적인 CRUD 생성 완료" }</p>
            </KanbanCard>
            <KanbanCard 
              title="2022.09.10 ~"
              description="댓글 추가 개발"
              tag={vec!["Rust", "Wasm"]}
              epic={TagProps::Work}
            > 
             <p> {"댓글도 에디터 적용?"} </p>
             <p> {"댓글 좋아요, 이모지 추가"} </p>
            </KanbanCard>
          </div>
          <div class="bg-gray-100 dark:bg-slate-700 md:min-h-[300px] rounded overflow-y-auto md:max-h-[700px] max-h-[450px] scrollbar-thin dark:scrollbar-thumb-gray-900 dark:scrollbar-track-gray-800 scrollbar-thumb-gray-400 scrollbar-track-gray-300">
            <KanbanCard 
              title="2022.09.17 ~ 2022.09.19"
              description="카테고리, 글쓰기 개선, 에디터 일부 버그 수정"
              tag={vec!["Rust", "Wasm"]}
              epic={TagProps::Bookmark}
            >
              <p>{ "글쓰기 인풋박스가 아닌 커스텀 컴포넌트 개발" }</p>
              <p>{ "태그입력, 카테고리 커스텀 컴포넌트 개발" }</p>
              <p>{ "최초 접속시 에디터 에러 버그 수정" }</p>
            </KanbanCard>
            <KanbanCard 
              title="2022.09.10 ~"
              description="댓글 개발"
              tag={vec!["Rust", "Wasm", "typescript"]}
              epic={TagProps::Complete}
            > 
              <p> {"댓글 기본 CURD 개발"} </p>
              <p> {"수정, 대댓글 기능"} </p>
              <p> {"Github Like"} </p>
            </KanbanCard>
          </div>
        </div>
      </Card>
    </article>
  }
}