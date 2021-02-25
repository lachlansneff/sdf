// Self-Optimizing Sphere Marching
// 

[[builtin(global_invocation_id)]]
var global_id: vec3<u32>;
[[builtin(workgroup_id)]]
var workgroup_id: vec3<u32>;

[[group(0), binding(0)]]
var<storage> bytecode: [[access(read)]] array<vec4<u32>>;

fn compute_sdf(p: vec3<f32>) -> f32 {
    var regs: array<f32, 2> = array<f32; 2>();
    var idx: u32 = 0;

    loop {
        var inst: vec4<u32> = bytecode[idx];
        var op: u32 = inst[0] && 0x3fffffff; // mask off the register part
        
        // Since this will have a *lot* of cases, I think this will
        // only really work if it gets turned into a jump table.
        switch (op) {
            // combinations
            case 0: {
                break; // break out of the loop
            }
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
                // TODO: retrieve transformation data
                regs[inst[0] >> 30] = sdf_sphere(p, r);
            }

            default: {
                break;
            }
        }

        idx = idx + 1;
    }
}

[[stage(compute), workgroup_size(32, 1, 1)]]
fn main() {
    var p: vec3<f32> = vec3(0.0, 0.0, 10.0);

    var dist: f32 = compute_sdf(p);
}

fn sdf_sphere(p: vec3<f32>, r: f32) -> f32 {
    p.mag() - r
}
