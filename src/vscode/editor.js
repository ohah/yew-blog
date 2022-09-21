let editor;
export async function editor_init(element, value = '') {
  const theme = localStorage.getItem("theme") && JSON.parse(localStorage.getItem("theme")).color === "dark" ? "dark" : "";
  console.log(theme);
  editor = await new VSCode({
    element : element,
    theme:`vs${theme === "" ? "" : `-${theme}`}`,
    height:`${document.querySelector(element).getBoundingClientRect().height - 27}px`,
    // height:`500px`,
    // markdownStyle : `github-${theme}`,
    markdownStyle: `github${theme === "" ? "-light" : `-${theme}`}`,
    preview: true,
    path: "https://cdn.jsdelivr.net/gh/ohah/mvmEditor@master/src/",
    imageUpload : function (files) {
      return new Promise(function(resolve, reject) {
        const f = new FormData();
        Object.values(files).forEach((file, i) => {
          console.log(`editor_image[${i}]`, file, file.name);
          f.append(`editor_image[${i}]`, file, file.name);
        });
        const check = Object.values(files).every((file, i) => {
          return file.size > 1024 * 1024 * 1024 * 6;
        });
        if ( check === true) {
          alert("6MB 이상의 이미지를 업로드 할 수 없습니다");
          reject('실패');
        } else {
          fetch("/api/image_upload", {
            method:"post",
            body:f,
          }).then(res=>res.json())
          .then((res)=>resolve({url:res.url}))
          .catch((err)=>console.log(err));
        }
        // resolve({url :res.data.tistory.url});
      });
    },
    value: value,
    mobile:true,
  });
  await editor.initialize();
  await editor.editor.getModel().onDidChangeContent((event) => {
    document.querySelector(`[data-id="${element}"]`).value = editor.getValue();
    document.querySelector(`[data-id="${element}"]`).dispatchEvent(new Event('change'));
  });
}
export async function setValue(value) {
  return editor.setValue(value);
}