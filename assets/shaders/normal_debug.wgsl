struct FragmentInput {
    #import bevy_pbr::mesh_vertex_output
};
// struct FragmentInput {
//     @location(0) world_position: vec4<f32>,
//     @location(1) world_normal: vec3<f32>,
//     #ifdef VERTEX_UVS
//     @location(2) uv: vec2<f32>,
//     #endif
//     #ifdef VERTEX_TANGENTS
//     @location(3) world_tangent: vec4<f32>,
//     #endif
//     #ifdef VERTEX_COLORS
//     @location(4) color: vec4<f32>,
//     #endif
// };

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.world_normal, 1.);
}
