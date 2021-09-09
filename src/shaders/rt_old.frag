#version 450 core

precision lowp float;

//uniform ivec2 viewportDimensions;
uniform ivec4 viewport;
uniform vec3 eye;
uniform vec3 direction;
uniform vec2 mouse;
uniform float angle;

out vec4 Colour;

const float big = 1.e10;

#define PI 3.14159

#define MAX 4
#define SAMPLES 8


struct Sphere {
    vec3 centre;
    float radius;
    vec3 colour;
};

float Sphere_intersect(in vec3 _origin, in vec3 _direction, in Sphere _sphere) {
    // Returns the intersection distance (not point (!)) of a vector line and sphere
    float b = 2. * dot(_direction, _origin - _sphere.centre);
    float c = dot(_sphere.centre, _sphere.centre) + dot(_origin, _origin) - 2. * dot(_sphere.centre, _origin) - _sphere.radius * _sphere.radius;
    float delta = b * b - 4. * c;
    
    float sq = sqrt(max(delta, 0.));

    float h0 = (-b-sq)/2.;
    float h1 = (-b+sq)/2.;

    float h = h0 >= 0. ? h0 : h1;

    return (delta >= 0.) && (h >= 0.) ? h : big;
}

vec3 Sphere_normal(in vec3 _position, in Sphere _sphere) {
    return (_position - _sphere.centre) / _sphere.radius;
}

// Randomising function
vec3 hash3(in float n) {
    return fract(sin(vec3(n,n+1.0,n+2.0))*vec3(43758.5453123,22578.1459123,19642.3490423));
}

// BRDFs vaguely
vec3 ray_random(in vec3 _normal, in float u1, in float u2) {
    float r = sqrt(1. - u1*u1);
    float phi = 2. * PI * u2;
    
    vec3 rand = vec3(r * cos(phi), r * sin(phi), u1);
    
    return normalize(_normal + rand);
}

vec3 ray_reflect(in vec3 _normal, in vec3 _direction) {
    return reflect(_direction, _normal);
}


bool intersect_world(in vec3 _origin, in vec3 _direction, in Sphere[2] _scene, out vec3 _intersection, out Sphere _which) {
    // Intersects a ray with the world and returns whether there's a hit
	
    // Ignore things which are too far away
    float t = big;
    float dist = 0.;
    
    // Intersect everything in the scene description
    for (int i = 0; i < _scene.length(); i++) {
        dist = Sphere_intersect(_origin, _direction, _scene[i]);
        
        // Update the current closest hit and which was hit
        if (dist < t) {
            _which = _scene[i];
            t = dist;
        }
        
    }
	
    // Calculate where the actual intersection was (sometimes unneeded)
    _intersection = _origin + _direction * t;
    return (t >= 0.) && (t < 1000.);
}


vec3 trace(in vec3 _origin, in vec3 _direction, in Sphere[2] _scene, in int j) {
    vec3 result = vec3(1.);

    const vec3 sky = vec3(.2);
    
    vec3 to_light = normalize(vec3(-1., 1., -1.));

    Sphere which;
    vec3 normal;
    bool hit;
    vec3 temp;

    for (int i = 0; i < MAX; i++) {
        hit = intersect_world(_origin, _direction, _scene, _origin, which);

        if (hit) {
            /*normal = Sphere_normal(_origin, which);

            float lambert = max(0., dot(normal, to_light));
            result += lambert * which.colour;
            result += specular * pow(max(0., dot(reflect(to_light, normal), _direction)), 100.) * step(0., lambert);
            result += ambient;

            return result;*/

            normal = Sphere_normal(_origin, which);
            temp = normalize(.5 - hash3(float(i+j) + 100.));
            _direction = temp * sign(dot(normal, temp));
            //_direction = ray_reflect(normal, _direction);

            result *= which.colour * dot(_direction, normal) / PI;
            _origin += 0.001 * normal;
        } else {
            //return sky + vec3(0.1) * max(0., dot(_direction, -to_light));
            return result * sky;
        }

    }

}


void main() {
    vec3 col = vec3(0.);
    ivec2 viewportDimensions = viewport.zw;
    //float viewportRatio = viewportDimensions.x/viewportDimensions.y;

    // Getting where the pixel is on the screen
    vec2 uv = (2 * gl_FragCoord.xy - viewportDimensions.xy) / viewportDimensions.y;
	
    const vec3 up = vec3(sin(angle), cos(angle), 0.);

    //vec3 direction = vec3(0., 0., 1.);
	
    vec3 canvas_side = normalize(cross(up, direction));
    vec3 canvas_up = cross(direction, canvas_side);

    //direction = normalize(direction + uv.x * .6 * canvas_side + uv.y * .6 * canvas_up + vec3(sin(mouse)/4., 0.));
    vec3 direction2 = normalize(direction + uv.x * .5 * canvas_side + uv.y * .5 * canvas_up);
    
    Sphere middle = Sphere(vec3(0., 0., 5.), 1., vec3(.8, .6, .2));
    Sphere ground = Sphere(vec3(0., -101., 5.), 100., vec3(0.3608, 0.1373, 0.4667));
	Sphere scene[] = Sphere[](middle, ground);

    for (int j = 0; j < SAMPLES; j++) {
        col += trace(eye, normalize(direction2 + 0.01 * hash3(float(j))), scene, j);
    }

    col /= 16.;
    //col = trace(eye, direction2, scene, 0);

    //Colour = vec4(pow(col, vec3(1./2.2)), 1.0);
    Colour = vec4(pow(vec3(1.)-exp(-col * 20.), vec3(.45)), 1.0);
}