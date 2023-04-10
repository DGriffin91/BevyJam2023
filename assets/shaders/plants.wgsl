#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

#import bevy_pbr::utils
#import bevy_pbr::fog
#import "shaders/fog.wgsl"


struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};


#import "shaders/common.wgsl"

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    //return vec4(vec3(in.color.rgb), 1.0);
    //var col = vec3(0.003);
    var col = vec3(0.003,0.004,0.003);
//#ifdef VERTEX_COLORS
//    col += in.color.rgb;
//#endif
    // fog
    if (fog.mode != FOG_MODE_OFF) {
        col = apply_fog_c(vec4(col, 1.0), in.world_position.xyz, view.world_position.xyz, 1.0).xyz;
    } 
    return vec4(col, 1.0);
}