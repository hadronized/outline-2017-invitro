//! Outline 2017 Invitation
//!
//! Released at Revision 2017, April 14th to 17th 2017 in Saarbrücken, Germany.
//!
//! Code: phaazon
//! GFX: preej, lycan, mxbyte, optic, ploopie
//! Music: defcon8
//! Direction: ramonb5, phaazon
//! Support: havoc
//!
//! Disclaimer: this invitro was started less than two weeks before the Revision deadline. The code
//! is messy. Really, messy. phaazon feels ashamed about it. You’ll find dirty hacks all around the
//! place. You’ve been warned.
#![feature(const_fn)]

extern crate spectra;

use spectra::audio::Audio;
use spectra::bootstrap::{Action, Device, EventHandler, EventSig, Key, WindowDim};
use spectra::camera::Camera;
use spectra::color::{RGB, RGBA};
use spectra::compositing::{Compositor, Node, RenderLayer};
use spectra::edit::{Clip, Cut, Track, Timeline, Overlap, Played};
use spectra::extra::new_cube;
use spectra::linear::{Direction, One, Orientation, Position, Scale};
use spectra::light::{Dir, LightProp};
use spectra::model::{Model, Part};
use spectra::object::Object;
use spectra::overlay::{Overlay, RenderInput as OverlayRenderInput, Text, Vert};
use spectra::projection::{Perspective, Projectable};
use spectra::resource::{Res, ResCache};
use spectra::shader::Program;
use spectra::spline::Spline;
use spectra::text::Rasterizer as TextRasterizer;
use spectra::texture::{Sampler, TextureImage, TextureRGBA32F, Wrap};
use std::f32::consts::FRAC_PI_2;

// TODO: remove that for release
use std::cell::RefCell;
use std::rc::Rc;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const TITLE: &'static str = "Outline 2017 – Invitation";

mod credit_cube;
mod forward_renderer;
//mod handler;
#[macro_use]
mod script;

use credit_cube::CreditCubeRenderer;
use forward_renderer::ForwardRenderer;
//use handler::{DemoHandler, Timing};

fn main() {
  Audio::open("data/audio/Outline17.ogg", demo);
}

fn demo(mut audio: Audio) {
  // ---------------------------------------------------
  // -- COMMON STUFF -----------------------------------
  let mut cache = ResCache::new("data");
  let mut dev = Device::bootstrap(WindowDim::FullScreen, TITLE);
  let persp = Perspective::new(WIDTH as f32 / HEIGHT as f32, FRAC_PI_2, 0.1, 100.);
  let mut handler = EscapeHandler;
  let mut compositor = Compositor::new(WIDTH, HEIGHT, &mut cache);
  let forward_renderer = ForwardRenderer::new(&mut cache);
  let credit_cube_renderer = CreditCubeRenderer::new(&mut cache);
  let text_rasterizer = TextRasterizer::from_file("data/fonts/varsity_regular.ttf").unwrap();
  let overlay = Overlay::new(WIDTH, HEIGHT, 0, 0, 100, &mut cache);

  // ---------------------------------------------------
  // -- RESOURCES --------------------------------------

  // colors
  let outline_blue = RGBA::new(0.2196078431372549, 0.3254901960784314, 0.8823529411764706, 1.);

  // effects
  let background_effect = cache.get::<Program>("background.glsl", vec![]).unwrap();
  let background_effect = background_effect.borrow();

  // models
  let cube = Res::new(Model::from_parts(vec![Part::new(new_cube())]));
  let outline_emblem = cache.get::<Model>("Outline-Logo-final.obj", ()).unwrap();
  let hedra_01 = cache.get::<Model>("DSR_OTLINV_Hedra01_Hi.obj", ()).unwrap();
  let hedra_02 = cache.get::<Model>("DSR_OTLINV_Hedra02_Hi.obj", ()).unwrap();
  let hedra_04 = cache.get::<Model>("DSR_OTLINV_Hedra04_Hi.obj", ()).unwrap();
  let hedra_04b = cache.get::<Model>("DSR_OTLINV_Hedra04b_Hi.obj", ()).unwrap();
  let twist_01 = cache.get::<Object>("DSR_OTLINV_Twist01_Hi.json", ()).unwrap();

  // textures
  let hedra_tex_01 = cache.get::<TextureImage>("DSR_OTLINV_Hedra01_DM01.png", (Sampler::default(), None)).unwrap();
  let hedra_tex_02 = cache.get::<TextureImage>("DSR_OTLINV_Hedra01_DM01b.png", (Sampler::default(), None)).unwrap();
  let hedra_tex_03 = cache.get::<TextureImage>("DSR_OTLINV_Hedra01_DM01c.png", (Sampler::default(), None)).unwrap();
  let hedra_tex_04 = cache.get::<TextureImage>("DSR_OTLINV_Hedra01_DM01d.png", (Sampler::default(), None)).unwrap();
  let hedra_tex_05 = cache.get::<TextureImage>("DSR_OTLINV_Hedra01_DM01e.png", (Sampler::default(), None)).unwrap();
  let hedra_tex_06 = cache.get::<TextureImage>("DSR_OTLINV_Hedra01_DM02.png", (Sampler::default(), None)).unwrap();
  let dsr_logo = cache.get::<TextureImage>("DESiREKid.png", (Sampler::default(), None)).unwrap();
  let dsr_logo = dsr_logo.borrow();
  let tus_logo = cache.get::<TextureImage>("tus.png", (Sampler::default(), None)).unwrap();
  let tus_logo = tus_logo.borrow();
  let outline_bg_sampler = Sampler {
    wrap_r: Wrap::MirroredRepeat,
    wrap_s: Wrap::MirroredRepeat,
    wrap_t: Wrap::MirroredRepeat,
    .. Sampler::default()
  };
  let outline_bg = cache.get::<TextureImage>("outline_bg.png", (outline_bg_sampler, None)).unwrap();
  let outline_bg = outline_bg.borrow();
  let bunny_tex = cache.get::<TextureImage>("a-little-easter-bunny.png", (Sampler::default(), None)).unwrap();
  let bunny_tex = bunny_tex.borrow();

  // texts
  let text_dsr_and_tus = script_text!(text_rasterizer, script::SCRIPT_DSR_AND_TUS, 72.);
  let text_proud_to_present = script_text!(text_rasterizer, script::SCRIPT_PROUD_TO_PRESENT, 60.);
  let text_that_atari_party = script_text!(text_rasterizer, script::SCRIPT_THAT_ATARI_PARTY, 60.);
  let text_farm = script_text!(text_rasterizer, script::SCRIPT_FARM, 60.);
  let text_location = script_text!(text_rasterizer, script::SCRIPT_LOCATION, 60.);
  let text_date = script_text!(text_rasterizer, script::SCRIPT_DATE, 60.);
  let text_comes_with = script_text!(text_rasterizer, script::SCRIPT_COMES_WITH, 60.);
  let text_features = script::SCRIPT_FEATURES.iter().map(|t| text_rasterizer.rasterize(t, 60.).unwrap()).collect::<Vec<_>>();
  let text_features_sum_up = script::SCRIPT_FEATURES_SUM_UP.iter().map(|t| text_rasterizer.rasterize(t, 60.).unwrap()).collect::<Vec<_>>();
  let text_experience_it = script_text!(text_rasterizer, script::SCRIPT_EXPERIENCE_IT, 60.);
  let text_secret_cheese = script_text!(text_rasterizer, script::SCRIPT_SECRET_CHEESE, 60.);
  let text_greets = script::SCRIPT_GREETS.iter().map(|t| text_rasterizer.rasterize(t, 80.).unwrap()).collect::<Vec<_>>();
  let text_credit_code = script_text!(text_rasterizer, script::SCRIPT_CREDIT_CODE, 80.);
  let text_credit_gfx_1 = script_text!(text_rasterizer, script::SCRIPT_CREDIT_GFX_1, 80.);
  let text_credit_gfx_2 = script_text!(text_rasterizer, script::SCRIPT_CREDIT_GFX_2, 80.);
  let text_credit_gfx_3 = script_text!(text_rasterizer, script::SCRIPT_CREDIT_GFX_3, 80.);
  let text_credit_gfx_4 = script_text!(text_rasterizer, script::SCRIPT_CREDIT_GFX_4, 80.);
  let text_credit_gfx_5 = script_text!(text_rasterizer, script::SCRIPT_CREDIT_GFX_5, 80.);
  let text_credit_music = script_text!(text_rasterizer, script::SCRIPT_CREDIT_MUSIC, 80.);
  let text_credit_direction = script_text!(text_rasterizer, script::SCRIPT_CREDIT_DIRECTION, 80.);
  let text_credit_support = script_text!(text_rasterizer, script::SCRIPT_CREDIT_SUPPORT, 80.);

  // ---------------------------------------------------
  // -- ANIMATION SPLINES ------------------------------
  let dsr_logo_mask = cache.get::<Spline<f32>>("desire_logo_mask.json", ()).unwrap();
  let tus_logo_mask = cache.get::<Spline<f32>>("tus_logo_mask.json", ()).unwrap();
  let blue_mask = cache.get::<Spline<f32>>("blue_mask.json", ()).unwrap();
  let outline_emblem_position = cache.get::<Spline<Position>>("outline_emblem_position.json", ()).unwrap();
  let outline_emblem_orientation = cache.get::<Spline<Orientation>>("outline_emblem_orientation.json", ()).unwrap();
  let outline_emblem_scale = cache.get::<Spline<Scale>>("outline_emblem_scale.json", ()).unwrap();
  let emblem_mask = cache.get::<Spline<f32>>("emblem_mask.json", ()).unwrap();
  let cube_orientation = cache.get::<Spline<Orientation>>("cube_orientation.json", ()).unwrap();
  let text_mask = cache.get::<Spline<f32>>("text_mask.json", ()).unwrap();
  let toy_01_scale = cache.get::<Spline<Scale>>("toy_01_scale.json", ()).unwrap();
  let toy_02_scale = cache.get::<Spline<Scale>>("toy_02_scale.json", ()).unwrap();
  let toy_03_scale = cache.get::<Spline<Scale>>("toy_03_scale.json", ()).unwrap();
  let toy_04_scale = cache.get::<Spline<Scale>>("toy_04_scale.json", ()).unwrap();
  let toy_01_orient = cache.get::<Spline<Orientation>>("toy_01_orient.json", ()).unwrap();
  let toy_02_orient = cache.get::<Spline<Orientation>>("toy_02_orient.json", ()).unwrap();
  let toy_03_orient = cache.get::<Spline<Orientation>>("toy_03_orient.json", ()).unwrap();
  let toy_04_orient = cache.get::<Spline<Orientation>>("toy_04_orient.json", ()).unwrap();
  let camera_position = cache.get::<Spline<Position>>("camera_position.json", ()).unwrap();
  let camera_orientation = cache.get::<Spline<Orientation>>("camera_orientation.json", ()).unwrap();

  // ---------------------------------------------------
  // -- CLIPS ------------------------------------------
  let splash_screen_clip = Clip::new(|t| {
    let t = t as f32;
    let blue_mask = blue_mask.borrow().clamped_sample(t);
    let blue = RGBA { w: blue_mask, ..outline_blue };

    let dsr: Node = Node::Texture(&dsr_logo, None);
    let dsr_mask_value = dsr_logo_mask.borrow().clamped_sample(t);
    let dsr_mask = RGBA::new(dsr_mask_value, dsr_mask_value, dsr_mask_value, dsr_mask_value).into();

    let tus = Node::Texture(&tus_logo, Some([1.5, 1.5]));
    let tus_mask_value = tus_logo_mask.borrow().clamped_sample(t);
    let tus_mask = RGBA::new(tus_mask_value, tus_mask_value, tus_mask_value, tus_mask_value).into();

    dsr * dsr_mask + (tus * tus_mask).over(blue.into())
  });

  let emblem_clip = Clip::new(|t| {
    let t = t as f32;

    let blue_mask = blue_mask.borrow().clamped_sample(t);
    let blue = RGBA { w: blue_mask, ..outline_blue };

    let forward_renderer = &forward_renderer;
    let outline_emblem = outline_emblem.clone();
    let outline_emblem_position = outline_emblem_position.clone();
    let outline_emblem_orientation = outline_emblem_orientation.clone();
    let outline_emblem_scale = outline_emblem_scale.clone();
    let emblem_mask_value = emblem_mask.borrow().clamped_sample(t);
    let emblem_mask = RGBA::new(1., 1., 1., emblem_mask_value).into();

    let camera_position = camera_position.clone();

    let emblem_render = Node::Render(RenderLayer::new(move |framebuffer| {
      let light_color = RGB::new(0.8, 0.8, 0.8);
      let dir_light = Dir::new(LightProp::new(light_color, light_color, 20.), Direction::new(1., 1., 1.).into());
      let position = outline_emblem_position.borrow().clamped_sample(t);
      let orientation = outline_emblem_orientation.borrow().clamped_sample(t);
      let scale = outline_emblem_scale.borrow().clamped_sample(t);
      let outline_emblem_object = Object::new(outline_emblem.clone(), position, orientation, scale);

      let it: &[(&Object, Option<&TextureRGBA32F>)] = &[
        (&outline_emblem_object, None),
      ];

      let camera = Camera::new(camera_position.borrow().clamped_sample(t), One::one(), persp);
      forward_renderer.render(framebuffer, &camera, &dir_light, it.into_iter());
    }));
    
    let tus_mask_value = tus_logo_mask.borrow().clamped_sample(t);
    let tus_mask = RGBA::new(tus_mask_value, tus_mask_value, tus_mask_value, tus_mask_value);

    (emblem_render * emblem_mask).over(Node::Color(blue).over(Node::Texture(&outline_bg, Some([2., 2.])))) * tus_mask.into()
  });

  let party_details_1_clip = Clip::new(|t| {
    let t = t as f32;

    let overlay = &overlay;
    let text_dsr_and_tus = &text_dsr_and_tus;
    let text_proud_to_present = &text_proud_to_present;

    let render_node = Node::Render(RenderLayer::new(move |framebuffer| {
      let textures = [
        Text::new(&*text_dsr_and_tus, Vert::new([WIDTH as f32 * 0.5 - text_dsr_and_tus.size().0 as f32 * 0.5, HEIGHT as f32 * 0.5 - 20., 0.], [1., 1., 1., 1.])),
        Text::new(&*text_proud_to_present, Vert::new([WIDTH as f32 * 0.5 - text_proud_to_present.size().0 as f32 * 0.5, HEIGHT as f32 * 0.5 - 80., 0.], [1., 1., 1., 1.])),
      ];

      let render_input = OverlayRenderInput::new()
        .texts(&textures, 1.);

      overlay.render(framebuffer, &render_input);
    }));

    let text_mask = RGBA::new(1., 1., 1., text_mask.borrow().clamped_sample(t));

    render_node * text_mask.into()
  });

  let party_details_2_clip = Clip::new(|t| {
    let t = t as f32;

    let overlay = &overlay;
    let text_that_atari_party = &text_that_atari_party;
    let text_farm = &text_farm;
    let text_location = &text_location;

    let render_node = Node::Render(RenderLayer::new(move |framebuffer| {
      let textures = [
        Text::new(&*text_that_atari_party, Vert::new([WIDTH as f32 * 0.5 - text_that_atari_party.size().0 as f32 * 0.5, HEIGHT as f32 * 0.5 - 80., 0.], [1., 1., 1., 1.])),
        Text::new(&*text_farm, Vert::new([WIDTH as f32 * 0.5 - text_farm.size().0 as f32 * 0.5, HEIGHT as f32 * 0.5 - 140., 0.], [1., 1., 1., 1.])),
        Text::new(&*text_location, Vert::new([WIDTH as f32 * 0.5 - text_location.size().0 as f32 * 0.5, HEIGHT as f32 * 0.5 - 200., 0.], [1., 1., 1., 1.])),
      ];

      let render_input = OverlayRenderInput::new()
        .texts(&textures, 1.);

      overlay.render(framebuffer, &render_input);
    }));

    let text_mask = RGBA::new(1., 1., 1., text_mask.borrow().clamped_sample(t));

    render_node * text_mask.into()
  });

  let party_details_3_clip = Clip::new(|t| {
    let t = t as f32;

    let overlay = &overlay;
    let text_date = &text_date;

    let render_node = Node::Render(RenderLayer::new(move |framebuffer| {
      let textures = [
        Text::new(&*text_date, Vert::new([WIDTH as f32 * 0.5 - text_date.size().0 as f32 * 0.5, HEIGHT as f32 * 0.5 - 80., 0.], [1., 1., 1., 1.])),
      ];

      let render_input = OverlayRenderInput::new()
        .texts(&textures, 1.);

      overlay.render(framebuffer, &render_input);
    }));

    let text_mask = RGBA::new(1., 1., 1., text_mask.borrow().clamped_sample(t));

    render_node * text_mask.into()
  });

  let party_details_4_clip = Clip::new(|t| {
    let t = t as f32;

    let overlay = &overlay;
    let text_comes_with = &text_comes_with;

    let render_node = Node::Render(RenderLayer::new(move |framebuffer| {
      let textures = [
        Text::new(&*text_comes_with, Vert::new([WIDTH as f32 * 0.5 - text_comes_with.size().0 as f32 * 0.5, HEIGHT as f32 * 0.5 - 80., 0.], [1., 1., 1., 1.])),
      ];

      let render_input = OverlayRenderInput::new()
        .texts(&textures, 1.);

      overlay.render(framebuffer, &render_input);
    }));

    let text_mask = RGBA::new(1., 1., 1., text_mask.borrow().clamped_sample(t));

    render_node * text_mask.into()
  });

  let party_details_5_clip = Clip::new(|t| {
    let t = t as f32;

    let overlay = &overlay;
    let text_features = &text_features;

    let render_node = Node::Render(RenderLayer::new(move |framebuffer| {
      let textures = [
        Text::new(&text_features[0], Vert::new([WIDTH as f32 * 0.5 - text_features[0].size().0 as f32 * 0.5, HEIGHT as f32 * 0.5 - 80., 0.], [1., 1., 1., 1.])),
        Text::new(&text_features[1], Vert::new([WIDTH as f32 * 0.5 - text_features[1].size().0 as f32 * 0.5, HEIGHT as f32 * 0.5 - 140., 0.], [1., 1., 1., 1.])),
        Text::new(&text_features[2], Vert::new([WIDTH as f32 * 0.5 - text_features[2].size().0 as f32 * 0.5, HEIGHT as f32 * 0.5 - 200., 0.], [1., 1., 1., 1.])),
        Text::new(&text_features[3], Vert::new([WIDTH as f32 * 0.5 - text_features[3].size().0 as f32 * 0.5, HEIGHT as f32 * 0.5 - 260., 0.], [1., 1., 1., 1.])),
      ];

      let render_input = OverlayRenderInput::new()
        .texts(&textures, 1.);

      overlay.render(framebuffer, &render_input);
    }));

    let text_mask = RGBA::new(1., 1., 1., text_mask.borrow().clamped_sample(t));

    render_node * text_mask.into()
  });

  let party_details_6_clip = Clip::new(|t| {
    let t = t as f32;

    let overlay = &overlay;
    let text_features_sum_up = &text_features_sum_up[0];

    let render_node = Node::Render(RenderLayer::new(move |framebuffer| {
      let textures = [
        Text::new(&text_features_sum_up, Vert::new([WIDTH as f32 * 0.5 - text_features_sum_up.size().0 as f32 * 0.5, HEIGHT as f32 * 0.5 - 80., 0.], [1., 1., 1., 1.])),
      ];

      let render_input = OverlayRenderInput::new()
        .texts(&textures, 1.);

      overlay.render(framebuffer, &render_input);
    }));

    let text_mask = RGBA::new(1., 1., 1., text_mask.borrow().clamped_sample(t));

    render_node * text_mask.into()
  });

  let party_details_7_clip = Clip::new(|t| {
    let t = t as f32;

    let overlay = &overlay;
    let text_features_sum_up = &text_features_sum_up[1];

    let render_node = Node::Render(RenderLayer::new(move |framebuffer| {
      let textures = [
        Text::new(&text_features_sum_up, Vert::new([WIDTH as f32 * 0.5 - text_features_sum_up.size().0 as f32 * 0.5, HEIGHT as f32 * 0.5 - 80., 0.], [1., 1., 1., 1.])),
      ];

      let render_input = OverlayRenderInput::new()
        .texts(&textures, 1.);

      overlay.render(framebuffer, &render_input);
    }));

    let text_mask = RGBA::new(1., 1., 1., text_mask.borrow().clamped_sample(t));

    render_node * text_mask.into()
  });

  let party_details_8_clip = Clip::new(|t| {
    let t = t as f32;

    let overlay = &overlay;
    let text_features_sum_up = &text_features_sum_up[2];

    let render_node = Node::Render(RenderLayer::new(move |framebuffer| {
      let textures = [
        Text::new(&text_features_sum_up, Vert::new([WIDTH as f32 * 0.5 - text_features_sum_up.size().0 as f32 * 0.5, HEIGHT as f32 * 0.5 - 80., 0.], [1., 1., 1., 1.])),
      ];

      let render_input = OverlayRenderInput::new()
        .texts(&textures, 1.);

      overlay.render(framebuffer, &render_input);
    }));

    let text_mask = RGBA::new(1., 1., 1., text_mask.borrow().clamped_sample(t));

    render_node * text_mask.into()
  });

  let party_details_9_clip = Clip::new(|t| {
    let t = t as f32;

    let overlay = &overlay;
    let text_features_sum_up = &text_features_sum_up[3];

    let render_node = Node::Render(RenderLayer::new(move |framebuffer| {
      let textures = [
        Text::new(&text_features_sum_up, Vert::new([WIDTH as f32 * 0.5 - text_features_sum_up.size().0 as f32 * 0.5, HEIGHT as f32 * 0.5 - 80., 0.], [1., 1., 1., 1.])),
      ];

      let render_input = OverlayRenderInput::new()
        .texts(&textures, 1.);

      overlay.render(framebuffer, &render_input);
    }));

    let text_mask = RGBA::new(1., 1., 1., text_mask.borrow().clamped_sample(t));

    render_node * text_mask.into()
  });

  let greets_clip = Clip::new(|t| {
    let t_ = t as f32;
    let t = t_ - 77.;

    let forward_renderer = &forward_renderer;
    let twist_01 = twist_01.clone();
    let hedra_tex_06 = hedra_tex_06.clone();
    let camera_position = camera_position.clone();
    let camera_orientation = camera_orientation.clone();

    let twist_node = Node::Render(RenderLayer::new(move |framebuffer| {
      let light_color = RGB::new(0.8, 0.8, 0.8);
      let dir_light = Dir::new(LightProp::new(light_color, light_color, 20.), Direction::new(0., 1., 1.).into());
      let twist_01 = twist_01.borrow();
      let hedra_tex_06 = hedra_tex_06.borrow();

      let objects: &[(&Object, Option<&TextureRGBA32F>)] = &[
        (&twist_01, Some(&hedra_tex_06))
      ];

      let camera = Camera::new(camera_position.borrow().clamped_sample(t_), camera_orientation.borrow().clamped_sample(t_), persp);
      forward_renderer.render(framebuffer, &camera, &dir_light, objects);
    }));

    let overlay = &overlay;
    let text_greets = &text_greets;

    let text_node = Node::Render(RenderLayer::new(move |framebuffer| {
      let textures = [
        Text::new(&text_greets[0], Vert::new([WIDTH as f32 - t*28. + 20., HEIGHT as f32 * 0.5 - 400., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[1], Vert::new([WIDTH as f32 - t*33. + 80., HEIGHT as f32 * 0.5 - 140., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[2], Vert::new([WIDTH as f32 - t*82. + 30., HEIGHT as f32 * 0.5 - 250., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[3], Vert::new([WIDTH as f32 - t*34. + 10., HEIGHT as f32 * 0.5 - 20., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[4], Vert::new([WIDTH as f32 - t*48. + 30., HEIGHT as f32 * 0.5 + 40., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[5], Vert::new([WIDTH as f32 - t*58. + 50., HEIGHT as f32 * 0.5 + 100., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[6], Vert::new([WIDTH as f32 - t*86. + 193., HEIGHT as f32 * 0.5 + 150., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[7], Vert::new([WIDTH as f32 - t*37. + 32., HEIGHT as f32 * 0.5 + 220., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[8], Vert::new([WIDTH as f32 - t*43. + 20., HEIGHT as f32 * 0.5 - 280., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[9], Vert::new([WIDTH as f32 - t*71. + 83., HEIGHT as f32 * 0.5 - 340., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[10], Vert::new([WIDTH as f32 - t*33. + 200., HEIGHT as f32 * 0.5 + 20., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[11], Vert::new([WIDTH as f32 - t*72. + 80., HEIGHT as f32 * 0.5 + 80., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[12], Vert::new([WIDTH as f32 - t*98. + 130., HEIGHT as f32 * 0.5 + 120., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[13], Vert::new([WIDTH as f32 - t*34. + 100., HEIGHT as f32 * 0.5 + 140., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[14], Vert::new([WIDTH as f32 - t*75. + 150., HEIGHT as f32 * 0.5 + 200., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[15], Vert::new([WIDTH as f32 - t*42. + 90., HEIGHT as f32 * 0.5 - 212., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[16], Vert::new([WIDTH as f32 - t*88. + 40., HEIGHT as f32 * 0.5 - 379., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[17], Vert::new([-100. + t*144., HEIGHT as f32 * 0.5 - 212., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[18], Vert::new([-120. + t*23., HEIGHT as f32 * 0.5 - 149., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[19], Vert::new([-140. + t*99., HEIGHT as f32 * 0.5 - 100., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[20], Vert::new([-190. + t*132., HEIGHT as f32 * 0.5 - 10., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[21], Vert::new([-110. + t*43., HEIGHT as f32 * 0.5 + 90., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[22], Vert::new([-140. + t*110., HEIGHT as f32 * 0.5 + 140., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[23], Vert::new([-170. + t*43., HEIGHT as f32 * 0.5 + 200., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[24], Vert::new([-240. + t*74., HEIGHT as f32 * 0.5 + 260., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[25], Vert::new([-120. + t*48., HEIGHT as f32 * 0.5 + 310., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[26], Vert::new([-300. + t*66., HEIGHT as f32 * 0.5 + 480., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[27], Vert::new([-130. + t*82., HEIGHT as f32 * 0.5 + 230., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[28], Vert::new([-280. + t*34., HEIGHT as f32 * 0.5 + 460., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[29], Vert::new([-130. + t*78., HEIGHT as f32 * 0.5 - 53., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[30], Vert::new([-150. + t*42., HEIGHT as f32 * 0.5 - 150., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[31], Vert::new([-210. + t*59., HEIGHT as f32 * 0.5 - 200., 0.], [1., 1., 1., 1.])),
        Text::new(&text_greets[32], Vert::new([-190. + t*41., HEIGHT as f32 * 0.5 - 343., 0.], [1., 1., 1., 1.])),
      ];

      let render_input = OverlayRenderInput::new()
        .texts(&textures, 1.);

      overlay.render(framebuffer, &render_input);
    }));

    let text_mask = RGBA::new(1., 1., 1., text_mask.borrow().clamped_sample(t_));

    twist_node.over((text_node * text_mask.into())).over(Node::FullScreenEffect(&background_effect))
  });

  let toys_clip = Clip::new(|t| {
    let t = t as f32;

    let forward_renderer = &forward_renderer;

    let hedra_01 = hedra_01.clone();
    let hedra_02 = hedra_02.clone();
    let hedra_03 = hedra_04.clone();
    let hedra_04 = hedra_04b.clone();

    let hedra_tex_01 = hedra_tex_01.clone();
    let hedra_tex_02 = hedra_tex_02.clone();
    let hedra_tex_03 = hedra_tex_03.clone();
    let hedra_tex_04 = hedra_tex_04.clone();
    let hedra_tex_05 = hedra_tex_05.clone();
    let hedra_tex_06 = hedra_tex_06.clone();

    let toy_01_orient = toy_01_orient.clone();
    let toy_02_orient = toy_02_orient.clone();
    let toy_03_orient = toy_03_orient.clone();
    let toy_04_orient = toy_04_orient.clone();

    let toy_01_scale = toy_01_scale.clone();
    let toy_02_scale = toy_02_scale.clone();
    let toy_03_scale = toy_03_scale.clone();
    let toy_04_scale = toy_04_scale.clone();

    let mask = tus_logo_mask.clone();

    let camera_position = camera_position.clone();
    let camera_orientation = camera_orientation.clone();

    let render_node = Node::Render(RenderLayer::new(move |framebuffer| {
      let light_color = RGB::new(0.8, 0.8, 0.8);
      let dir_light = Dir::new(LightProp::new(light_color, light_color, 20.), Direction::new(0., 1., 1.).into());

      let hedra_tex_01 = hedra_tex_01.borrow();
      let hedra_tex_02 = hedra_tex_02.borrow();
      let hedra_tex_03 = hedra_tex_03.borrow();
      let hedra_tex_04 = hedra_tex_04.borrow();
      let hedra_tex_05 = hedra_tex_05.borrow();
      let hedra_tex_06 = hedra_tex_06.borrow();

      let obj_01 = Object::new(hedra_01.clone(), Position::new(5., 0., 0.), toy_01_orient.borrow().clamped_sample(t), toy_01_scale.borrow().clamped_sample(t));
      let obj_02 = Object::new(hedra_02.clone(), Position::new(0., 5., 0.), toy_02_orient.borrow().clamped_sample(t), toy_02_scale.borrow().clamped_sample(t));
      let obj_03 = Object::new(hedra_03.clone(), Position::new(5., 0., 5.), toy_03_orient.borrow().clamped_sample(t), toy_03_scale.borrow().clamped_sample(t));
      let obj_04 = Object::new(hedra_04.clone(), Position::new(-10., 0., 0.), toy_04_orient.borrow().clamped_sample(t), toy_04_scale.borrow().clamped_sample(t));
      let obj_05 = Object::new(hedra_01.clone(), Position::new(0., -10., 0.), toy_02_orient.borrow().clamped_sample(t), toy_02_scale.borrow().clamped_sample(t));
      let obj_06 = Object::new(hedra_02.clone(), Position::new(0., 0., -10.), toy_04_orient.borrow().clamped_sample(t), toy_04_scale.borrow().clamped_sample(t));
      let obj_07 = Object::new(hedra_03.clone(), Position::new(0., -10., -10.), toy_01_orient.borrow().clamped_sample(t), toy_03_scale.borrow().clamped_sample(t));
      let obj_08 = Object::new(hedra_04.clone(), Position::new(-10., -10., -10.), toy_03_orient.borrow().clamped_sample(t), toy_01_scale.borrow().clamped_sample(t));

      let objects: &[(&Object, Option<&TextureRGBA32F>)] = &[
        (&obj_01, Some(&hedra_tex_01)),
        (&obj_02, Some(&hedra_tex_02)),
        (&obj_03, Some(&hedra_tex_03)),
        (&obj_04, Some(&hedra_tex_04)),
        (&obj_05, Some(&hedra_tex_04)),
        (&obj_06, Some(&hedra_tex_05)),
        (&obj_07, Some(&hedra_tex_06)),
        (&obj_08, Some(&hedra_tex_02)),
      ];

      let camera = Camera::new(camera_position.borrow().clamped_sample(t), camera_orientation.borrow().clamped_sample(t), persp);

      forward_renderer.render(framebuffer, &camera, &dir_light, objects);
    }));

    let v = mask.borrow().clamped_sample(t);
    let mask = RGBA::new(v, v, v, v);

    (render_node * mask.into()).over(Node::FullScreenEffect(&background_effect))
  });

  let credit_clip_1 = Clip::new(|t| {
    let t = t as f32;

    let credit_cube_renderer = &credit_cube_renderer;
    let cube = cube.clone();
    let cube_orientation = cube_orientation.clone();

    let text_credit_code = &text_credit_code;
    let text_credit_gfx_1 = &text_credit_gfx_1;
    let text_credit_gfx_2 = &text_credit_gfx_2;
    let text_credit_gfx_3 = &text_credit_gfx_3;
    let text_credit_gfx_4 = &text_credit_gfx_4;
    let text_credit_gfx_5 = &text_credit_gfx_5;
    
    let render_node = Node::Render(RenderLayer::new(move |framebuffer| {
      let cube_position = Position::new(0., 0., 3.);
      let cube_orientation = cube_orientation.borrow().clamped_sample(t);
      let cube_object = Object::new(cube.clone(), cube_position, cube_orientation, One::one());

      credit_cube_renderer.render(framebuffer,
                                  &cube_object,
                                  *persp.projection().as_ref(),
                                  text_credit_gfx_5,
                                  text_credit_gfx_1,
                                  text_credit_gfx_2,
                                  text_credit_gfx_3,
                                  text_credit_gfx_4,
                                  text_credit_code);
    }));

    render_node.over(Node::Texture(&outline_bg, Some([2., 2.])) * Node::Color(RGBA::new(0.5, 0.5, 0.5, 1.).into()))
  });

  let credit_clip_2 = Clip::new(|t| {
    let t = t as f32;

    let credit_cube_renderer = &credit_cube_renderer;
    let cube = cube.clone();
    let cube_orientation = cube_orientation.clone();

    let text_credit_code = &text_credit_code;
    let text_credit_music = &text_credit_music;
    let text_credit_direction = &text_credit_direction;
    let text_credit_support = &text_credit_support;
    let text_credit_gfx_4 = &text_credit_gfx_4;
    let text_credit_gfx_5 = &text_credit_gfx_5;
    
    let render_node = Node::Render(RenderLayer::new(move |framebuffer| {
      let cube_position = Position::new(0., 0., 3.);
      let cube_orientation = cube_orientation.borrow().clamped_sample(t);
      let cube_object = Object::new(cube.clone(), cube_position, cube_orientation, One::one());

      credit_cube_renderer.render(framebuffer,
                                  &cube_object,
                                  *persp.projection().as_ref(),
                                  text_credit_gfx_5,
                                  text_credit_music,
                                  text_credit_direction,
                                  text_credit_support,
                                  text_credit_gfx_4,
                                  text_credit_code);
    }));

    let v = tus_logo_mask.borrow().clamped_sample(t);
    let tus_mask = RGBA::new(v, v, v, v);

    render_node.over(Node::Texture(&outline_bg, Some([2., 2.])) * Node::Color(RGBA::new(0.5, 0.5, 0.5, 1.).into())) * tus_mask.into()
  });

  let outro_bunny_clip = Clip::new(|t| {
    let t = t as f32;
    let v = tus_logo_mask.borrow().clamped_sample(t);
    let color_mask = RGBA::new(v, v, v, v);
    let overlay = &overlay;

    let text_experience_it = &text_experience_it;
    let text_secret_cheese = &text_secret_cheese;

    let render_node = Node::Render(RenderLayer::new(move |framebuffer| {
      let textures = [
        Text::new(&*text_experience_it, Vert::new([WIDTH as f32 * 0.5 - text_secret_cheese.size().0 as f32 * 0.5, HEIGHT as f32 * 0.5 + 400., 0.], [1., 1., 1., 1.])),
        Text::new(&*text_secret_cheese, Vert::new([WIDTH as f32 * 0.5 - text_secret_cheese.size().0 as f32 * 0.5, HEIGHT as f32 * 0.5 - 500., 0.], [1., 1., 1., 1.])),
      ];

      let render_input = OverlayRenderInput::new()
        .texts(&textures, 1.);

      overlay.render(framebuffer, &render_input);
    }));

    render_node.over(Node::Texture(&bunny_tex, Some([WIDTH as f32 / HEIGHT as f32, 1.]))) * color_mask.into()
  });

  // ---------------------------------------------------
  // -- CUTS -------------------------------------------
  let splash_screen_cut   = Cut::new(0., 26.,  0., &splash_screen_clip);
  let emblem_cut          = Cut::new(0., 38., 26., &emblem_clip);
  let party_details_1_cut = Cut::new(0., 10., 30., &party_details_1_clip);
  let party_details_2_cut = Cut::new(0., 6., 43., &party_details_2_clip);
  let party_details_3_cut = Cut::new(0., 6., 49., &party_details_3_clip);
  let party_details_5_cut = Cut::new(0., 12., 55., &party_details_5_clip);
  let toys_cut            = Cut::new(0., 13., 64., &toys_clip);
  let party_details_6_cut = Cut::new(0., 3., 65., &party_details_6_clip);
  let party_details_7_cut = Cut::new(0., 3., 68., &party_details_7_clip);
  let party_details_8_cut = Cut::new(0., 3., 71., &party_details_8_clip);
  let party_details_9_cut = Cut::new(0., 3., 74., &party_details_9_clip);
  let greets_cut          = Cut::new(0., 14., 77., &greets_clip);
  let credit_cut_1        = Cut::new(0., 7.5, 90.5, &credit_clip_1);
  let credit_cut_2        = Cut::new(0., 6., 98., &credit_clip_2);
  let outro_bunny_cut     = Cut::new(0., 15., 104., &outro_bunny_clip);

  // ---------------------------------------------------
  // -- OVERLAPS ---------------------------------------
  let universal_overlap = Overlap::new(0., 1000., |mut nodes| {
    let b = nodes.pop().unwrap();
    let a = nodes.pop().unwrap();

    a.over(b)
  });

  // ---------------------------------------------------
  // -- TRACKS -----------------------------------------
  let main_track = {
    let mut track = Track::new();

    track.add_cut(splash_screen_cut);
    track.add_cut(party_details_1_cut);
    track.add_cut(party_details_2_cut);
    track.add_cut(party_details_3_cut);
    track.add_cut(party_details_5_cut);
    track.add_cut(party_details_6_cut);
    track.add_cut(party_details_7_cut);
    track.add_cut(party_details_8_cut);
    track.add_cut(party_details_9_cut);
    track.add_cut(credit_cut_1);
    track.add_cut(credit_cut_2);
    track.add_cut(outro_bunny_cut);

    track
  };

  let secondary_track = {
    let mut track = Track::new();

    track.add_cut(emblem_cut);
    track.add_cut(toys_cut);
    track.add_cut(greets_cut);

    track
  };

  // ---------------------------------------------------
  // -- TIMELINE ---------------------------------------
  let timeline = {
    let mut timeline = Timeline::new();

    // tracks
    timeline.add_track(main_track);
    timeline.add_track(secondary_track);

    // overlaps
    timeline.add_overlap(universal_overlap);

    timeline
  };

  audio.play();

  while dev.dispatch_events(&mut handler) {
    dev.step(None, |t| {

      let t = audio.cursor() as f64;

      match timeline.play(t) {
        Played::Resolved(play_node) => {
          compositor.display(play_node);
        },
        _ => { 
          compositor.display(RGBA::new(0., 0., 0., 1.).into());
        }
      }
    });
  }
}

pub struct EscapeHandler;

impl EventHandler for EscapeHandler {
  fn on_key(&mut self, k: Key, a: Action) -> EventSig {
    if let (Key::Escape, Action::Release) = (k, a) {
      EventSig::Aborted
    } else {
      EventSig::Ignored
    }
  }
}
