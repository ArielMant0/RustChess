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
layout(location = 2) in vec3 frag_pos;
layout(location = 3) in vec3 view_pos;

layout(location = 0) out vec4 f_color;

const vec3 LIGHT = vec3(0.0, 10.0, 0.0);

void main() {

	vec3 norm = normalize(v_normal);
	vec3 light_dir = normalize(LIGHT - frag_pos);

	float diff = max(dot(norm, light_dir), 0.0);
	vec3 light_color = vec3(1.0, 1.0, 1.0);
	vec3 diffuse = diff * light_color;

	float ambient_strength = 0.2;
	vec3 ambient = ambient_strength * light_color;

	float specular_strength = 1.0;
    vec3 view_dir = normalize(view_pos - frag_pos);
	vec3 reflect_dir = reflect(-light_dir, norm);

	float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32);
	vec3 specular = specular_strength * spec * light_color;

	// 	vec3 result = (diffuse + ambient + specular) * base_color;
	if (diffuse.x < 0.2 && diffuse.y < 0.2 && diffuse.z < 0.2) {
		vec3 result = ambient * base_color;
	}
	vec3 result = (diffuse + specular) * base_color;

	f_color = vec4(result, 1.0f);
}