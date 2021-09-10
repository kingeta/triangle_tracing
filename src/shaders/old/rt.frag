#version 450 core

precision lowp float;

uniform isampler1D textureSampler;
uniform ivec4 viewport;
uniform vec3 origin;
uniform vec3 forward;
uniform vec3 up;
uniform uint frame;

out vec4 Colour;

#define INF 3.402823466e+38
#define PI 3.141592653
#define WORLD_SIZE 4
#define SAMPLES 8
#define MAX_BOUNCE 4


// Jenkins hash function, specialized for a uint key
uint hash(uint key) {
    uint h = 0;
    for (int i=0; i < 4; ++i) {
        h += (key >> (i * 8)) & 0xFF;
        h += h << 10;
        h ^= h >> 6;
    }
    h += h << 3;
    h ^= h >> 11;
    h += h << 15;
    return h;
}


// Returns a pseudorandom value between -1 and 1
float rand(inout uint seed) {
    // 32-bit LCG Multiplier from
    // "Computationally Easy, Spectrally Good Multipliers for
    //  Congruential Pseudorandom Number Generators" [Steele + Vigna]
    seed = 0xadb4a92du * seed + 1;

    // Low bits have less randomness [L’ECUYER '99], so we'll shift the high
    // bits into the mantissa position of an IEEE float32, then mask with
    // the bit-pattern for 2.0
    uint m = (seed >> 9) | 0x40000000u;

    float f = uintBitsToFloat(m);   // Range [2:4]
    return f - 3.0;                 // Range [-1:1]
}

// Returns a coordinate uniformly distributed on a sphere's surface
vec3 rand3_on_sphere(inout uint seed) {
    float t = PI * rand(seed);
    float z = rand(seed);
    float r = sqrt((z + 1) * (1 - z));
    float x = cos(t) * r;
    float y = sin(t) * r;
    return vec3(x, y, z);
}

// Returns a coordinate uniformly distributed in a circle of radius 1
vec2 rand2_in_circle(inout uint seed) {
    float t = PI * rand(seed);
    float r = sqrt((rand(seed) + 1) / 2);
    return r * vec2(cos(t), sin(t));
}

vec2 rand2_on_circle(inout uint seed) {
    float t = PI * rand(seed);
    return vec2(cos(t), sin(t));
}

float length_squared(vec3 v) {
    return dot(v, v);
}

struct Ray {
    vec3 o; // Origin
    vec3 d; // Direction
};

vec3 Ray_position(Ray r, float t) {
    return r.o + t * r.d;
}

struct Hit_Record {
    float dist;
    vec3 p;
    vec3 normal;
};

struct Mat {
    vec3 colour;
    uint type;
    // 0: diffuse
    // 1: mirror
    // 2: light
};

struct Sphere {
    vec3 centre;
    float radius;
    //vec3 colour;
    Mat mat;
};


bool Sphere_hit(in Ray r, in const Sphere sphere, in const float t_min, in const float t_max, out Hit_Record hit) {
    vec3 oc = r.o - sphere.centre;
    //float a = dot(r.d, r.d);
    float a = length_squared(r.d);
    float half_b = dot(oc, r.d);
    //float c = dot(oc, oc) - radius * radius;
    float c = length_squared(oc) - sphere.radius * sphere.radius;
    float disc = half_b * half_b - a * c;
    if (disc > 0.) {
        float disc_sqrt = sqrt(disc);
        float dist = (-half_b - disc_sqrt) / a;
        if (dist < t_min || dist > t_max) {
            disc = (-half_b + disc_sqrt) / a;
            if (disc < t_min || t_max > disc) {
                return false;
            }
        }

        vec3 p = Ray_position(r, dist);
        //hit = Hit_Record(dist, p, normalize(p - sphere.centre));
        hit.dist = dist;
        hit.p = p;
        hit.normal = normalize(p - sphere.centre);

        return true;
        //return (-half_b - sqrt(disc)) / a;
    } else {
        //return -1.;
        return false;
    }
}

Sphere scene[] = Sphere[](
    Sphere(
        vec3(0., -1000.5, -1.), 1000.,
        Mat(
            vec3(0.2, 0., 0.2),
            0
        )
    ),
    Sphere(
        vec3(1., 0., -1.), 0.5,
        Mat(
            vec3(0.8, 0.1, 0.1),
            0
        )
    ),
    Sphere(
        vec3(0., 0., -1.), 0.5,
        Mat(
            vec3(0.8),
            1
        )
    ),
    Sphere(
        vec3(-1., 0., -1.), 0.5,
        Mat(
            10. * vec3(0.1, 0.1, 0.8),
            2
        )
    )
);

const vec3 to_light = normalize(vec3(-1., 1., 1.));

bool World_hit(in Ray r, in const Sphere[WORLD_SIZE] world, in const float t_min, in const float t_max, out Hit_Record hit_record, out int hit_which) {
    bool hit_happened = false;
    float closest = t_max;
    Hit_Record temp_record;

    for ( int i = 0; i < WORLD_SIZE; i++ ) {
        if ( Sphere_hit(r, world[i], t_min, t_max, temp_record) && temp_record.dist < closest ) {
            hit_happened = true;
            closest = temp_record.dist;
            hit_record = temp_record;
            hit_which = i;
        }
    }

    return hit_happened;
}

bool trace(inout uint seed, inout Ray r, inout vec3 col) {
    Hit_Record hit_record;
    int hit_which;
    float t;

    if ( World_hit(r, scene, 0.001, INF, hit_record, hit_which) ) {
        //r.o = hit_record.p + EPS * hit_record.normal;
        r.o = hit_record.p;

        switch ( scene[hit_which].mat.type ) {
            case 0: // diffuse
                /*r.d = normalize(hit_record.normal + rand3_on_sphere(seed));
                col *= scene[hit_which].mat.colour * max(0., dot(r.d, hit_record.normal));
                return false;*/

                r.d = to_light;
                //col *= (scene[hit_which].mat.colour * max(0., dot(r.d, hit_record.normal))) + (vec3(1.) * pow(max(0., dot(r.d, hit_record.normal)), 100.));
                col *= (scene[hit_which].mat.colour * (0.03 + max(0., dot(r.d, hit_record.normal))))/1.03 + (vec3(1.) * pow(max(0., dot(r.d, hit_record.normal)), 100.));

                //r.o += rand3_on_sphere(seed)/100.;
                if ( World_hit(r, scene, 0.001, INF, hit_record, hit_which) ) {
                    // Obscured
                    col /= 10.;
                }

                return true;
            case 1: // mirror
                r.d = reflect(r.d, hit_record.normal);
                col *= scene[hit_which].mat.colour * max(0., dot(r.d, hit_record.normal));
                return false;
            case 2: // light
                col *= scene[hit_which].mat.colour;
                return true;
        }
    } else {
        // No hit
        t = 0.5 * (clamp(r.d.y, -1., 1.) + 1.);
        col *= t * vec3(.5, .7, 1.) + (1. - t) * vec3(1.) + vec3(.4) * max(0., dot(to_light, r.d));
        return true;
    }
}

vec3 bounce(inout uint seed, in Ray r) {
    vec3 col = vec3(1.);

    for ( int i = 0; i < MAX_BOUNCE; i++ ) {
        if ( trace(seed, r, col) ) {
            return col;
        }
    }

    return vec3(0.);
}

void main() {
    // Initial
    vec3 final_col = vec3(0.);
    ivec2 dimensions = viewport.zw;
    // Divided by height i.e uv.x \in [-r, r], uv.y \in [-1, 1], r = width/height (generally > 1)

    uint frame_seed = hash(frame);
    uint seed = hash(frame_seed ^ hash(uint(gl_FragCoord.x)*uint(dimensions.y) + uint(gl_FragCoord.y)));


    // Setting up scene variables
    float focal_length = 2.;

    vec2 uv = (2 * gl_FragCoord.xy - dimensions.xy) / dimensions.y;

    vec3 canvas_side = normalize(cross(up, forward));
    vec3 canvas_up = cross(forward, canvas_side);
    vec3 dir = vec3(uv, focal_length);

    
    for ( uint j = 0; j < SAMPLES; j++ ) {

        //uint frame_seed = hash(SAMPLES * frame + j);
        //uint seed = hash(frame_seed ^ hash(uint(gl_FragCoord.x)*uint(dimensions.y) + uint(gl_FragCoord.y)));

        //uv = (2 * gl_FragCoord.xy - dimensions.xy + vec2(rand(seed), rand(seed))) / dimensions.y;
        //dir = vec3(uv + vec2(rand(seed), rand(seed))/float(dimensions.y), focal_length);
        dir = vec3(uv + 2.*rand2_in_circle(seed)/float(dimensions.y), focal_length);

        Ray r = Ray(origin, normalize(-dir.x * canvas_side + dir.y * canvas_up + dir.z * forward));
        final_col += bounce(seed, r);
    }

    //final_col = vec3(rand(seed));

    //Colour = vec4(pow(final_col / float(SAMPLES), vec3(1./2.2)), 1.);
    final_col /= float(SAMPLES);
    final_col = vec3(1.) - exp(-final_col);
    Colour = vec4(pow(final_col, vec3(1./2.2)), 1.);
    //Colour = vec4(vec3(float(texelFetch(textureSampler, 1, 0)))/255., 1.);
}