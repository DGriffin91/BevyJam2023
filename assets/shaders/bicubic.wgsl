fn cubic(v: f32) -> vec4<f32> {
    let n = vec4(1.0, 2.0, 3.0, 4.0) - v;
    let s = n * n * n;
    let x = s.x;
    let y = s.y - 4.0 * s.x;
    let z = s.z - 4.0 * s.y + 6.0 * s.x;
    let w = 6.0 - x - y - z;
    return vec4(x, y, z, w) * (1.0/6.0);
}

fn textureSampleBicubic(tex: texture_2d<f32>, tex_sampler: sampler, texCoords: vec2<f32>) -> vec4<f32> {
    let texture_size = vec2<f32>(textureDimensions(tex).xy);

    let invTexSize = 1.0 / texture_size;
   
    var texCoords = texCoords * texture_size - 0.5;

    let fxy = fract(texCoords);
    texCoords = texCoords - fxy;

    let xcubic = cubic(fxy.x);
    let ycubic = cubic(fxy.y);

    let c = texCoords.xxyy + vec2(-0.5, 1.5).xyxy;
    
    let s = vec4(xcubic.xz + xcubic.yw, ycubic.xz + ycubic.yw);
    var offset = c + vec4(xcubic.yw, ycubic.yw) / s;
    
    offset = offset * invTexSize.xxyy;
    
    let sample0 = textureSample(tex, tex_sampler, offset.xz);
    let sample1 = textureSample(tex, tex_sampler, offset.yz);
    let sample2 = textureSample(tex, tex_sampler, offset.xw);
    let sample3 = textureSample(tex, tex_sampler, offset.yw);

    let sx = s.x / (s.x + s.y);
    let sy = s.z / (s.z + s.w);

    return mix(mix(sample3, sample2, vec4(sx)), mix(sample1, sample0, vec4(sx)), vec4(sy));
}