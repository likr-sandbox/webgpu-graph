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
fn warshall_floyd_kernel([[builtin(global_invocation_id)]] GlobalInvocationID : vec3<u32>) {
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


let workgroup_size_x : u32 = 16u;
let workgroup_size_y : u32 = 16u;

var<workgroup> wf_a : array<f32, 256>;
var<workgroup> wf_b : array<f32, 256>;
var<workgroup> wf_c : array<f32, 256>;

[[stage(compute), workgroup_size(workgroup_size_x, workgroup_size_y)]]
fn blocked_warshall_floyd_kernel(
  [[builtin(global_invocation_id)]] GlobalInvocationID : vec3<u32>,
  [[builtin(local_invocation_id)]] LocalInvocationID : vec3<u32>,
  [[builtin(num_workgroups)]] NumWorkgroups : vec3<u32>,
) {
}

[[stage(compute), workgroup_size(workgroup_size_x, workgroup_size_y)]]
fn tropical_matmul(
  [[builtin(global_invocation_id)]] GlobalInvocationID : vec3<u32>,
  [[builtin(local_invocation_id)]] LocalInvocationID : vec3<u32>,
  [[builtin(workgroup_id)]] WorkgroupID : vec3<u32>,
) {
  var i : u32 = GlobalInvocationID.x;
  var j : u32 = GlobalInvocationID.y;
  var x : u32 = LocalInvocationID.x;
  var y : u32 = LocalInvocationID.y;
  var n : u32 = wf_params.size;

  var s : f32 = 100000000.;
  var k : u32 = u32(0);
  loop {
    if (workgroup_size_x * k >= n) {
      break;
    }
    workgroupBarrier();
    wf_a[y * workgroup_size_x + x] = wf_buffer_in.buffer[(workgroup_size_y * k + y) * n + (workgroup_size_x * WorkgroupID.x + x)];
    wf_b[y * workgroup_size_x + x] = wf_buffer_in.buffer[(workgroup_size_y * WorkgroupID.y + y) * n + (workgroup_size_x * k + x)];
    workgroupBarrier();
    var z : u32 = u32(0);
    loop {
      if (z >= workgroup_size_x) {
        break;
      }
      s = min(s, wf_a[y * workgroup_size_x + z] + wf_b[z * workgroup_size_x + x]);
      z = z + 1u;
    }
    k = k + 1u;
  }
  wf_buffer_out.buffer[i * n + j] = s;
}


