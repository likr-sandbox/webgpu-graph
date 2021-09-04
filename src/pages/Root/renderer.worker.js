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
};
