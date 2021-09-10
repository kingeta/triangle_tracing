#version 430 core
//#version 450 core


uniform isampler1D textureSampler;
uniform ivec4 viewport;
uniform vec3 origin;
uniform vec3 forward;
uniform vec3 up;
uniform uint frame;
uniform float focus_dist;

out vec4 Colour;

//#define INF 3.402823466e+38
#define INF 1.e+10
#define CLOSE 0.001
#define PI 3.141592653
#define WORLD_SIZE 4
#define SAMPLES 4
#define MAX_BOUNCE 5
#define DOF_RADIUS 0.2


#define DIFFUSE 0
#define MIRROR 1
#define LIGHT 2
#define GLASS 3

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
    float r = sqrt((z + 1.) * (1. - z));
    float x = cos(t) * r;
    float y = sin(t) * r;
    return vec3(x, y, z);
}

// Returns a coordinate uniformly distributed in a circle of radius 1
vec2 rand2_in_circle(inout uint seed) {
    float t = PI * rand(seed);
    float r = sqrt((rand(seed) + 1.) / 2.);
    return r * vec2(cos(t), sin(t));
}

vec2 rand2_on_circle(inout uint seed) {
    float t = PI * rand(seed);
    return vec2(cos(t), sin(t));
}

/// Cosine weighted sampling on the unit hemisphere upwards
vec3 rand3_cos(inout uint seed) {
    float u1 = abs(rand(seed));
    float theta = 2. * PI * abs(rand(seed));

    float r = sqrt(u1);

    return vec3(r * cos(theta), r * sin(theta), sqrt(max(0., 1. - u1)));
}

/// Generate, from one vector, an orthonormal basis
/// without too many divisions by zero etc.
/// Taken from Duff etc paper
mat3 onb(vec3 u) {
    //let sign = (1_f32).copysign(u.z);
    float sign_ = sign(u.z);
    float a = -1. / (sign_ + u.z);
    float b = u.x * u.y * a;

    return mat3(vec3(1. + sign_ * u.x * u.x * a, sign_ * b, -sign_ * u.x), vec3(b, sign_ + u.y * u.y * a, -u.y), u);
}

/// Cosine weighted sampling on the hemisphere defined by u.
vec3 rand3_hemisphere_cos(vec3 u, inout uint seed) {
    return onb(u) * rand3_cos(seed);
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
    
    if (disc > 0.001) {
        float disc_sqrt = sqrt(disc);
        float dist = (-half_b - disc_sqrt) / a;
        //if (dist < t_min || dist > t_max) {
        if (t_min > dist || dist > t_max) {
            //return false;
            dist = (-half_b + disc_sqrt) / a;
            //if (dist < t_min || t_max < dist) {
            if (t_min > dist || dist > t_max) {
                return false;
            }
        }

        vec3 p = Ray_position(r, dist);
        //hit = Hit_Record(dist, p, normalize(p - sphere.centre));
        hit.dist = dist;
        hit.p = p;
        hit.normal = normalize(p - sphere.centre);
        //hit.normal *= -sign(dot(hit.normal, r.d));

        return true;

    } else {
        return false;
    }
}


const Sphere scene[] = Sphere[](
    Sphere( // Left
        vec3(-2., 0., 0.), 1.,
        Mat(
            vec3(1.),
            GLASS
        )
    ),
    Sphere( // Middle
        vec3(0.), 1.,
        Mat(
            0.9 * vec3(1., 1., 0.),
            DIFFUSE
        )
    ),
    Sphere( // Right
        vec3(2., -0.2, 0.), 0.8,
        Mat(
            0.95 * vec3(0.8, 0.4, 0.4),
            MIRROR
        )
    ),
    Sphere( // Floor
        vec3(0., -1e+3-1., 0.), 1e+3,
        Mat(
            0.9 * vec3(0.3, 0.25, 0.25),
            DIFFUSE
        )
    )
);

const Sphere scene2[] = Sphere[](
    Sphere(
        vec3(0., -10000.5, -1.), 10000.,
        Mat(
            vec3(0.2, 0., 0.2),
            0
        )
    ),
    /*Sphere(
        vec3(0., 0., -10010.), 10000.,
        Mat(
            vec3(0.2, 0., 0.2),
            0
        )
    ),*/
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
        vec3(-1., 1., -1.), 0.5,
        Mat(
            10. * vec3(0.1, 0.1, 0.8),
            2
        )
    )
);

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


const vec3 SUN_DIRECTION = vec3(0.577350, 0.577350, 0.577350);
const vec3 SKY_COLOUR = vec3(0.45, 0.68, 0.87);


/// Schlick approximation or something
float schlick(float cos_, float n_dielectric) {
    float r0 = pow((1.-n_dielectric)/(1.+n_dielectric), 2.);
    //return r0 + (1. - r0) * (1. - cos).powf(5.)
    return r0 + (1. - r0) * pow(1. - cos_, 5.);
}



bool trace(inout uint seed, inout Ray r, inout vec3 col) {
    Hit_Record hit_record;
    int hit_which;

    if ( World_hit(r, scene, CLOSE, INF, hit_record, hit_which) ) {
        //r.o = hit_record.p + EPS * hit_record.normal;
        r.o = hit_record.p;
        //col = hit_record.normal;
        //return true;
        
        switch ( scene[hit_which].mat.type ) {
            case 0: // diffuse
                //r.d = normalize(hit_record.normal + rand3_on_sphere(seed));
                r.d = rand3_hemisphere_cos(hit_record.normal, seed);
                //col *= scene[hit_which].mat.colour * max(0., dot(r.d, hit_record.normal));
                col *= scene[hit_which].mat.colour;
                return false;
            case 1: // mirror
                r.d = reflect(r.d, hit_record.normal);
                // col *= scene[hit_which].mat.colour * max(0., dot(r.d, hit_record.normal)); // with cos
                col *= scene[hit_which].mat.colour; // without cos
                return false;
            case 2: // light
                col *= scene[hit_which].mat.colour;
                return true;
            case 3: // glass with n = 1.54
                const float refr = 1.54;
                
                
                float cos_ = dot(hit_record.normal, r.d);
                vec3 norm = -hit_record.normal * sign(cos_);
                
                float ratio;
                if (cos_ < 0.) {
                    ratio = 1./refr;
                } else {
                    ratio = refr;
                }
                
                float discriminant = 1. - ratio*ratio * (1.- cos_*cos_);
                
                //r.d = refract(r.d, -sign(dot(hit_record.normal, r.d)) * hit_record.normal, 1.54);
                if ( discriminant > 0. && (abs(rand(seed)) > schlick(abs(cos_), refr)) ) {
                    r.d = refract(r.d, norm, ratio);
                } else {
                    r.d = reflect(r.d, norm);
                }
                
                col *= scene[hit_which].mat.colour;
                return false;

        }
    } else {
        // No hit
        //float t = 0.5 * (clamp(r.d.y, -1., 1.) + 1.);
        //col *= t * vec3(.5, .7, 1.) + (1. - t) * vec3(1.) + vec3(.2) * max(0., dot(vec3(1., 0., 1.), r.d));
        
        vec3 sun = vec3(pow(clamp(dot(SUN_DIRECTION, r.d) + 0.03, 0., 1.), 100.));
        float lerp = pow(0.5 + r.d.y/2., 1.5);
        vec3 sky = (1. - lerp) * SKY_COLOUR + vec3(lerp);
        
        col *= sun + 0.4 * sky;
        
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
    // (Divided by height i.e uv.x \in [-r, r], uv.y \in [-1, 1], r = width/height (generally > 1))

    //uint frame_seed = hash(frame);
    //uint seed = hash(frame_seed ^ hash(uint(gl_FragCoord.x)*uint(dimensions.y) + uint(gl_FragCoord.y)));
    uint frame_seed;
    uint seed;

    // Setting up scene variables
    float focal_length = 1.;

    //vec2 uv = (2 * gl_FragCoord.xy - dimensions.xy) / dimensions.y;
    vec2 uv;
    
    vec3 canvas_side = normalize(cross(up, forward));
    vec3 canvas_up = cross(forward, canvas_side);
    //vec3 dir = vec3(uv, focal_length);
    vec3 dir;
    

    /*Hit_Record temp;
    int _;
    bool test = World_hit(Ray(origin, forward), scene, CLOSE, INF, temp, _);
    float focus_dist;

    if ( test ) {
        focus_dist = temp.dist;
        focus_dist = sqrt(length_squared(scene[_].centre - origin));
    } else {
        focus_dist = 1e3;
    }*/


    frame_seed = hash(frame);
    seed = hash(frame_seed ^ hash(uint(gl_FragCoord.x)*uint(dimensions.y) + uint(gl_FragCoord.y)));


    for ( uint j = 0; j < SAMPLES; j++ ) {

        //frame_seed = hash(SAMPLES * frame + j);
        //seed = hash(frame_seed ^ hash(uint(gl_FragCoord.x)*uint(dimensions.y) + uint(gl_FragCoord.y)));

        float angle = rand(seed) * PI;
        vec3 random_position = DOF_RADIUS * sqrt(abs(rand(seed))) * (sin(angle) * canvas_side + cos(angle) * canvas_up);

        //vec3 random_origin = random_position + origin;
        
        uv = ( 2 * (gl_FragCoord.xy +  abs(vec2(rand(seed), rand(seed))) ) - dimensions.xy) / dimensions.y;
        dir = vec3(uv, focal_length);
        dir = normalize(-dir.x * canvas_side + dir.y * canvas_up + dir.z * forward);

        Ray r = Ray(origin + random_position, normalize(-random_position + focus_dist * dir));
        final_col += bounce(seed, r);
    }

    //final_col = vec3(rand(seed));

    //Colour = vec4(pow(final_col / float(SAMPLES), vec3(1./2.2)), 1.);
    final_col /= float(SAMPLES);
    final_col = vec3(1.) - exp(2. * -final_col);
    
    if ( abs(2 * gl_FragCoord.x - dimensions.x) < 6 && abs(2 * gl_FragCoord.y - dimensions.y) < 6) {
        final_col = vec3(1.) - final_col;
    }

    //final_col = vec3(focus_dist)/10.;
    Colour = vec4(pow(final_col, vec3(1./2.2)), 1.);
    
    //Colour = vec4(final_col, 1.);
}
