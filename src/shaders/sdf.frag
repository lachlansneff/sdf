#version 450

layout(location = 0) out vec4 out_color;

layout(set = 0, binding = 0) uniform Uniforms {
    mat4 matrix;
    vec3 eye;
    vec2 resolution;
    float z_depth;

    vec3 light_pos;
} uniforms;

float sdf_sphere(vec3 p) {
    return length(p) - 1.0;
}

float sdf_box(vec3 p, vec3 b ) {
    vec3 q = abs(p) - b;
    return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
}

float sdf_cylinder(vec3 p, float h, float r)
{
  vec2 d = abs(vec2(length(p.xz),p.y)) - vec2(r,h);
  return min(max(d.x,d.y),0.0) + length(max(d,0.0));
}

float sdf_gyroid(vec3 p, float scale, float thickness) {
    p *= scale;
    return (abs(dot(sin(p), cos(p.zxy))) / scale - thickness) * 0.7;
}

float sdf_union(float d1, float d2) {
    return min(d1, d2);
}

float intersect(float d0, float d1) {
    return max(d0, d1);
}

float subtract(float d0, float d1) {
    return max(-d0, d1);
}

float sdf_smooth_union( float d1, float d2, float k ) {
    float h = clamp( 0.5 + 0.5*(d2-d1)/k, 0.0, 1.0 );
    return mix( d2, d1, h ) - k*h*(1.0-h);
}

float sdf(vec3 p) {
    float s = sdf_sphere(p + vec3(1.0, 0.0, 0.0));
    // float s2 = sdBox(p + vec3(-2.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0));
    // return opSmoothUnion(s, s2, 0.5);
    float gyroid = sdf_gyroid(p, 10.0, 0.02);
    float b = sdf_box(p + vec3(-1.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0));
    float i = intersect(gyroid, b);

    float u = sdf_smooth_union(s, i, 0.7);

    float c = sdf_cylinder(p + vec3(-2.0, 0.0, 0.0), 1.1, 1.0);

    return subtract(c, u);
}

struct Intersection {
    float dist;
    uint steps;
};

const uint MAX_STEPS = 64;

Intersection sphere_trace(vec3 origin, vec3 ray_dir) {
    const float EPSILON = 0.001;

    float t = 0.0;

    for (uint i = 0; i < MAX_STEPS; ++i) {
        float res = sdf(origin + ray_dir * t);

        if (res < (EPSILON * t)) {
            return Intersection(t, i);
        }

        t += res;
    }

    return Intersection(-1.0, MAX_STEPS);
}

// // fn estimate_normal(p: vec3<f32>) -> vec3<f32> {
// //     const k: vec2<f32> = vec2<f32>(1.0, -1.0);
// //     return normalize(
// //         k.xyy * sdf(p + k.xyy * EPSILON) +
// //         k.yyx * sdf(p + k.yyx * EPSILON) +
// //         k.yxy * sdf(p + k.yxy * EPSILON) +
// //         k.xxx * sdf(p + k.xxx * EPSILON)
// //     );
// //     // return normalize(vec3<f32>(
// //     //     sdf(vec3<f32>(p.x + EPSILON, p.y, p.z)) - sdf(vec3<f32>(p.x - EPSILON, p.y, p.z)),
// //     //     sdf(vec3<f32>(p.x, p.y + EPSILON, p.z)) - sdf(vec3<f32>(p.x, p.y - EPSILON, p.z)),
// //     //     sdf(vec3<f32>(p.x, p.y, p.z + EPSILON)) - sdf(vec3<f32>(p.x, p.y, p.z - EPSILON)),
// //     // ));
// // }

vec3 estimate_normal(vec3 p) {
    const float EPSILON = 0.001;
    const vec2 k = vec2(1.0, -1.0);
    return normalize(
        k.xyy * sdf(p + k.xyy * EPSILON) +
        k.yyx * sdf(p + k.yyx * EPSILON) +
        k.yxy * sdf(p + k.yxy * EPSILON) +
        k.xxx * sdf(p + k.xxx * EPSILON)
    );
}

vec3 ray_direction() {
    vec2 xy =  gl_FragCoord.xy - (uniforms.resolution / 2.0);
    return (uniforms.matrix * vec4(normalize(vec3(xy, -uniforms.z_depth)), 0.0)).xyz;
}

// float ao(vec3 pos, vec3 nor) {
// 	float occ = 0.0;
//     float sca = 1.0;
//     for( int i=0; i<5; i++ )
//     {
//         float hr = 0.01 + 0.12 * float( i ) / 4.0;
//         vec3 aopos =  nor * hr + pos;
//         float dd = sdf( aopos ).x;
//         occ += -(dd-hr)*sca;
//         sca *= 0.95;
//     }
//     return clamp( 1.0 - 3.0*occ, 0.0, 1.0 );    
// }

// float hard_shadow( in vec3 ro, in vec3 rd, float mint, float maxt )
// {
//     for( float t=mint; t<maxt; )
//     {
//         float h = sdf(ro + rd*t);
//         if( h<0.001 )
//             return 0.0;
//         t += h;
//     }
//     return 1.0;
// }

// float hard_shadow(vec3 hit_pos, vec3 light_pos, vec3 normals) {
//     float shadow = 0.0;
//     vec3 ray_origin = hit_pos + normals * 0.01;
//     vec3 ray_dir = normalize(light_pos - hit_pos);
//     Intersection marched_ray = sphere_trace(ray_origin, ray_dir);
//     if (marched_ray.dist != -1.0) {
//         shadow = 1.0;
//     }
//     return shadow;
// }

void main(void) {
    vec3 ray_dir = ray_direction();
    Intersection marched_ray = sphere_trace(uniforms.eye, ray_dir);

    if (marched_ray.dist == -1.0) {
        // Didn't hit anything.
        out_color = vec4(0.0, 0.0, 0.0, 0.0);
        return;
    }

    // // If steps is greater than some number, just draw blue
    // if (marched_ray.steps > 25) {
    //     out_color = vec4(0.0, 0.0, 1.0, 0.0);
    //     return;
    // }

    vec3 color = vec3(171.0/255.0, 146.0/255.0, 103.0/255.0);

    float ao = 1 - float(marched_ray.steps) / (MAX_STEPS - 1);

    // vec3 color = vec3(171.0/255.0, 146.0/255.0, 103.0/255.0) * ao;

    vec3 hit_pos = uniforms.eye + ray_dir * marched_ray.dist;
    vec3 normals = estimate_normal(hit_pos);
    float dif = dot(normals, normalize(uniforms.eye - hit_pos)) * 0.5 - 0.5;
    color += dif;

    color *= ao;

    color = pow(color, vec3(0.4545)); // gamma correction
    // float shadow = hard_shadow(hit_pos, uniforms.light_pos, normals);

    // color = mix(color, color * 0.2, shadow);
    
    // if (marched_ray.steps > 5) {
        
    // }
    out_color = vec4(color, 1.0);
}
