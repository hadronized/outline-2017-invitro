use spectra::camera::Camera;
use spectra::compositing::{ColorMap, DepthMap};
use spectra::framebuffer::Framebuffer2D;
use spectra::light::Dir;
use spectra::luminance::buffer::{Binding, Buffer};
use spectra::luminance::linear::M44;
use spectra::luminance::pipeline::Pipeline;
use spectra::luminance::tess::TessRender;
use spectra::luminance::texture::Unit;
use spectra::object::Object;
use spectra::projection::Projectable;
use spectra::resource::{Res, ResCache};
use spectra::shader::{Program, Uniform};
use spectra::texture::TextureRGBA32F;
use spectra::transform::Transformable;
use std::cell::RefCell;

const SHADING_PROJ: &'static Uniform<M44> = &Uniform::new(0);
const SHADING_VIEW: &'static Uniform<M44> = &Uniform::new(1);
const SHADING_INST: &'static Uniform<M44> = &Uniform::new(2);
const SHADING_CAM_POS: &'static Uniform<[f32; 3]> = &Uniform::new(3);
const SHADING_COLOR_MAP: &'static Uniform<Unit> = &Uniform::new(4);
const SHADING_USE_COLOR_MAP: &'static Uniform<bool> = &Uniform::new(5);

const SHADING_DIR_LIGHT: &'static Uniform<Binding> = &Uniform::new(0);

pub struct ForwardRenderer {
  shading_program: Res<Program>,
  dir_light_buffer: RefCell<Buffer<Dir>>
}

impl ForwardRenderer {
  pub fn new(cache: &mut ResCache) -> Self {
    let sems = vec![
      SHADING_PROJ.sem("proj"),
      SHADING_VIEW.sem("view"),
      SHADING_INST.sem("inst"),
      SHADING_CAM_POS.sem("cam_pos"),
      SHADING_COLOR_MAP.sem("color_map"),
      SHADING_USE_COLOR_MAP.sem("use_color_map"),
      SHADING_DIR_LIGHT.sem("dir_light")
    ];
    let shading_program = cache.get("forward_renderer/shading.glsl", sems).unwrap();
    let dir_light_buffer = RefCell::new(Buffer::new(1));

    ForwardRenderer {
      shading_program: shading_program,
      dir_light_buffer: dir_light_buffer
    }
  }

  pub fn render<'a, C, O>(&self,
                          framebuffer: &Framebuffer2D<ColorMap, DepthMap>,
                          camera: &Camera<C>,
                          dir_light: &Dir,
                          objects: O)
      where Camera<C>: Projectable + Transformable,
            O: IntoIterator<Item = &'a (&'a Object, Option<&'a TextureRGBA32F>)> {
    let shading_program = self.shading_program.borrow();
    let proj = *camera.projection().as_ref();
    let view = *camera.transform().as_ref();

    let mut dir_light_buffer = self.dir_light_buffer.borrow_mut();
    dir_light_buffer.set(0, *dir_light);

    let uniform_buffers = [&**dir_light_buffer];

    Pipeline::new(framebuffer, [0., 0., 0., 0.], &[], &uniform_buffers).enter(|shd_gate| {
      let unis = [
        SHADING_PROJ.alter(proj),
        SHADING_VIEW.alter(view),
        SHADING_CAM_POS.alter(*camera.position.as_ref()),
        SHADING_DIR_LIGHT.alter(Binding::new(0))
      ];

      shd_gate.new(&shading_program, &unis, &[], &[]).enter(|rdr_gate| {
        for &(obj, color_tex) in objects {
          let model = obj.model.borrow();
          let inst = *obj.transform().as_ref();

          // TODO: refactor this shit
          if let Some(color_tex) = color_tex {
            let unis = [
              SHADING_INST.alter(inst),
              SHADING_COLOR_MAP.alter(Unit::new(0)),
              SHADING_USE_COLOR_MAP.alter(true)
            ];

            rdr_gate.new(None, true, &unis, &[color_tex], &[]).enter(|tess_gate| {
              for part in &model.parts {
                let tess_render = TessRender::one_whole(&part.tess);
                tess_gate.render(tess_render, &[], &[], &[]);
              }
            });
          } else {
            let unis = [
              SHADING_INST.alter(inst),
              SHADING_USE_COLOR_MAP.alter(false)
            ];

            rdr_gate.new(None, true, &unis, &[], &[]).enter(|tess_gate| {
              for part in &model.parts {
                let tess_render = TessRender::one_whole(&part.tess);
                tess_gate.render(tess_render, &[], &[], &[]);
              }
            });
          }
        }
      });
    });
  }
}
