#import "shaders/bicubic.wgsl"

#import bevy_pbr::mesh_view_bindings
//#import bevy_pbr::pbr_bindings

struct CustomStandardMaterial {
    base_color: vec4<f32>,
    emissive: vec4<f32>,
    perceptual_roughness: f32,
    metallic: f32,
    reflectance: f32,
    // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    flags: u32,
    alpha_cutoff: f32,
    env_spec: f32,
    env_diff: f32,
    emit_mult: f32,
};

#import bevy_pbr::pbr_types
@group(1) @binding(0)
var<uniform> material: CustomStandardMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;
@group(1) @binding(3)
var emissive_texture: texture_2d<f32>;
@group(1) @binding(4)
var emissive_sampler: sampler;
@group(1) @binding(5)
var metallic_roughness_texture: texture_2d<f32>;
@group(1) @binding(6)
var metallic_roughness_sampler: sampler;
@group(1) @binding(7)
var detail_texture: texture_2d<f32>;
@group(1) @binding(8)
var detail_sampler: sampler;
@group(1) @binding(9)
var normal_map_texture: texture_2d<f32>;
@group(1) @binding(10)
var normal_map_sampler: sampler;

#import bevy_pbr::mesh_bindings

#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::pbr_ambient
//#import bevy_pbr::shadows
#import "shaders/shadows.wgsl"
#import bevy_pbr::fog
#import "shaders/fog.wgsl"
//#import bevy_pbr::pbr_functions
#import "shaders/pbr_functions.wgsl"

#import "shaders/grass.wgsl"

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

#import "shaders/common.wgsl"

@fragment
fn fragment_fast(in: FragmentInput) -> @location(0) vec4<f32> {
    var emit_image = textureSampleBicubic(emissive_texture, emissive_sampler, in.uv).rgb;
    return vec4(emit_image, 1.0);
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var N = normalize(in.world_normal);
    var V = normalize(view.world_position.xyz - in.world_position.xyz);

    var output_color: vec4<f32> = material.base_color;
    var metallic = material.metallic; // Griffin
    var perceptual_roughness = material.perceptual_roughness; // Griffin
// Used for material props
var use_vertex_colors = false;
var direct_light_mult = 0.001;
#ifdef VERTEX_COLORS
    direct_light_mult = 0.0;
    //output_color = output_color * in.color;
    metallic = in.color.g; // Griffin
    perceptual_roughness = saturate(in.color.r); // Griffin
#endif
#ifdef VERTEX_UVS
    if ((material.flags & STANDARD_MATERIAL_FLAGS_BASE_COLOR_TEXTURE_BIT) != 0u) {
        output_color = output_color * textureSample(base_color_texture, base_color_sampler, in.uv);
    }
#endif


    
    // ---------------- Texture noise

    var detail = 0.0;
#ifdef GRASS
  output_color = vec4(grass(V, in.world_position.xz * 0.08), 1.0);
#else
    detail += textureSample(detail_texture, detail_sampler, in.uv * 0.1).r * 1.5;
    detail += textureSample(detail_texture, detail_sampler, in.uv * 0.8).r * 0.5;
    detail += textureSampleBias(detail_texture, detail_sampler, in.uv * 8.0, -1.5).r * 1.5;
#endif
    // ---------------- Texture noise

    // NOTE: Unlit bit not set means == 0 is true, so the true case is if lit
    if ((material.flags & STANDARD_MATERIAL_FLAGS_UNLIT_BIT) == 0u) {
        // Prepare a 'processed' StandardMaterial by sampling all textures to resolve
        // the material members
        var pbr_input: PbrInput;
        pbr_input.material.base_color = output_color;
        pbr_input.material.reflectance = material.reflectance;
        pbr_input.material.flags = material.flags;
        pbr_input.material.alpha_cutoff = material.alpha_cutoff;

        // TODO use .a for exposure compensation in HDR
        var emissive: vec4<f32> = material.emissive;
#ifdef VERTEX_UVS
        if ((material.flags & STANDARD_MATERIAL_FLAGS_EMISSIVE_TEXTURE_BIT) != 0u) {
            var emit_image = textureSampleBicubic(emissive_texture, emissive_sampler, in.uv).rgb;
            emissive = vec4<f32>(emissive.rgb * pow(emit_image, vec3(1.3)), 1.0) * material.emit_mult;
        }
#endif
        pbr_input.material.emissive = emissive;

#ifdef VERTEX_UVS
        if ((material.flags & STANDARD_MATERIAL_FLAGS_METALLIC_ROUGHNESS_TEXTURE_BIT) != 0u) {
            let metallic_roughness = textureSample(metallic_roughness_texture, metallic_roughness_sampler, in.uv);
            // Sampling from GLTF standard channels for now
            metallic = metallic * metallic_roughness.b;
            perceptual_roughness = perceptual_roughness * metallic_roughness.g;
        }
#endif
        pbr_input.material.metallic = metallic;
        pbr_input.material.perceptual_roughness = perceptual_roughness;

        var occlusion: f32 = 1.0;
//#ifdef VERTEX_UVS
//        if ((material.flags & STANDARD_MATERIAL_FLAGS_OCCLUSION_TEXTURE_BIT) != 0u) {
//            occlusion = textureSample(occlusion_texture, occlusion_sampler, in.uv).r;
//        }
//#endif
        pbr_input.frag_coord = in.frag_coord;
        pbr_input.world_position = in.world_position;
        pbr_input.world_normal = prepare_world_normal(
            in.world_normal,
            (material.flags & STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT) != 0u,
            in.is_front,
        );

        pbr_input.is_orthographic = view.projection[3].w == 1.0;

        pbr_input.N = apply_normal_mapping(
            material.flags,
            pbr_input.world_normal,
#ifdef VERTEX_TANGENTS
#ifdef STANDARDMATERIAL_NORMAL_MAP
            in.world_tangent,
#endif
#endif
#ifdef VERTEX_UVS
            in.uv,
#endif
        );
        pbr_input.V = calculate_view(in.world_position, pbr_input.is_orthographic);
        pbr_input.occlusion = occlusion;

        pbr_input.flags = mesh.flags;
#ifdef GRASS
        output_color *= pbr(pbr_input, direct_light_mult) * 3.0;
#else
        output_color = pbr(pbr_input, direct_light_mult);
#endif



        // ---------------- detail noise
        output_color = mix(output_color, output_color * vec4(vec3(detail), 1.0), 0.175);


    } else {
        output_color = alpha_discard(material, output_color);
    }

    // fog
    if (fog.mode != FOG_MODE_OFF && (material.flags & STANDARD_MATERIAL_FLAGS_FOG_ENABLED_BIT) != 0u) {
        output_color = apply_fog_c(output_color, in.world_position.xyz, view.world_position.xyz, 1.0);
    }



    // ---------------- noise
    
    // TODO make optional, maybe put in post proc shader (with fxaa?)
    var uv_rand = in.frag_coord.xy / vec2<f32>(view.viewport.zw);
    uv_rand.y *= random(vec2(uv_rand.y, globals.time));
    output_color = mix(output_color, output_color + vec4(vec3(random(uv_rand)), 0.0), 0.005);


#ifdef TONEMAP_IN_SHADER
        output_color = tone_mapping(output_color);
#endif
#ifdef DEBAND_DITHER
    var output_rgb = output_color.rgb;
    output_rgb = powsafe(output_rgb, 1.0 / 2.2);
    output_rgb = output_rgb + screen_space_dither(in.frag_coord.xy);
    // This conversion back to linear space is required because our output texture format is
    // SRGB; the GPU will assume our output is linear and will apply an SRGB conversion.
    output_rgb = powsafe(output_rgb, 2.2);
    output_color = vec4(output_rgb, output_color.a);
#endif
#ifdef PREMULTIPLY_ALPHA
        output_color = premultiply_alpha(material.flags, output_color);
#endif


    return output_color;
}