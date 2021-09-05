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

[[block]] struct Buffer {
  buffer : [[stride(4)]] array<f32>;
};

[[group(0), binding(0)]] var<storage, read> buffer_in: Buffer;
[[group(0), binding(1)]] var<storage, write> buffer_out: Buffer;

[[stage(compute), workgroup_size(64)]]
fn compute_main([[builtin(global_invocation_id)]] GlobalInvocationID : vec3<u32>) {
  var index : u32 = GlobalInvocationID.x;
  buffer_out.buffer[index] = buffer_in.buffer[index] + 1.;
}
