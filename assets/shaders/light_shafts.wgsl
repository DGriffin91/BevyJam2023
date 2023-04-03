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


#import "shaders/common.wgsl"

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var N = normalize(in.world_normal);
    var V = normalize(view.world_position.xyz - in.world_position.xyz);
    var Z = distance(view.world_position.xyz, in.world_position.xyz);
    var shaft = distance(in.uv, vec2(0.5, 0.5));
    shaft = pow(shaft, 0.6) * 1.6;
    shaft = saturate(1.0 - shaft);

    var dust = noise(in.world_position.xyz * 250.0 + globals.time * 0.5);
    let dist_size = saturate(1.0/Z - 0.5 * 1.9 + 0.5);
    dust = (dust - 0.9 - 0.05 * dist_size) * 7.0;
    var coarse_noise = (noise(in.world_position.xyz * 8.0 + globals.time * 0.1) - 0.5) * 2.0 + 0.35;
    dust *= saturate(coarse_noise) * (1.8 - dist_size);
    dust = saturate(dust * shaft * 1.7);

    shaft = shaft * 0.04;

    // TODO adjust dust size by window res so we don't alias?


    return vec4(vec3(dust + shaft), 0.0);
}