struct Uniforms {
    offset: vec2<f32>,
    rotation: f32,
    pad: f32,
};

@group(0) @binding(0)
var<uniform> u: Uniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VertexOutput {

    var pos = array<vec2<f32>,3>(
        vec2<f32>(0.0,0.2),
        vec2<f32>(-0.2,-0.2),
        vec2<f32>(0.2,-0.2)
    );

    let p = pos[i];

    let c = cos(u.rotation);
    let s = sin(u.rotation);

    let rotated = vec2<f32>(
        p.x * c - p.y * s,
        p.x * s + p.y * c
    );

    var out: VertexOutput;

    out.position = vec4<f32>(rotated + u.offset,0.0,1.0);

    return out;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0,0.5,0.2,1.0);
}