async function init() {
  let wasm = await wasm_bindgen("wasm_custom_allocator_test_bg.wasm");
  console.log(wasm);
  self.wasm = wasm;
}

document.addEventListener("DOMContentLoaded", () => {
  init();
});
