// Self-Optimizing Sphere Marching
// 

[[builtin(global_invocation_id)]]
var global_id: vec3<u32>;
[[builtin(workgroup_id)]]
var workgroup_id: vec3<u32>;

[[group(0), binding(0)]]
var<storage> bytecode: [[access(read)]] array<vec4<u32>>;

fn sdf_sphere(p: vec3<f32>, r: f32) -> f32 {
    p.mag() - r
}

fn compute_sdf(p: vec3<f32>) -> f32 {
    var regs: array<f32, 2> = array<f32; 2>();
    var idx: u32 = 0;

    loop {
        var inst: vec4<u32> = bytecode[idx];
        
        // Since this will have a *lot* of cases, I think this will
        // only really work if it gets turned into a jump table.
        switch (inst[0] & 0x3fffffff) { // mask off the register part
            case 0: {
                return regs[inst[0] >> 30];
            }

            // combinations
            case 1: { // union
                regs[inst[0] >> 30] = min(regs[0], regs[1]);
            }
            case 2: { // intersection
                regs[inst[0] >> 30] = max(regs[0], regs[1]);
            }
            case 3: { // subtraction
                regs[inst[0] >> 30] = max(-regs[0], regs[1]);
            }
            case 4: { // smooth union
                
            }
            case 5: { // smooth intersection
                
            }
            case 6: { // smooth subtraction
                
            }

            // shapes
            case 7: { // sphere
                var r: f32 = bitcast<f32>(inst[2]);
                // TODO: retrieve transformation data from `inst[1]`
                regs[inst[0] >> 30] = sdf_sphere(p, r);
            }

            default: {
                return 0.0;
            }
        }

        idx = idx + 1;
    }

    return 0.0;
}

struct Intersection {
    dist: f32;
    steps: u32;
};

const MAX_STEPS: u32 = 64;

fn sphere_march(rayorigin: vec3<f32>, raydir: vec3<f32>) -> Intersection {
    const EPSILON: f32 = 0.001;

    var t: f32 = 0.0;

    for (var i: u32 = 0; i < MAX_STEPS; i = i + 1) {
        var dist: f32 = compute_sdf(rayorigin + raydir * t);

        if (res < (EPSILON * t)) {
            return Intersection(dist, i);
        }

        t = t + res;
    }

    return Intersection(-1.0, MAX_STEPS);
}

[[stage(compute), workgroup_size(32, 1, 1)]]
fn main() {
    var p: vec3<f32> = vec3(0.0, 0.0, 10.0);

    var dist: f32 = compute_sdf(p);
}

