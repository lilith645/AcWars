use crate::modules::player;
use crate::modules::abilities::{BoxAbility};
use crate::modules::ui::{Ui, UiData};
use maat_gui::widgets::{Button, Image};

use maat_graphics::DrawCall;

use crate::cgmath::{Vector2, Vector4};

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AbilitySwitchUI {
  data: UiData,
  all_abilities: Vec<BoxAbility>,
}

impl AbilitySwitchUI {
  pub fn new(position: Vector2<f32>, size: Vector2<f32>, spacing: f32, abilities: Vec<BoxAbility>, window_dim: Vector2<f32>) -> AbilitySwitchUI {
    
    let mut buttons = Vec::new();
    
    let coloums = 3;
    let mut index = 0;
    let mut y_index = 0;
    
    for ability in &abilities {
      if index%coloums == 0 {
        y_index+=1;
      }
      
      let mut temp_pos = position;
      let coloum = index%coloums;
      match coloum {
        0 => { // left
          temp_pos.x -= spacing;
        },
        1 => { // middle
          
        },
        2 => {//right
          temp_pos.x += spacing;
        },
        _ => {}
      }
      
      temp_pos.y += spacing*y_index as f32;
      
      buttons.push(Box::new(Button::new(temp_pos, size).with_texture(&ability.texture())));
      
      
      index += 1;
    }
    
    let mut data = UiData::new();
    
    let mut bg_colour = Vector4::new(0.0, 0.1, 0.4, 0.5);
    let small_bg = Image::new(position, (size+Vector2::new(spacing, spacing))*0.5).with_primary_colour(bg_colour);
    bg_colour.w = 1.0;
    
    let bg_size = Vector2::new(size.x*coloums as f32 + spacing*(coloums as f32+1.0),
                               size.x*y_index as f32 + spacing*(y_index as f32+1.0))*0.5;
    let bg_pos = Vector2::new(position.x, position.y+bg_size.y*0.5+size.y*0.5);
    let large_bg = Image::new(bg_pos, bg_size).with_primary_colour(bg_colour);
    
    data = data.with_widget(Box::new(small_bg));
    data = data.with_widget(Box::new(large_bg));
    
    for button in buttons {
      data = data.with_widget(button);
    }
    
    AbilitySwitchUI {
      data,
      all_abilities: abilities,
    }
  }
}

impl Ui for AbilitySwitchUI {
 fn data(&self) -> &UiData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut UiData {
    &mut self.data
  }
  
  fn check_if_needs_reenabling(&mut self) {
    
  }
  
  fn update_ui(&mut self, _mouse_pos: Vector2<f32>, _left_mouse: bool, _escape_pressed: bool, window_size: Vector2<f32>, should_close: &mut bool, _should_resize: &mut Option<(Vector2<f32>, bool)>, should_next_scene: &mut bool, _delta_time: f32) {
    for i in 2..self.data().widgets.len() {
      if self.data().widgets[i].activated() {
        self.mut_data().external_option_value = (i as i32-2)
      }
    }
  }
  
  fn custom_draw(&self, _draw_calls: &mut Vec<DrawCall>) {
    
  }
}
