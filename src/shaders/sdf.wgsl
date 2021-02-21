[[builtin(vertex_index)]]
var<in> in_vertex_index: u32;
[[builtin(position)]]
var<out> out_pos: vec4<f32>;

[[stage(vertex)]]
fn vs_main() {
    var uv: vec2<f32> = vec2<f32>((u32(in_vertex_index) << 1) & 2, u32(in_vertex_index) & 2);
    out_pos = vec4<f32>(uv.x * 2.0 - 1.0, -uv.y * 2.0 + 1.0, 0.0, 1.0);
}

[[builtin(frag_coord)]]
var<in> in_frag_coord: vec4<f32>;

[[location(0)]]
var<out> out_color: vec4<f32>;

[[block]]
struct Uniforms {
    eye: vec3<f32>;
    resolution: vec2<f32>;
    z_depth: f32; // resolution.y / tan(fieldOfView / 2.0);
};

[[group(0), binding(0)]]
var<uniform> uniforms: Uniforms;

const MAX_STEPS: u32 = 100;
const EPSILON: f32 = 0.001;
const MIN_DIST: f32 = 0.0;
const MAX_DIST: f32 = 100.0;

fn sdf_sphere(p: vec3<f32>) -> f32 {
    return length(p) - 1.0;
}

fn sdf_union(d1: f32, d2: f32) -> f32 {
    return min(d1, d2);
}

fn sdf(p: vec3<f32>) -> f32 {
    var s: f32 = sdf_sphere(p);
    var s2: f32 = sdf_sphere(p + vec3<f32>(1.0, 0.0, 0.0));
    return sdf_union(s, s2);
}

struct MarchedRay {
    dist: f32;
    steps: u32;
};

fn ray_march(origin: vec3<f32>, ray_dir: vec3<f32>) -> MarchedRay {
    const max_steps: u32 = 64;

    var t: f32 = 0.0;

    for (var i: u32 = 0; i < MAX_STEPS; i = i + 1) {
        var res: f32 = sdf(origin + ray_dir * t);
        
        if (res < (EPSILON * t)) {
            return MarchedRay(t, i);
        }

        t = t + res;
    }

    return MarchedRay(-1.0, max_steps);
}

// fn estimate_normal(p: vec3<f32>) -> vec3<f32> {
//     const k: vec2<f32> = vec2<f32>(1.0, -1.0);
//     return normalize(
//         k.xyy * sdf(p + k.xyy * EPSILON) +
//         k.yyx * sdf(p + k.yyx * EPSILON) +
//         k.yxy * sdf(p + k.yxy * EPSILON) +
//         k.xxx * sdf(p + k.xxx * EPSILON)
//     );
//     // return normalize(vec3<f32>(
//     //     sdf(vec3<f32>(p.x + EPSILON, p.y, p.z)) - sdf(vec3<f32>(p.x - EPSILON, p.y, p.z)),
//     //     sdf(vec3<f32>(p.x, p.y + EPSILON, p.z)) - sdf(vec3<f32>(p.x, p.y - EPSILON, p.z)),
//     //     sdf(vec3<f32>(p.x, p.y, p.z + EPSILON)) - sdf(vec3<f32>(p.x, p.y, p.z - EPSILON)),
//     // ));
// }

fn ray_direction() -> vec3<f32> {
    var xy: vec2<f32> = in_frag_coord.xy - (uniforms.resolution / 2.0);
    return normalize(vec3<f32>(xy, -uniforms.z_depth));
}

[[stage(fragment)]]
fn fs_main() {
    var marched_ray: MarchedRay = ray_march(uniforms.eye, ray_direction(), MIN_DIST, MAX_DIST);

    if (marched_ray.dist == -1.0) {
        // Didn't hit anything.
        out_color = vec4<f32>(0.0, 0.0, 0.0, 0.0);
        return;
    }

    out_color = vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
