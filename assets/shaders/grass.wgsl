fn parallax_grass(tex: texture_2d<f32>, samp: sampler, uv: vec2<f32>, V: vec3<f32>, ofs: f32, count: u32, thresh: f32, fade: f32) -> vec2<f32> {
    var vuv = vec2(V.x, V.z);
    let scale = 2.0;
    let fcount = f32(count);

    let ofs = ofs / fcount;

    var depth = 0.0;

    var bmp = 0.0;
    var s0 = 0.0;
    for (var i = 0u; i < count; i+=1u) {
        let ofs_uv = uv + vuv * ofs * f32(i);
        let s1 = textureSampleBias(tex, samp, ofs_uv, -1.5).x;
        //let pick = s1 > 1.0 / fcount * f32(i);
        let pick = s1 > thresh + fade * f32(i) / fcount;
        bmp = select(bmp, s1, pick);
        depth = select(depth, f32(i) / fcount, pick);
        s0 = s1;
    }

    return vec2(bmp, depth);
}

fn grass(V: vec3<f32>, uv: vec2<f32>) -> vec3<f32> {
    var lf_var = textureSample(detail_texture, detail_sampler, uv * 0.07).x;
    var bmp = textureSampleBias(detail_texture, detail_sampler, uv, -1.0).x;
    let spacing = 0.030;//mix(0.030, 0.010 + lf_var * 0.1, 0.0);
    let p = parallax_grass(detail_texture, detail_sampler, uv, V, spacing, 8u, 0.13, 0.15);
    bmp = mix(bmp * 0.35, p.x * p.y, p.y);
    let col1 = bmp * vec3(0.1, 0.4, 0.1);
    let col2 = bmp * vec3(0.5, 0.3, 0.15) * 0.6;
    let col = mix(col2, col1, saturate(lf_var * lf_var * 10.0)) * 1.6;
    return col;
}