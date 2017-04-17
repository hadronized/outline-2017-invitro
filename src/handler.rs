use spectra::bootstrap::{Action, EventHandler, EventSig, FreeflyHandler, Key, MouseButton};
use spectra::camera::{Camera, Freefly};
use spectra::color::RGBA;
use spectra::gui::{self, GUI, ProgressBarListener};
use spectra::resource::ResCache;
use std::cell::RefCell;
use std::rc::Rc;

pub struct DemoHandler<'a> {
  pub freefly_handler: FreeflyHandler,
  pub gui: Rc<RefCell<GUI<'a>>>,
  pub timing: Rc<RefCell<Timing>>
}

pub struct Timing {
  paused: bool,
  pause_t: Option<f64>, // time at which the last pause occurred – if any
  offset_t: f64, // offset seconds to add to time to take pauses into account
  dur: f64, // duration of the demo in seconds
  gui_timeline: Rc<RefCell<gui::ProgressBar>>
}

impl<'a> DemoHandler<'a> {
  pub fn new(dur: f64, w: u32, h: u32, camera: Rc<RefCell<Camera<Freefly>>>, cache: &mut ResCache) -> Self {
    let (w, h) = (w as f32, h as f32);
    let gui = Rc::new(RefCell::new(GUI::new(gui::Viewport { x: 0., y: 0., w: w, h: h }, cache)));
    let gui_timeline = gui::ProgressBar::new(0., 0., w, 14., RGBA::new(0.5, 1.5, 0.5, 1.), RGBA::new(0.3, 0.5, 0.3, 0.5), dur as f32);
    let timing = Rc::new(RefCell::new(Timing::new(dur, gui_timeline.clone())));

    gui_timeline.borrow_mut().add_listener("timing", timing.clone());
    gui.borrow_mut().add_widget("timeline", gui_timeline);

    DemoHandler {
      freefly_handler: FreeflyHandler::new(camera.clone()),
      gui: gui.clone(),
      timing: timing
    }
  }
}

impl Timing {
  fn new(dur: f64, gui_timeline: Rc<RefCell<gui::ProgressBar>>) -> Self {
    Timing {
      paused: true,
      pause_t: Some(0.),
      offset_t: 0.,
      dur: dur,
      gui_timeline: gui_timeline
    }
  }

  pub fn recompute_time(timing: &Rc<RefCell<Timing>>, t: f64) -> f64 {
    if timing.borrow().paused {
      let mut timing = timing.borrow_mut();

      if let Some(pt) = timing.pause_t { // already paused
        pt
      } else {
        let t = (t + timing.offset_t) % timing.dur;
        timing.pause_t = Some(t);
        t
      }
    } else {
      {
        let mut timing = timing.borrow_mut();

        if let Some(pt) = timing.pause_t { // unpause
          timing.offset_t = pt - t;
          timing.pause_t = None;
        }
      }

      let t = {
        let timing = timing.borrow();
        (t + timing.offset_t) % timing.dur
      };

      let gui_timeline = timing.borrow().gui_timeline.clone();
      gui_timeline.borrow_mut().set(t as f32);
      t
    }
  }
}

impl<'a> EventHandler for DemoHandler<'a> {
  fn on_key(&mut self, key: Key, action: Action) -> EventSig {
    let mut gui = self.gui.borrow_mut();

    if self.freefly_handler.on_key(key, action) == EventSig::Aborted || gui.on_key(key, action) == EventSig::Aborted {
      return EventSig::Aborted;
    }
    
    match (key, action) {
      (Key::F5, Action::Release) => {
        let mut timing = self.timing.borrow_mut();
        timing.paused = !timing.paused
      },
      _ => ()
    }

    EventSig::Handled
  }

  fn on_mouse_button(&mut self, button: MouseButton, action: Action) -> EventSig {
    let mut gui = self.gui.borrow_mut();

    self.freefly_handler.on_mouse_button(button, action);
    gui.on_mouse_button(button, action);

    EventSig::Handled
  }

  fn on_cursor_move(&mut self, cursor: [f32; 2]) -> EventSig {
    let mut gui = self.gui.borrow_mut();

    if gui.on_cursor_move(cursor) == EventSig::Ignored {
      self.freefly_handler.on_cursor_move(cursor);
    }

    EventSig::Handled
  }
}

impl ProgressBarListener for Timing {
  fn on_set(&mut self, cursor: f32) {
    if self.paused {
      self.pause_t = Some(cursor as f64);
    }
  }

  fn on_click(&mut self, _: [f32; 2]) {
    self.paused = true;
  }

  fn on_drag(&mut self, _: [f32; 2], _: [f32; 2]) {
    self.paused = true;
  }
}
