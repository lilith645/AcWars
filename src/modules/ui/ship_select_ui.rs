use maat_graphics::DrawCall;
use maat_gui::widgets::{Widget, Image, Button, Text, RadioButton};

use crate::modules::ui::{Ui, UiData, OptionsUi};
use crate::modules::entities::{BoxEntity};

use crate::cgmath::{Vector2, Vector4};

enum UiIndex {
  ShipSelectUi,
}

impl UiIndex {
  pub fn n(self) -> usize {
    self as usize
  }
}

enum WidgetIndex {
  Background,
  PlayButton,
  PlayText,
  QuitButton,
  QuitText,
  ShipOptions,
}

impl WidgetIndex {
  pub fn n(self) -> usize {
    self as usize
  }
}

#[derive(Clone)]
pub struct ShipSelectUi {
  data: UiData,
}

impl ShipSelectUi {
  pub fn new(window_size: Vector2<f32>, textures: Vec<String>) -> ShipSelectUi {
    let background_colour = Vector4::new(0.2, 0.2, 0.35, 1.0);
    let button_colour = Vector4::new(0.8, 0.8, 0.2, 1.0);
    let font = "Arial".to_string();
    
    let background = ShipSelectUi::create_background_image(window_size, background_colour);
    let play_button_position = ShipSelectUi::play_button_position(window_size);
    let button_size = ShipSelectUi::button_size(window_size);
    let (play_button, play_text) = ShipSelectUi::create_button(window_size, button_colour, play_button_position,
                                                               button_size, font.to_string(), "Play".to_string());
    
    let quit_button_position = ShipSelectUi::quit_button_position(window_size);
    let (quit_button, quit_text) = ShipSelectUi::create_button(window_size, button_colour, quit_button_position,
                                                               button_size, font.to_string(), "Quit".to_string());
    
    let radio_button_position = ShipSelectUi::radio_button_position(window_size);
    let radio_button_size = ShipSelectUi::radio_button_size(window_size);
    let mut ship_options = RadioButton::new(radio_button_position, radio_button_size);
    
    for texture in &textures {
      ship_options = ship_options.add_radio_option(texture);
    }
    let ship_options = Box::new(ship_options);
    
    ShipSelectUi {
      data: UiData::new()
                    .with_widget(background)
                    .with_widget(play_button)
                    .with_widget(play_text)
                    .with_widget(quit_button)
                    .with_widget(quit_text)
                    .with_widget(ship_options)
                    ,
    }
  }
  
  fn background_position(window_size: Vector2<f32>) -> Vector2<f32> {
    window_size*0.5
  }
  
  fn background_size(window_size: Vector2<f32>) -> Vector2<f32> {
    window_size
  }
  
  fn create_background_image(window_size: Vector2<f32>, colour: Vector4<f32>) -> Box<Widget> {
    let pos = ShipSelectUi::background_position(window_size);
    let size = ShipSelectUi::background_size(window_size);
    
    Box::new(Image::new(pos, size).with_primary_colour(colour))
  }
  
  fn play_button_position(window_size: Vector2<f32>) -> Vector2<f32> {
    Vector2::new(window_size.x*0.9, window_size.y*0.1)
  }
  
  fn quit_button_position(window_size: Vector2<f32>) -> Vector2<f32> {
    Vector2::new(window_size.x*0.1, window_size.y*0.1)
  }
  
  fn button_size(window_size: Vector2<f32>) -> Vector2<f32> {
    Vector2::new(window_size.x*0.12, window_size.y*0.07)
  }
  
  fn radio_button_position(window_size: Vector2<f32>) -> Vector2<f32> {
    Vector2::new(window_size.x*0.1, window_size.y*0.9)
  }
  
  fn radio_button_size(window_size: Vector2<f32>) -> Vector2<f32> {
    Vector2::new(window_size.y*0.12, window_size.y*0.12)
  }
  
  fn create_button(window_size: Vector2<f32>, primary_colour: Vector4<f32>, position: Vector2<f32>, size: Vector2<f32>, font: String, text: String) -> (Box<Widget>, Box<Widget>) {
    
    let button = Box::new(Button::new(position, size)
                                  .with_primary_colour(primary_colour));
    
    let text = Box::new(Text::new(position-Vector2::new(0.0, size.y*0.25), size.y*2.2, &font, &text).center_text());
    
    (button, text)
  }
}


impl Ui for ShipSelectUi {
  fn data(&self) -> &UiData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut UiData {
    &mut self.data
  }
  
  fn check_if_needs_reenabling(&mut self) {
    
  }
  
  fn update_ui(&mut self, mouse_pos: Vector2<f32>, left_mouse: bool, escape_pressed: bool, window_size: Vector2<f32>, mut should_close: &mut bool, _should_resize: &mut Option<(Vector2<f32>, bool)>, should_next_scene: &mut bool, _delta_time: f32) {
    
    if self.data().widgets[WidgetIndex::PlayButton.n()].pressed() {
      self.mut_data().external_option_value = self.data().widgets[WidgetIndex::ShipOptions.n()].external_option_value();
      if self.data().external_option_value > -1 {
        *should_next_scene = true;
      }
    }
    
    if self.data().widgets[WidgetIndex::QuitButton.n()].pressed() {
      *should_close = true;
    }
  }
}


