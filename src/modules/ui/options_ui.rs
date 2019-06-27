use maat_graphics::DrawCall;

use crate::modules::ui::{Ui, UiData};
use maat_gui::widgets::{Widget, Image};

use crate::cgmath::{Vector2, Vector4};

pub struct OptionsUi {
  data: UiData,
}

enum WidgetIndex {
  Background,
}

impl WidgetIndex {
  pub fn n(self) -> usize {
    self as usize
  }
}

impl OptionsUi {
  pub fn new(window_size: Vector2<f32>) -> OptionsUi {
    let pause_width = window_size.x*0.8;
    let pause_height = window_size.y*0.8;
    let positions = OptionsUi::realign_widget_positions(window_size, pause_height);
    let background = Box::new(Image::new(positions[WidgetIndex::Background.n()], Vector2::new(pause_width, pause_height))
                                     .with_colour(Vector4::new(0.0, 0.0, 0.4, 1.0)));
    
    OptionsUi {
      data: UiData::new()
                    .with_widget(background)
    }
  }
  
  fn realign_widget_positions(window_size: Vector2<f32>, pause_height: f32) -> Vec<Vector2<f32>> {
    let background_position = window_size*0.5;
    
    // Backgound pos, resume pos, resume text pos, options pos, options text pos, quit pos, quit text pos
    vec!(background_position)
  }
}

impl Ui for OptionsUi {
  fn data(&self) -> &UiData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut UiData {
    &mut self.data
  }
  
  fn update_ui(&mut self, mouse_pos: Vector2<f32>, left_mouse: bool, window_size: Vector2<f32>, should_close: &mut bool, _delta_time: f32) {
    
  }
}
