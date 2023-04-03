#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

const TAU: f32 = 6.28318530717958647692528676655900577;

fn dir_to_equirectangular(dir: vec3<f32>) -> vec2<f32> {
    let x = atan2(dir.z, dir.x) / TAU + 0.5; // 0-1
    let y = acos(dir.y) / PI; // 0-1
    return vec2<f32>(x, y);
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var N = normalize(in.world_normal);
    var V = normalize(view.world_position.xyz - in.world_position.xyz);
    return textureSample(texture, texture_sampler, dir_to_equirectangular(-V));
}