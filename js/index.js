import * as PIXI from './pixi.min.js';
const rust = import("./../wasm_bindgen/wasm_bindgen_hello");

let app = new PIXI.Application({
  width: 512,
  height: 256,
  antialias: true,
  transparent: false
});

document.body.appendChild(app.view);

app.renderer.backgroundColor = 0x20202e;

rust.then(rust => {
  rust.greet("World");
  //rust.add_paragraph("Hello, World!");

  let type = "WebGL";
  if(!PIXI.utils.isWebGLSupported()){
    type = "canvas";
  }

  PIXI.utils.sayHello(type);

  PIXI.loader
    .add("testface", "./img/testface.png")
    .load(setup);
});

function setup() {
    let sprite = new PIXI.Sprite(
        PIXI.loader.resources["testface"].texture
    );
    sprite.position.set(256, 128);
    sprite.anchor.set(0.5, 0.5);
    app.stage.addChild(sprite);
}
