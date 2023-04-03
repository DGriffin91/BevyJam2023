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