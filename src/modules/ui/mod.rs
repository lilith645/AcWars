pub use self::ability_ui::AbilityUi;
pub use self::options_ui::OptionsUi;
pub use self::pause_ui::PauseUi;

mod ability_ui;
mod options_ui;
mod pause_ui;

use maat_graphics::DrawCall;

use maat_gui::widgets::Widget;

use crate::cgmath::Vector2;

pub struct UiData {
  widgets: Vec<Box<Widget>>,
  uis: Option<Vec<Box<Ui>>>,
  enabled: bool,
}

impl UiData {
  pub fn new() -> UiData {
    UiData {
      widgets: Vec::new(),
      uis: None,
      enabled: true,
    }
  }
  
  pub fn with_widget(mut self, new_widget: Box<Widget>) -> UiData {
    self.widgets.push(new_widget);
    self
  }
  
  pub fn with_ui(mut self, new_ui: Box<Ui>) -> UiData {
    if let Some(uis) = &mut self.uis {
      uis.push(new_ui);
    } else {
      self.uis = Some(vec!(new_ui));
    }
    
    self
  }
}

pub trait Ui {
  fn data(&self) -> &UiData;
  fn mut_data(&mut self) -> &mut UiData;
  
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
  
  fn enable(&mut self) {
    self.mut_data().enabled = true;
  }
  
  fn disable(&mut self) {
    self.mut_data().enabled = false;
  }
  
  fn update_widgets(&mut self, mouse_pos: Vector2<f32>, left_mouse: bool, _delta_time: f32) {
    for widget in &mut self.mut_data().widgets {
      widget.update(mouse_pos, left_mouse, _delta_time);
    }
  }
  
  fn update_inner_uis(&mut self, mouse_pos: Vector2<f32>, left_mouse: bool, window_size: Vector2<f32>, should_close: &mut bool, delta_time: f32) {
    if let Some(uis) = &mut self.mut_data().uis {
      for ui in uis {
        ui.update(mouse_pos, left_mouse, window_size, should_close, delta_time);
      }
    }
  }
  
  fn update_ui(&mut self, mouse_pos: Vector2<f32>, left_mouse: bool, window_size: Vector2<f32>, should_close: &mut bool, delta_time: f32);
  
  fn update(&mut self, mouse_pos: Vector2<f32>, left_mouse: bool, window_size: Vector2<f32>, should_close: &mut bool, _delta_time: f32) {
    self.update_inner_uis(mouse_pos, left_mouse, window_size, should_close, _delta_time);
    
    if !self.data().enabled {
      return;
    }
    
    self.update_ui(mouse_pos, left_mouse, window_size, should_close, _delta_time);
    self.update_widgets(mouse_pos, left_mouse, _delta_time);
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    if !self.data().enabled {
      return;
    }
    
    for widget in &self.data().widgets {
      widget.draw(draw_calls);
    }
    
    if let Some(uis) = &self.data().uis {
      for ui in uis {
        ui.draw(draw_calls);
      }
    }
  }
}
