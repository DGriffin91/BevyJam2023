fn apply_fog_c(input_color: vec4<f32>, fragment_world_position: vec3<f32>, view_world_position: vec3<f32>, height_fog: f32) -> vec4<f32> {
    let view_to_world = fragment_world_position.xyz - view_world_position.xyz;

    // `length()` is used here instead of just `view_to_world.z` since that produces more
    // high quality results, especially for denser/smaller fogs. we get a "curved"
    // fog shape that remains consistent with camera rotation, instead of a "linear"
    // fog shape that looks a bit fake
    let distance = length(view_to_world);

    var scattering = vec3<f32>(0.0);
    //if fog.directional_light_color.a > 0.0 {
    //    let view_to_world_normalized = view_to_world / distance;
    //    let n_directional_lights = lights.n_directional_lights;
    //    for (var i: u32 = 0u; i < n_directional_lights; i = i + 1u) {
    //        let light = lights.directional_lights[i];
    //        scattering += pow(
    //            max(
    //                dot(view_to_world_normalized, light.direction_to_light),
    //                0.0
    //            ),
    //            fog.directional_light_exponent
    //        ) * light.color.rgb;
    //    }
    //}

    // height fog (make 500.0/ larger for more gradual)
    let exp_height_fog = exponential_fog(input_color, 400.0/max(fragment_world_position.y + 1.1, 0.9), vec3(0.0));
    var input_color = mix(input_color, exp_height_fog, 0.4 * height_fog); 

    if fog.mode == FOG_MODE_LINEAR {
        return linear_fog(input_color, distance, scattering);
    } else if fog.mode == FOG_MODE_EXPONENTIAL {
        return exponential_fog(input_color, distance, scattering);
    } else if fog.mode == FOG_MODE_EXPONENTIAL_SQUARED {
        return exponential_squared_fog(input_color, distance, scattering);
    } else if fog.mode == FOG_MODE_ATMOSPHERIC {
        return atmospheric_fog(input_color, distance, scattering);
    } else {
        return input_color;
    }
}