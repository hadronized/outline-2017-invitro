#vs

layout (location = 0) in vec3 co;
layout (location = 1) in vec3 no;
layout (location = 2) in vec2 uv;

out vec3 v_co;
out vec3 v_no;
out vec2 v_uv;
flat out uint v_face;

uniform mat4 proj;
uniform mat4 inst;

void main() {
  vec4 p = inst * vec4(co, 1.);

  v_co = p.xyz;
  v_no = normalize((transpose(inverse(inst)) * vec4(no, 0.)).xyz);
  v_uv = uv;
  v_face = uint(gl_VertexID / 4);

  gl_Position = proj * p;
}

#fs

in vec3 v_co;
in vec3 v_no;
in vec2 v_uv;
flat in uint v_face;

out vec4 frag;

uniform sampler2D code_tex;
uniform sampler2D gfx_1_tex;
uniform sampler2D gfx_2_tex;
uniform sampler2D music_tex;
uniform sampler2D direction_tex;
uniform sampler2D support_tex;

float fetch() {
  float scale = 4.5;
  float r = 0.;

  // le gros bordel
  switch (v_face) {
    case 0u:
      r = texture(direction_tex, vec2(0., -(scale - 1.) * .5) + v_uv * vec2(1., scale)).r;
      break;

    case 1u:
      {
        vec2 uv = v_uv;
        float x = uv.x;
        uv.x = uv.y;
        uv.y = x;
        r = texture(gfx_1_tex, vec2(0., -(scale - 1.) * .5) + uv * vec2(1., scale)).r;
      }
      break;

    case 2u:
      r = texture(code_tex, vec2(0., -(scale - 1.) * .5) + v_uv * vec2(1., scale)).r;
      break;

    case 3u:
      {
        vec2 uv = vec2(1., 1.) - v_uv;
        r = texture(support_tex, vec2(0., -(scale - 1.) * .5) + uv * vec2(1., scale)).r;
      }
      break;

    case 4u:
      {
        vec2 uv = vec2(1., 1.) - v_uv;
        r = texture(gfx_2_tex, vec2(0., -(scale - 1.) * .5) + uv * vec2(1., scale)).r;
      }
      break;
      
    case 5u:
      r = texture(music_tex, vec2(0., -(scale - 1.) * .5) + v_uv * vec2(1., scale)).r;
      break;
  }

  return r;
}

void main() {
  vec3 text_color = vec3(.8, .8, .8);
  vec3 cube_color = vec3(0.2196078431372549, 0.3254901960784314, 0.8823529411764706);

  vec3 light_color = vec3(.8, .8, .8);
  vec3 light_pos = vec3(0., 0., -1.3);
  vec3 light_dir = normalize(light_pos - v_co);

  float kd = max(0., dot(light_dir, v_no));
  float ks = pow(max(0., dot(reflect(normalize(v_co), v_no), light_dir)), 3.);

  float scale = 8.5;
  float r = fetch();

  vec3 diff = light_color * (cube_color + text_color * r) * kd + (r * ks);

  frag = vec4(diff, 1.);
}
