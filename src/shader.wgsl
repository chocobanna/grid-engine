// Vertex shader — emits three points
@vertex
fn vs_main(@builtin(vertex_index) idx : u32) -> @builtin(position) vec4<f32> {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>(0.5, -0.5),
    );

    let pos = positions[idx];
    return vec4<f32>(pos, 0.0, 1.0);
}

// Fragment shader — solid white
@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
