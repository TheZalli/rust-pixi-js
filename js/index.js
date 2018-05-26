const rust = import("./../wasm_bindgen/wasm_bindgen_hello");

rust.then(rust => {
  rust.greet("World");
  rust.add_paragraph("Hello, World!");
});
