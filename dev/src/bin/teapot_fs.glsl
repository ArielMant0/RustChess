// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_450pack : enable

layout(location = 0) in vec3 v_normal;
layout(location = 1) in vec3 base_color;

layout(location = 0) out vec4 f_color;

const vec3 LIGHT = vec3(0.0, -1.0, 1.0);

void main() {
    float brightness = dot(normalize(v_normal), normalize(LIGHT));
    
	vec3 dark_color = vec3(0.0, 0.0, 0.0);
	if (base_color.r > 0.2) {
		dark_color.r = base_color.r - 0.1;
	}
	if (base_color.g > 0.2) {
		dark_color.g = base_color.g - 0.1;
	}
	if (base_color.b > 0.2) {
		dark_color.b = base_color.b - 0.1;
	}

	//float intensity = clamp(dot(v_normal, -1 * LIGHT), 0.0, 1.0);

	//f_color.rgb = base_color;
	//f_color.a = 1.0;
	//f_color = f_color * vec4(vec3(0.9, 0.85, 0.85) * (0.2 + intensity), 1.0);

	//f_color.r = clamp(f_color.r, 0.0, 1.0);
	//f_color.g = clamp(f_color.g, 0.0, 1.0);
	//f_color.b = clamp(f_color.b, 0.0, 1.0);

    f_color = vec4(mix(dark_color, base_color, brightness), 1.0);
}