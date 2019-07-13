pub use self::ability_ui::AbilityUi;
pub use self::options_ui::OptionsUi;
pub use self::pause_ui::PauseUi;
pub use self::ship_select_ui::ShipSelectUi;
pub use self::ship_module_viewer::ShipModuleViewer;
pub use self::ability_switch_ui::AbilitySwitchUI;

mod ability_ui;
mod options_ui;
mod pause_ui;
mod ship_select_ui;
mod ship_module_viewer;
mod ability_switch_ui;

use maat_graphics::DrawCall;

use maat_gui::widgets::Widget;

use crate::cgmath::Vector2;

pub type BoxUi = Box<Ui>;

#[derive(Clone)]
pub struct UiData {
  widgets: Vec<Box<Widget>>,
  uis: Option<Vec<BoxUi>>,
  enabled: bool,
  external_option_value: i32,
}

impl UiData {
  pub fn new() -> UiData {
    UiData {
      widgets: Vec::new(),
      uis: None,
      enabled: true,
      external_option_value: -1,
    }
  }
  
  pub fn disable(mut self) -> UiData {
    self.enabled = false;
    self
  }
  
  pub fn with_widget(mut self, new_widget: Box<Widget>) -> UiData {
    self.widgets.push(new_widget);
    self
  }
  
  pub fn with_ui(mut self, new_ui: BoxUi) -> UiData {
    if let Some(uis) = &mut self.uis {
      uis.push(new_ui);
    } else {
      self.uis = Some(vec!(new_ui));
    }
    
    self
  }
}

pub trait UiClone {
  fn clone_ui(&self) -> Box<Ui>;
}

impl<T: 'static + Ui + Clone> UiClone for T {
  fn clone_ui(&self) -> Box<Ui> {
    Box::new(self.clone())
  }
}

impl Clone for Box<Ui> {
  fn clone(&self) -> Box<Ui> {
    self.clone_ui()
  }
}

pub trait Ui: UiClone {
  fn data(&self) -> &UiData;
  fn mut_data(&mut self) -> &mut UiData;
  
  fn custom_draw(&self, draw_calls: &mut Vec<DrawCall>);
  
  fn external_option_value(&self) -> i32 {
    self.data().external_option_value
  }
  
  fn check_mouse_in_ui_space(&self, mouse_pos: Vector2<f32>) -> bool {
    let mut is_in_ui_space = false;
    for widget in &self.data().widgets {
      if widget.check_mouse_collision(mouse_pos) {
        is_in_ui_space = true;
        break;
      }
    }
    
    is_in_ui_space
  }
  
  fn enabled(&self) -> bool {
    let mut enabled = false;
    if self.data().enabled {
      enabled = true;
    } else {
      if let Some(uis) = &self.data().uis {
        for ui in uis {
          if ui.enabled() {
            enabled = true;
            break;
          }
        }
      }
    }
    
    enabled
  }
  
  fn enable(&mut self) {
    self.mut_data().enabled = true;
    
  }
  
  fn disable(&mut self) {
    self.mut_data().enabled = false;
  }
  
  fn update_widgets(&mut self, mouse_pos: Vector2<f32>, left_mouse: bool, scroll_delta: f32, delta_time: f32) {
    for widget in &mut self.mut_data().widgets {
      widget.update(mouse_pos, left_mouse, scroll_delta, delta_time);
    }
  }
  
  fn update_inner_uis(&mut self, mouse_pos: Vector2<f32>, left_mouse: bool, escape_pressed: bool, window_size: Vector2<f32>, should_close: &mut bool, should_resize: &mut Option<(Vector2<f32>, bool)>, should_next_scene: &mut bool, scroll_delta: f32, delta_time: f32) {
    if let Some(uis) = &mut self.mut_data().uis {
      for ui in uis {
        ui.update(mouse_pos, left_mouse, escape_pressed, window_size, should_close, should_resize, should_next_scene, scroll_delta, delta_time);
      }
    }
  }
  
  fn update_ui(&mut self, mouse_pos: Vector2<f32>, left_mouse: bool, escape_pressed: bool, window_size: Vector2<f32>, should_close: &mut bool,  should_resize: &mut Option<(Vector2<f32>, bool)>, should_next_scene: &mut bool, delta_time: f32);
  fn check_if_needs_reenabling(&mut self);
  
  fn update(&mut self, mouse_pos: Vector2<f32>, left_mouse: bool, escape_pressed: bool, window_size: Vector2<f32>, should_close: &mut bool, should_resize: &mut Option<(Vector2<f32>, bool)>, should_next_scene: &mut bool, scroll_delta: f32, delta_time: f32) {
    self.update_inner_uis(mouse_pos, left_mouse, escape_pressed, window_size, should_close, should_resize, should_next_scene, scroll_delta, delta_time);
    
    if !self.data().enabled {
      self.check_if_needs_reenabling();
      return;
    }
    
    self.update_widgets(mouse_pos, left_mouse, scroll_delta, delta_time);
    self.update_ui(mouse_pos, left_mouse, escape_pressed, window_size, should_close, should_resize, 
                   should_next_scene, delta_time);
    
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::set_texture_scale(1.0));
    draw_calls.push(DrawCall::reset_ortho_camera());
    
    if self.data().enabled {
      for widget in &self.data().widgets {
        widget.draw(draw_calls);
      }
    }
    
    if let Some(uis) = &self.data().uis {
      for ui in uis {
        ui.draw(draw_calls);
      }
    }
    
    self.custom_draw(draw_calls);
  }
}
