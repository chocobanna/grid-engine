struct Out {
    @builtin(position) pos: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> Out {

    var verts = array<vec2<f32>,3>(
        vec2<f32>(0.0,0.3),
        vec2<f32>(-0.3,-0.3),
        vec2<f32>(0.3,-0.3)
    );

    var out: Out;
    out.pos = vec4<f32>(verts[i],0.0,1.0);
    return out;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0,0.5,0.2,1.0);
}