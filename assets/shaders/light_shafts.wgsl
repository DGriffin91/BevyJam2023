#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

#import bevy_pbr::utils

struct Material {
    color: vec3<f32>,
    shaft: f32,
    dust: f32,
    dust_size: f32,
    dust_qty: f32,
}

@group(1) @binding(0)
var<uniform> material: Material;

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

    var fresnel = dot(N, -V);
    fresnel = saturate(fresnel * fresnel * 2.0) + 0.4;

    var dust = noise((in.world_position.xyz * 250.0) / material.dust_size + globals.time * 0.5);
    let dist_size = saturate(1.0/Z - 0.5 * 1.9 + 0.5);
    dust = (dust - 0.9 - 0.05 * dist_size - material.dust_qty) * 7.0;
    var coarse_noise = (noise(in.world_position.xyz * 8.0 + globals.time * 0.1) - 0.5) * 2.0 + 0.35;
    dust *= saturate(coarse_noise) * (1.8 - dist_size);
    dust = saturate(dust * shaft * 1.7);

    shaft = shaft * 0.04 * material.shaft;
    dust = dust * material.dust;

    // TODO adjust dust size by window res so we don't alias?

    var col = dust + shaft;

    col *= saturate(Z - 0.1);
    col *= 1.2 * fresnel;

    return vec4(vec3(col * material.color), 0.0);
}