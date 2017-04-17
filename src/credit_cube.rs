use spectra::compositing::{ColorMap, DepthMap};
use spectra::framebuffer::Framebuffer2D;
use spectra::luminance::linear::M44;
use spectra::luminance::pipeline::Pipeline;
use spectra::luminance::tess::TessRender;
use spectra::object::Object;
use spectra::resource::{Res, ResCache};
use spectra::shader::{Program, Uniform};
use spectra::text::TextTexture;
use spectra::texture::Unit;
use spectra::transform::Transformable;

const CREDIT_CUBE_PROJ: &'static Uniform<M44> = &Uniform::new(0);
const CREDIT_CUBE_INST: &'static Uniform<M44> = &Uniform::new(1);
const CREDIT_CUBE_CODE_TEXTURE: &'static Uniform<Unit> = &Uniform::new(2);
const CREDIT_CUBE_GFX_1_TEXTURE: &'static Uniform<Unit> = &Uniform::new(3);
const CREDIT_CUBE_GFX_2_TEXTURE: &'static Uniform<Unit> = &Uniform::new(4);
const CREDIT_CUBE_MUSIC_TEXTURE: &'static Uniform<Unit> = &Uniform::new(5);
const CREDIT_CUBE_DIRECTION_TEXTURE: &'static Uniform<Unit> = &Uniform::new(6);
const CREDIT_CUBE_SUPPORT_TEXTURE: &'static Uniform<Unit> = &Uniform::new(7);

pub struct CreditCubeRenderer {
  program: Res<Program>
}

impl CreditCubeRenderer {
  pub fn new(cache: &mut ResCache) -> Self {
    let program = cache.get("credit_cube.glsl", vec![
      CREDIT_CUBE_PROJ.sem("proj"),
      CREDIT_CUBE_INST.sem("inst"),
      CREDIT_CUBE_CODE_TEXTURE.sem("code_tex"),
      CREDIT_CUBE_GFX_1_TEXTURE.sem("gfx_1_tex"),
      CREDIT_CUBE_GFX_2_TEXTURE.sem("gfx_2_tex"),
      CREDIT_CUBE_MUSIC_TEXTURE.sem("music_tex"),
      CREDIT_CUBE_DIRECTION_TEXTURE.sem("direction_tex"),
      CREDIT_CUBE_SUPPORT_TEXTURE.sem("support_tex"),
    ]).unwrap();

    CreditCubeRenderer {
      program: program
    }
  }

  pub fn render(&self,
                framebuffer: &Framebuffer2D<ColorMap, DepthMap>,
                cube: &Object,
                proj: M44,
                code_texture: &TextTexture,
                gfx_1_texture: &TextTexture,
                gfx_2_texture: &TextTexture,
                music_texture: &TextTexture,
                direction_texture: &TextTexture,
                support_texture: &TextTexture) {
    let model = cube.model.borrow();
    let inst = *cube.transform().as_ref();

    let unis = [
      CREDIT_CUBE_PROJ.alter(proj),
      CREDIT_CUBE_INST.alter(inst),
      CREDIT_CUBE_CODE_TEXTURE.alter(Unit::new(0)),
      CREDIT_CUBE_GFX_1_TEXTURE.alter(Unit::new(1)),
      CREDIT_CUBE_GFX_2_TEXTURE.alter(Unit::new(2)),
      CREDIT_CUBE_MUSIC_TEXTURE.alter(Unit::new(3)),
      CREDIT_CUBE_DIRECTION_TEXTURE.alter(Unit::new(4)),
      CREDIT_CUBE_SUPPORT_TEXTURE.alter(Unit::new(5))
    ];
    let textures = [
      &***code_texture,
      &***gfx_1_texture,
      &***gfx_2_texture,
      &***music_texture,
      &***direction_texture,
      &***support_texture,
    ];

    Pipeline::new(framebuffer, [0., 0., 0., 0.], &textures, &[]).enter(|shd_gate| {
      shd_gate.new(&self.program.borrow(), &unis, &[], &[]).enter(|rdr_gate| {
        rdr_gate.new(None, true, &[], &[], &[]).enter(|tess_gate| {
          let parts = &model.parts;
          let tess = &parts[0].tess;

          tess_gate.render(TessRender::one_whole(tess), &[], &[], &[]);
        });
      });
    });
  }
}
