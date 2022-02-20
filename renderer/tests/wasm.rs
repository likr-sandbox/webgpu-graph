extern crate renderer;

use js_sys::Float32Array;
use renderer::Renderer;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{window, GpuAdapter, GpuCanvasContext, GpuDevice, HtmlCanvasElement};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn shortest_path() {
  let n = 4096;

  let canvas = window()
    .unwrap()
    .document()
    .unwrap()
    .create_element("canvas")
    .unwrap()
    .dyn_into::<HtmlCanvasElement>()
    .unwrap();
  let context = canvas
    .get_context("webgpu")
    .unwrap()
    .unwrap()
    .dyn_into::<GpuCanvasContext>()
    .unwrap();
  let adapter = JsFuture::from(window().unwrap().navigator().gpu().request_adapter())
    .await
    .unwrap()
    .dyn_into::<GpuAdapter>()
    .unwrap();
  let device = JsFuture::from(adapter.request_device())
    .await
    .unwrap()
    .dyn_into::<GpuDevice>()
    .unwrap();
  let renderer = Renderer::new(&context, &adapter, &device, n as usize).unwrap();

  let src = Float32Array::new_with_length(n * n);
  src.fill(std::f32::INFINITY, 0, n * n);
  for i in 0..n {
    src.set_index(i * n + i, 0.)
  }
  for i in 1..n {
    let j = i - 1;
    src.set_index(i * n + j, 1.);
    src.set_index(j * n + i, 1.);
  }

  JsFuture::from(renderer.map_write()).await.unwrap();
  renderer.unmap_write(&src);
  renderer.compute();
  let dst = Float32Array::new_with_length(n * n);
  JsFuture::from(renderer.map_read()).await.unwrap();
  renderer.unmap_read(&dst);

  for i in 0..n {
    for j in 0..=i {
      assert_eq!(dst.get_index(i * n + j), (i - j) as f32);
    }
  }
}
