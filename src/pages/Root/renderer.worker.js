onmessage = async function (event) {
  const { Renderer } = await import("renderer");
  const { canvas } = event.data;
  const context = canvas.getContext("webgpu");
  const adapter = await navigator.gpu.requestAdapter();
  const device = await adapter.requestDevice();
  const renderer = new Renderer(context, adapter, device);

  function render() {
    renderer.frame();
    requestAnimationFrame(render);
  }
  render();

  const src = new Float32Array(100);
  src.fill(1);
  await renderer.map_write();
  renderer.unmap_write(src);
  renderer.compute();
  const dst = new Float32Array(100);
  await renderer.map_read();
  renderer.unmap_read(dst);
  console.log(src, dst);
};
