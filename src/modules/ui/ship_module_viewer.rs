use maat_graphics::DrawCall;
use maat_gui::widgets::{Widget, Image, Button, Text, RadioButton};

use crate::modules::ui::{Ui, UiData, OptionsUi};
use crate::modules::entities::{BoxEntity};

use crate::cgmath::{Vector2, Vector4};

enum UiIndex {
  ShipModuleViewer,
}

impl UiIndex {
  pub fn n(self) -> usize {
    self as usize
  }
}

enum WidgetIndex {
  Background,
}

impl WidgetIndex {
  pub fn n(self) -> usize {
    self as usize
  }
}

#[derive(Clone)]
pub struct ShipModuleViewer {
  data: UiData,
}

impl ShipModuleViewer {
  pub fn new(window_size: Vector2<f32>) -> ShipModuleViewer {
    let background_colour = Vector4::new(0.2, 0.2, 0.35, 1.0);
    let button_colour = Vector4::new(0.8, 0.8, 0.2, 1.0);
    let font = "Arial".to_string();
    
    let background = ShipModuleViewer::create_background_image(window_size, background_colour);
    
    ShipModuleViewer {
      data: UiData::new()
                    .with_widget(background)
                    ,
    }
  }
  
  fn background_position(window_size: Vector2<f32>) -> Vector2<f32> {
    Vector2::new(window_size.x*0.835, window_size.y*0.5)
  }
  
  fn background_size(window_size: Vector2<f32>) -> Vector2<f32> {
    Vector2::new(window_size.x*0.33, window_size.y)
  }
  
  fn create_background_image(window_size: Vector2<f32>, colour: Vector4<f32>) -> Box<Widget> {
    let pos = ShipModuleViewer::background_position(window_size);
    let size = ShipModuleViewer::background_size(window_size);
    
    Box::new(Image::new(pos, size).with_primary_colour(colour))
  }
  
  fn create_button(window_size: Vector2<f32>, primary_colour: Vector4<f32>, position: Vector2<f32>, size: Vector2<f32>, font: String, text: String) -> (Box<Widget>, Box<Widget>) {
    
    let button = Box::new(Button::new(position, size)
                                  .with_primary_colour(primary_colour));
    
    let text = Box::new(Text::new(position-Vector2::new(0.0, size.y*0.25), size.y*2.2, &font, &text).center_text());
    
    (button, text)
  }
  
  pub fn updated_positions(window_size: Vector2<f32>) -> Vec<Vector2<f32>> {
     vec!(ShipModuleViewer::background_position(window_size))
  }
  
  pub fn updated_sizes(window_size: Vector2<f32>) -> Vec<Vector2<f32>> {
     vec!(ShipModuleViewer::background_size(window_size))
  }
}


impl Ui for ShipModuleViewer {
  fn data(&self) -> &UiData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut UiData {
    &mut self.data
  }
  
  fn check_if_needs_reenabling(&mut self) {
    
  }
  
  fn update_ui(&mut self, mouse_pos: Vector2<f32>, left_mouse: bool, escape_pressed: bool, window_size: Vector2<f32>, mut should_close: &mut bool, _should_resize: &mut Option<(Vector2<f32>, bool)>, should_next_scene: &mut bool, _delta_time: f32) {
    let new_positions = ShipModuleViewer::updated_positions(window_size);
    let new_sizes = ShipModuleViewer::updated_sizes(window_size);
    for i in 0..self.data().widgets.len() {
      self.mut_data().widgets[i].set_position(new_positions[i]);
      self.mut_data().widgets[i].set_size(new_sizes[i]);
    }
  }
}


