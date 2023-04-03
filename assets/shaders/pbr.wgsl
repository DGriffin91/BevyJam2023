#import "shaders/bicubic.wgsl"

#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::pbr_bindings
#import bevy_pbr::mesh_bindings

#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::pbr_ambient
//#import bevy_pbr::shadows
#import "shaders/shadows.wgsl"
#import bevy_pbr::fog
#import bevy_pbr::pbr_functions

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

// https://simonharris.co/making-a-noise-film-grain-post-processing-effect-from-scratch-in-threejs/
fn random(p: vec2<f32>) -> f32 {
    let K1 = vec2(
        23.14069263277926, // e^pi (Gelfond's constant)
        2.665144142690225 // 2^sqrt(2) (Gelfond–Schneider constant)
    );
    return fract( cos( dot(p, K1) ) * 12345.6789 );
}

fn uhash(a: u32, b: u32) -> u32 { 
    var x = ((a * 1597334673u) ^ (b * 3812015801u));
    // from https://nullprogram.com/blog/2018/07/31/
    x = x ^ (x >> 16u);
    x = x * 0x7feb352du;
    x = x ^ (x >> 15u);
    x = x * 0x846ca68bu;
    x = x ^ (x >> 16u);
    return x;
}

fn unormf(n: u32) -> f32 { 
    return f32(n) * (1.0 / f32(0xffffffffu)); 
}

fn hash_noise(ifrag_coord: vec2<i32>, frame: u32) -> f32 {
    let urnd = uhash(u32(ifrag_coord.x), (u32(ifrag_coord.y) << 11u) + frame);
    return unormf(urnd);
}

// replace this by something better
fn hash(p: vec3<f32>) -> f32 {
    var p = fract(p * 0.3183099 + .1);
	p *= 17.0;
    return fract(p.x * p.y * p.z * (p.x + p.y + p.z));
}

fn noise(x: vec3<f32>) -> f32 {
    let i = vec3(floor(x));
    var f = fract(x);
    f = f * f * (3.0 - 2.0 * f);
	
    return mix(mix(mix( hash(i+vec3(0.0,0.0,0.0)), 
                        hash(i+vec3(1.0,0.0,0.0)),f.x),
                   mix( hash(i+vec3(0.0,1.0,0.0)), 
                        hash(i+vec3(1.0,1.0,0.0)),f.x),f.y),
               mix(mix( hash(i+vec3(0.0,0.0,1.0)), 
                        hash(i+vec3(1.0,0.0,1.0)),f.x),
                   mix( hash(i+vec3(0.0,1.0,1.0)), 
                        hash(i+vec3(1.0,1.0,1.0)),f.x),f.y),f.z);
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var output_color: vec4<f32> = material.base_color;
#ifdef VERTEX_COLORS
    output_color = output_color * in.color;
#endif
#ifdef VERTEX_UVS
    if ((material.flags & STANDARD_MATERIAL_FLAGS_BASE_COLOR_TEXTURE_BIT) != 0u) {
        output_color = output_color * textureSample(base_color_texture, base_color_sampler, in.uv);
    }
#endif

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
            emissive = vec4<f32>(emissive.rgb * emit_image, 1.0);
        }
#endif
        pbr_input.material.emissive = emissive;

        var metallic: f32 = material.metallic;
        var perceptual_roughness: f32 = material.perceptual_roughness;
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
#ifdef VERTEX_UVS
        if ((material.flags & STANDARD_MATERIAL_FLAGS_OCCLUSION_TEXTURE_BIT) != 0u) {
            occlusion = textureSample(occlusion_texture, occlusion_sampler, in.uv).r;
        }
#endif
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

        output_color = pbr(pbr_input);


        // ---------------- noise
        var uv_rand = in.frag_coord.xy / vec2<f32>(view.viewport.zw);
        uv_rand.y *= random(vec2(uv_rand.y, globals.time));
        output_color = mix(output_color, output_color + vec4(vec3(random(uv_rand)), 1.0), 0.008);

        let noise_size = 1.5;
        var noise = noise(in.world_position.xyz * 512.0 * noise_size);
        noise += noise(in.world_position.xyz * 256.0 * noise_size);
        noise += noise(in.world_position.xyz * 16.0 * noise_size) * 0.2;
        noise += noise(in.world_position.xyz * 6.0 * noise_size) * 0.3;
        output_color = mix(output_color, output_color * vec4(vec3(noise), 1.0), 0.18);
        // ---------------- noise

    } else {
        output_color = alpha_discard(material, output_color);
    }

    // fog
    if (fog.mode != FOG_MODE_OFF && (material.flags & STANDARD_MATERIAL_FLAGS_FOG_ENABLED_BIT) != 0u) {
        output_color = apply_fog(output_color, in.world_position.xyz, view.world_position.xyz);
    }




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