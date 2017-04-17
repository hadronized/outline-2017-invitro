#vs

out vec2 v_co;

const vec2[4] CO = vec2[](
  vec2(1., -1.),
  vec2(1., 1.),
  vec2(-1., -1.),
  vec2(-1., 1.)
);

void main() {
  vec2 co = CO[gl_VertexID];

  v_co = (co + 1.) * .5;

  gl_Position = vec4(co, 0., 1.);
}

#fs

in vec2 v_co;
out vec4 frag;

void main() {
  vec3 top = vec3(.157, .569, .792);
  vec3 bottom = vec3(.737, .969, .149);

  vec3 color = mix(bottom, top, v_co.y);

  frag = vec4(color, 1.);
}
