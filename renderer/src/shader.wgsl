[[stage(vertex)]]
fn vertex_main([[builtin(vertex_index)]] VertexIndex : u32)
     -> [[builtin(position)]] vec4<f32> {
  var pos = array<vec2<f32>, 3>(
      vec2<f32>(0.0, 0.5),
      vec2<f32>(-0.5, -0.5),
      vec2<f32>(0.5, -0.5));

  return vec4<f32>(pos[VertexIndex], 0.0, 1.0);
}

[[stage(fragment)]]
fn fragment_main() -> [[location(0)]] vec4<f32> {
  return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}

[[block]] struct WfBuffer {
  buffer : [[stride(4)]] array<f32>;
};
[[block]] struct WfParams {
  size : u32;
  k : u32;
};
[[group(0), binding(0)]] var<storage, read> wf_buffer_in: WfBuffer;
[[group(0), binding(1)]] var<storage, write> wf_buffer_out: WfBuffer;
[[group(0), binding(2)]] var<uniform> wf_params: WfParams;

[[stage(compute), workgroup_size(16, 16)]]
fn warshall_floyd([[builtin(global_invocation_id)]] GlobalInvocationID : vec3<u32>) {
  var i : u32 = GlobalInvocationID.x;
  var j : u32 = GlobalInvocationID.y;
  var k : u32 = wf_params.k;
  var n : u32 = wf_params.size;
  if (i < n && j < n) {
    wf_buffer_out.buffer[i * n + j] = min(
      wf_buffer_in.buffer[i * n + j],
      wf_buffer_in.buffer[i * n + k] + wf_buffer_in.buffer[k * n + j]
    );
  }
}
