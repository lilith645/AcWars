use maat_graphics::DrawCall;
use maat_gui::widgets::{Widget, Image, Button, Text, RadioButton};

use crate::modules::ui::{Ui, UiData};
use crate::modules::entities::{MutexEntity};
use crate::modules::entities::sections::ShipSection;
use crate::modules::abilities::{BoxAbility, SingleShot, DoubleShot, Laser, Dash};

use crate::cgmath::{Vector2, Vector4};

use std::sync::Arc;
/*
enum UiIndex {
  ShipModuleViewer,
}

impl UiIndex {
  pub fn n(self) -> usize {
    self as usize
  }
}*/


//const BACKGROUND: usize = 0;
//const SHIP_VISUAL: usize =  1;
const ABILITY_LIST: usize = 2;

#[derive(Clone)]
pub struct ShipModuleViewer {
  data: UiData,
  mutex_ship: MutexEntity,
}

impl ShipModuleViewer {
  pub fn new(window_size: Vector2<f32>, entity: &MutexEntity) -> ShipModuleViewer {
    let background_colour = Vector4::new(0.2, 0.2, 0.35, 1.0);
    let _button_colour = Vector4::new(0.8, 0.8, 0.2, 1.0);
    let ship_section_colour = Vector4::new(0.2, 0.2, 0.2, 1.0);
    let _font = "Arial".to_string();
    
    let ship = entity.lock().unwrap();
    let ship_sections = ship.ship_sections();
    
    let background = ShipModuleViewer::create_background_image(window_size, background_colour);
    let ship_visual = ShipModuleViewer::create_ship_image(window_size, &ship.texture());
    
    let mut ship_section_visuals = Vec::new();
    for ship_section in ship_sections {
      ship_section_visuals.push(ShipModuleViewer::create_ship_section_image(window_size, ship_section_colour, ship_section));
    }
    
    let abilities: Vec<BoxAbility> = vec!(Box::new(SingleShot::new()), Box::new(DoubleShot::new()), Box::new(Dash::new()), Box::new(Laser::new()));
    let ability_list = ShipModuleViewer::create_ability_list_buttons(window_size, abilities);
    
    let mut ui_data = UiData::new()
                         .with_widget(background)
                         .with_widget(ship_visual)
                         .with_widget(ability_list);
    
    for ship_section in ship_section_visuals {
      ui_data = ui_data.with_widget(ship_section);
    }
    
    ShipModuleViewer {
      data: ui_data,
      mutex_ship: Arc::clone(&entity),
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
  
  fn ship_position(window_size: Vector2<f32>) -> Vector2<f32> {
    Vector2::new(window_size.x*0.835, window_size.y*0.75)
  }
  
  fn ship_size(window_size: Vector2<f32>) -> Vector2<f32> {
    Vector2::new(window_size.x*0.33, window_size.x*0.33)
  }
  
  fn create_ship_image(window_size: Vector2<f32>, texture: &String) -> Box<Widget> {
    let pos = ShipModuleViewer::ship_position(window_size);
    let size = ShipModuleViewer::ship_size(window_size);
    
    Box::new(Image::new(pos, size).with_texture(texture))
  }
  
  fn ship_section_position(window_size: Vector2<f32>, offset: Vector2<f32>) -> Vector2<f32> {
    ShipModuleViewer::ship_position(window_size)+offset
  }
  
  fn create_ship_section_image(window_size: Vector2<f32>, primary_colour: Vector4<f32>, ship_section: &Box<ShipSection>) -> Box<Widget> {
    let pos = ShipModuleViewer::ship_section_position(window_size, ship_section.offset());
    let size = ship_section.size();
    
    Box::new(Image::new(pos, size).with_primary_colour(primary_colour))
  }
  
  
  fn create_button(_window_size: Vector2<f32>, primary_colour: Vector4<f32>, position: Vector2<f32>, size: Vector2<f32>, font: String, text: String) -> (Box<Widget>, Box<Widget>) {
    
    let button = Box::new(Button::new(position, size)
                                  .with_primary_colour(primary_colour));
    
    let text = Box::new(Text::new(position-Vector2::new(0.0, size.y*0.25), size.y*2.2, &font, &text).center_text());
    
    (button, text)
  }
  
  fn ability_list_position(window_size: Vector2<f32>) -> Vector2<f32> {
    let background_size = ShipModuleViewer::background_size(window_size);
    ShipModuleViewer::background_position(window_size)+Vector2::new(-background_size.x*0.45, -background_size.y*0.11)
  }
  
  fn ability_list_size(window_size: Vector2<f32>) -> Vector2<f32> {
    Vector2::new(window_size.y*0.05, window_size.y*0.05)
  }
  
  fn create_ability_list_buttons(window_size: Vector2<f32>, abilities: Vec<BoxAbility>) -> Box<Widget> {
    let pos = ShipModuleViewer::ability_list_position(window_size);
    let size = ShipModuleViewer::ability_list_size(window_size);
    
    let mut radio_buttons = RadioButton::new(pos, size);
    for ability in &abilities {
      radio_buttons = radio_buttons.add_radio_option(&ability.texture());
    }
    
    Box::new(radio_buttons)
  }
  
  pub fn updated_positions(window_size: Vector2<f32>, ship_sections: &Vec<Box<ShipSection>>) -> Vec<Vector2<f32>> {
     let background_pos = ShipModuleViewer::background_position(window_size);
     let ship_visual_pos = ShipModuleViewer::ship_position(window_size);
     let ability_list_pos = ShipModuleViewer::ability_list_position(window_size);
     
     let mut positions = vec!(background_pos, ship_visual_pos, ability_list_pos);
     
     for ship_section in ship_sections {
      positions.push(ShipModuleViewer::ship_section_position(window_size, ship_section.offset()));
     }
     
     positions
  }
  
  pub fn updated_sizes(window_size: Vector2<f32>, ship_sections: &Vec<Box<ShipSection>>) -> Vec<Vector2<f32>> {
     let background_size = ShipModuleViewer::background_size(window_size);
     let ship_size = ShipModuleViewer::ship_size(window_size);
     let ability_list_size = ShipModuleViewer::ability_list_size(window_size);
     
     let mut sizes = vec!(background_size, ship_size, ability_list_size);
     
     for ship_section in ship_sections {
      sizes.push(ship_section.size());
     }
     
     sizes
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
  
  fn update_ui(&mut self, _mouse_pos: Vector2<f32>, _left_mouse: bool, _escape_pressed: bool, window_size: Vector2<f32>, _should_close: &mut bool, _should_resize: &mut Option<(Vector2<f32>, bool)>, _should_next_scene: &mut bool, _delta_time: f32) {
    let new_positions;
    let new_sizes;
    
    {
      let ship = self.mutex_ship.lock().unwrap();
      let ship_sections = ship.ship_sections();
      
      new_positions = ShipModuleViewer::updated_positions(window_size, ship_sections);
      new_sizes = ShipModuleViewer::updated_sizes(window_size, ship_sections);
    }
     
    for i in 0..self.data().widgets.len() {
      if i == ABILITY_LIST {
        continue;
      }
      self.mut_data().widgets[i].set_position(new_positions[i]);
      self.mut_data().widgets[i].set_size(new_sizes[i]);
    }
  }
  
  fn custom_draw(&self, _draw_calls: &mut Vec<DrawCall>) {
    
  }
}


