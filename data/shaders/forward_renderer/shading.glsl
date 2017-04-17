#vs

layout (location = 0) in vec3 co;
layout (location = 1) in vec3 no;
layout (location = 2) in vec2 uv;

out vec3 v_co;
out vec3 v_no;
out vec2 v_uv;

uniform mat4 proj;
uniform mat4 view;
uniform mat4 inst;

void main() {
  vec4 co2 = inst * vec4(co, 1.);

  v_co = co2.xyz;
  v_no = normalize((transpose(inverse(inst)) * vec4(no, 0.)).xyz);
  v_uv = uv;

  gl_Position = proj * view * co2;
}

#fs

in vec3 v_co;
in vec3 v_no;
in vec2 v_uv;

uniform vec3 cam_pos;
uniform sampler2D color_map;
uniform bool use_color_map;

uniform dir_light {
  vec3 diff;
  vec3 spec;
  float gloss;
  vec3 dir;
} directional;

layout (location = 0) out vec4 frag_color;
layout (location = 1) out vec4 frag_normal;

const float PI = 3.1415926535;
const float PI_2 = 2. * PI;

float cel_shading(float k) {
  if (k <= 0.24) {
    return 0.;
  } else if (k <= .25) {
    return mix(.5, .1, (.25 - k) / 0.01);
  } else if (k <= .49) {
    return .5;
  } else if (k <= .5) {
    return mix(1., .5, (.5 - k) / 0.01);
  } else {
    return 1.;
  }
}

void main() {
  // scene info
  vec3 ambient = vec3(.1, .1, .1);

  vec3 dir = vec3(0., 0.5, 1.);
  // diffuse
  float kd = max(0., dot(v_no, dir));
  //kd = cel_shading(kd);

  // specular
  float ks = pow(max(0., dot(reflect(-dir, v_no), normalize(cam_pos - v_co))), 50.);
  //ks = cel_shading(ks);

  // color
  vec3 object_color = vec3(1., 1., 1.);
  
  if (use_color_map) {
    object_color = texture(color_map, v_uv).rgb;
  }

  vec3 color = ambient * object_color + (directional.diff * object_color) * kd + (directional.spec * object_color) * ks;

  // outputs
  frag_color = vec4(color, 1.);
  frag_normal = vec4(v_no, 1.);
}
