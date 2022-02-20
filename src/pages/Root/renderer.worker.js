onmessage = async function (event) {
  const n = 1024 * 4;
  const { Renderer } = await import("renderer");
  const { canvas } = event.data;
  const context = canvas.getContext("webgpu");
  const adapter = await navigator.gpu.requestAdapter();
  const device = await adapter.requestDevice();
  console.log(adapter, device);
  const renderer = new Renderer(context, adapter, device, n);

  function render() {
    renderer.frame();
    requestAnimationFrame(render);
  }
  render();

  const src = new Float32Array(n * n);
  src.fill(Infinity);
  for (let i = 0; i < n; ++i) {
    src[i * n + i] = 0;
  }
  for (let i = 1; i < n; ++i) {
    let j = i - 1;
    src[i * n + j] = 1;
    src[j * n + i] = 1;
  }
  await renderer.map_write();
  renderer.unmap_write(src);
  const start = performance.now();
  renderer.compute();
  const stop = performance.now();
  const dst = new Float32Array(n * n);
  await renderer.map_read();
  renderer.unmap_read(dst);
  console.log(src, dst);
  console.log(stop - start);
};
