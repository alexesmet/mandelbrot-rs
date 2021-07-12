#version 150 core

uniform vec4 base_color = vec4(0.0, 0.0, 0.0, 1.0);
uniform int max_iterations;

uniform vec4 complex_plane;
uniform float pixel_size;
out vec4 color;

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main() {
    vec2 min = complex_plane.xy;
    vec2 c = (pixel_size * gl_FragCoord.xy) + min;

    vec2 z = c;
    int iterationen = 0;

    for (int i = 0; i < max_iterations / 5; i++) {
        z = vec2(pow(z.x, 2) - pow(z.y, 2), 2 * z.x * z.y) + c;
        z = vec2(pow(z.x, 2) - pow(z.y, 2), 2 * z.x * z.y) + c;
        z = vec2(pow(z.x, 2) - pow(z.y, 2), 2 * z.x * z.y) + c;
        z = vec2(pow(z.x, 2) - pow(z.y, 2), 2 * z.x * z.y) + c;
        z = vec2(pow(z.x, 2) - pow(z.y, 2), 2 * z.x * z.y) + c;
        iterationen += 5;
        if (length(z) > 2.0) {
            break;
        }
    }

    /*
    while (iterationen < max_iterations && length(z) < 2.0) {
        z = vec2(pow(z.x, 2) - pow(z.y, 2), 2 * z.x * z.y) + c;
        iterationen++;
    }
    */

    if (iterationen == max_iterations)
        color = base_color;
    else {
        float iter_f32 = float(iterationen);
        float max_iter_f32 = float(max_iterations);
        float perc = iter_f32 / max_iter_f32;

        color = vec4(hsv2rgb(vec3((1 - perc) * 0.75, 1.0, 1.0)), 1.0);
    }
}