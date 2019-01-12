async function init() {
  await wasm_bindgen("dist/wasm_custom_allocator_example_bg.wasm");
  self.wasm = wasm_bindgen;
}

function dumpResult(message) {
  const el = document.createElement("div");
  el.innerHTML = message;
  document.body.appendChild(el);
}

function testAllocateManyArrays() {
  const expected = 499500;
  for (let count = 0; count < 10000; ++count) {
    const arr = [];
    for (let i = 0; i < 1000; i++) {
      arr.push(i);
    }
    const sum = wasm.add_values(arr);
    if (sum !== expected) {
      throw new Error(`Unexpected result: ${sum} !== ${expected}`);
    }
  }
  dumpResult("OK: testAllocateManyArrays");
}

function testLevenshteinDistance() {
  const testcases = [
    { s: "kitten", t: "sitting", expected: 3 },
    { s: "Saturday", t: "Sunday", expected: 3 },
    { s: "apple", t: "pineapple", expected: 4 }
  ];
  for (let testcase of testcases) {
    const actual = wasm.levenshtein_distance(testcase.s, testcase.t);
    if (actual !== testcase.expected) {
      throw new Error(`Unexpected result: ${actual} != ${testcase.expected}`);
    }
  }
  dumpResult("OK: testLevenshteinDistance");
}

document.addEventListener("DOMContentLoaded", async () => {
  await init();

  testAllocateManyArrays();
  testLevenshteinDistance();
});
