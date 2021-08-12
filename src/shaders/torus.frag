#version 450 core

//uniform ivec2 viewportDimensions;
uniform ivec4 viewport;
uniform vec3 eye_position;

out vec4 Color;


precision mediump float;


float sphereSDF(vec3 position, vec3 centre, float radius) {
    // Signed distance function of a sphere
    return distance(position, centre) - radius;
}

vec3 sphereNorm(vec3 position, vec3 centre, float radius) {
    // Sphere normal
    return (position - centre) / radius;
}

float torusSDF( vec3 p, vec2 t ) {
    // Torus SDF
  vec2 q = vec2(length(p.xz)-t.x,p.y);
  return length(q)-t.y;
}

vec3 torusNorm(vec3 p, vec2 t) {
    // This is just a gradient approximation, used with the torus SDF
    vec2 ts = vec2(1, 0) * 0.01;
    
    float res = torusSDF(p, t);
    
    return normalize(vec3(torusSDF(p + ts.xyy, t) - res, \
                         torusSDF(p + ts.yxy, t) - res, \
                         torusSDF(p + ts.yyx, t) - res));
}


vec3 Norm(vec3 position, vec3 centre, float radius) {
    vec2 t = vec2(1, 0) * 0.01;
    
    float res = sphereSDF(position, centre, radius);
    
    return normalize(vec3(sphereSDF(t.xyy + position, centre, radius) - res, \
                         sphereSDF(t.yxy + position, centre, radius) - res, \
                         sphereSDF(t.yyx + position, centre, radius) - res));
}

void main() {
    ivec2 viewportDimensions = viewport.zw;
    float iTime = 0;
    // Initialise the final colour stuff
    vec3 col = vec3(0);
    
    // Defining some colours
    const vec3 sky = vec3(0, .1, .2);
    const vec3 diffuse = vec3(.9, .2, .2) * .8;
    const vec3 specular = vec3(.2);
    const vec3 ambient = sky / 50.;  
    
    // Light position (it rotates)
    vec3 light = vec3(0., 2., 2.);
	
    // Getting where the pixel is on the screen
    vec2 uv = (gl_FragCoord.xy/viewportDimensions.xy - .5) * 2.;
    uv.x *= viewportDimensions.x/viewportDimensions.y;
	
    // Gets the actual vector pointing into the scene
    //ivec2 iMouse = ivec2(0, 0);
	//vec3 eye = vec3(5. * sin(iMouse.x/100.), 5. * sin(iMouse.y/100.), 5. * cos(iMouse.x/100.));
	//vec3 eye = vec3(0., 5., 5.);
    vec3 eye = eye_position;
    const vec3 up = vec3(0., 1., 0.);
    const vec3 looking_at = vec3(0);
    vec3 direction = normalize(looking_at - eye);
	
    vec3 canvas_side = normalize(cross(up, direction));
    vec3 canvas_up = cross(direction, canvas_side);

    direction = normalize(direction + uv.x * .6 * canvas_side + uv.y * .6 * canvas_up);
    
    // Sets up the shapes in the scene 

    // Torus definition
    vec2 tds = vec2(2, .5);
    
    
    // Actual raymarching here
    float dist = 0.;
    
    for (int i; i < 200; i++) {
        //dist += sphereSDF(direction * dist + eye, centre, radius);
        dist += torusSDF(direction * dist + eye, tds);
    }

    // Shading the points in the scene
    if (torusSDF(direction * dist + eye, tds) < 0.0001) {
        vec3 intersection = direction * dist + eye;
        //vec3 normal = sphereNorm(intersection, centre, radius);
        vec3 normal = torusNorm(intersection, tds);
        vec3 to_light = normalize(light - intersection);
        
        // Lambertian coefficient
        float lambert = max(0., dot(normal, to_light));
        
        col += lambert * diffuse;
        col += specular * pow(max(0., dot(reflect(to_light, normal), direction)), 100.) * step(0., lambert);
        col += ambient;
    }
    else col = sky;

    // Output to screen
    Color = vec4(pow(col, vec3(1./2.2)), 1.0);
}