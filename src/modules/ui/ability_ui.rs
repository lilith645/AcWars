use crate::modules::player;
use crate::modules::abilities::{Ability};
use crate::modules::ui::{Ui, UiData, AbilitySwitchUI};
use maat_gui::widgets::{Image, RadioButton};

use maat_graphics::DrawCall;

use crate::cgmath::{Vector2, Vector4};

use parking_lot::Mutex;

use std::sync::Arc;

const ABILITY_SWITCH_UI_INDEX: usize = 0;

const RADIO_BUTTON_INDEX: usize = 0;

#[derive(Clone)]
pub struct AbilityUi {
  data: UiData,
  ship_abilities: Arc<Mutex<player::Input>>,
  current_selected_option: i32,
}

impl AbilityUi {
  pub fn new(ship_abilities: Arc<Mutex<player::Input>>, window_dim: Vector2<f32>) -> AbilityUi {
    let positions = AbilityUi::realign_positions(window_dim);
    let ability_size = AbilityUi::get_ability_size(window_dim);
    
    let mut radio_buttons = RadioButton::new(positions[1], ability_size)
                                        .add_radio_option_with_offset(&"".to_string(), positions[1]-positions[1])
                                        .add_radio_option_with_offset(&"".to_string(), positions[2]-positions[1])
                                        .add_radio_option_with_offset(&"".to_string(), positions[3]-positions[1])
                                        .add_radio_option_with_offset(&"".to_string(), positions[4]-positions[1])
                                        .add_radio_option_with_offset(&"".to_string(), positions[5]-positions[1])
                                        .add_radio_option_with_offset(&"".to_string(), positions[6]-positions[1])
                                        .add_radio_option_with_offset(&"".to_string(), positions[7]-positions[1]);
                                        //.unselected_when_clicked();
    
    let q_ability;
    let w_ability;
    let e_ability;
    let r_ability;
    let lm_ability;
    let mm_ability;
    let rm_ability;
    let cd_q_ability;
    let cd_w_ability;
    let cd_e_ability;
    let cd_r_ability;
    let cd_lm_ability;
    let cd_mm_ability;
    let cd_rm_ability;
    
    {
      let ship = ship_abilities.lock();
      
      let textures = ship.get_ability_textures();
      
      q_ability = Box::new(Image::new(positions[1], ability_size)
                                  .with_texture(&textures[0].to_string()));
      w_ability = Box::new(Image::new(positions[2], ability_size)
                                  .with_texture(&textures[1].to_string()));
      e_ability = Box::new(Image::new(positions[3], ability_size)
                                  .with_texture(&textures[2].to_string()));
      r_ability = Box::new(Image::new(positions[4], ability_size)
                                  .with_texture(&textures[3].to_string()));

      lm_ability = Box::new(Image::new(positions[5], ability_size)
                                  .with_texture(&textures[4].to_string()));
      mm_ability = Box::new(Image::new(positions[6], ability_size)
                                  .with_texture(&textures[5].to_string()));
      rm_ability = Box::new(Image::new(positions[7], ability_size)
                                  .with_texture(&textures[6].to_string()));
      
       cd_q_ability = Box::new(Image::new(positions[1], ability_size)
                                  .with_primary_colour(Vector4::new(1.0, 1.0, 1.0, 0.3)));
       cd_w_ability = Box::new(Image::new(positions[2], ability_size)
                                    .with_primary_colour(Vector4::new(1.0, 1.0, 1.0, 0.3)));
       cd_e_ability = Box::new(Image::new(positions[3], ability_size)
                                    .with_primary_colour(Vector4::new(1.0, 1.0, 1.0, 0.3)));
       cd_r_ability = Box::new(Image::new(positions[4], ability_size)
                                    .with_primary_colour(Vector4::new(1.0, 1.0, 1.0, 0.3)));

       cd_lm_ability = Box::new(Image::new(positions[5], ability_size)
                                    .with_primary_colour(Vector4::new(1.0, 1.0, 1.0, 0.3)));
       cd_mm_ability = Box::new(Image::new(positions[6], ability_size)
                                    .with_primary_colour(Vector4::new(1.0, 1.0, 1.0, 0.3)));
       cd_rm_ability = Box::new(Image::new(positions[7], ability_size)
                                    .with_primary_colour(Vector4::new(1.0, 1.0, 1.0, 0.3)));
    }
    
    let mut ui = AbilityUi::create_ability_switch_ui(positions[0], Arc::clone(&ship_abilities), window_dim);
    
    ui.disable();
    
    AbilityUi {
      data: UiData::new()
                    .with_widget(Box::new(radio_buttons))
                    .with_widget(q_ability)
                    .with_widget(w_ability)
                    .with_widget(e_ability)
                    .with_widget(r_ability)
                    .with_widget(lm_ability)
                    .with_widget(mm_ability)
                    .with_widget(rm_ability)
                    .with_widget(cd_q_ability)
                    .with_widget(cd_w_ability)
                    .with_widget(cd_e_ability)
                    .with_widget(cd_r_ability)
                    .with_widget(cd_lm_ability)
                    .with_widget(cd_mm_ability)
                    .with_widget(cd_rm_ability)
                    .with_ui(ui),
      ship_abilities,
      current_selected_option: -1,
    }
  }
  
  pub fn create_ability_switch_ui(position: Vector2<f32>, ship_abilities: Arc<Mutex<player::Input>>, window_dim: Vector2<f32>) -> Box<Ui> {
    let ability_size = AbilityUi::get_ability_size(window_dim);
    
    let ship = ship_abilities.lock();
    
    let all_abilities = ship.all_abilities();
    
    let mut ui = AbilitySwitchUI::new(position, ability_size, AbilityUi::get_ability_spacing(window_dim), all_abilities, window_dim);
    
    Box::new(ui)
  }
  
  pub fn get_ability_size(window_dim: Vector2<f32>) -> Vector2<f32> {
    let size = window_dim.x/25.6;
    Vector2::new(size, size)
  }
  
  pub fn get_ability_space_from_edge(window_dim: Vector2<f32>) -> f32 {
    window_dim.x*0.05
  }
  
  pub fn get_ability_spacing(window_dim: Vector2<f32>) -> f32 {
    AbilityUi::get_ability_space_from_edge(window_dim)*1.0
  }
  
  pub fn q_ability_position(window_size: Vector2<f32>) -> Vector2<f32> {
    let space_from_edge = AbilityUi::get_ability_space_from_edge(window_size);
    Vector2::new(space_from_edge, space_from_edge)
  }
  
  pub fn w_ability_position(window_size: Vector2<f32>) -> Vector2<f32> {
    let spacing = AbilityUi::get_ability_spacing(window_size);
    
    AbilityUi::q_ability_position(window_size)+Vector2::new(spacing, 0.0)
  }
  
  pub fn e_ability_position(window_size: Vector2<f32>) -> Vector2<f32> {
    let spacing = AbilityUi::get_ability_spacing(window_size);
    
    AbilityUi::w_ability_position(window_size)+Vector2::new(spacing, 0.0)
  }
  
  pub fn r_ability_position(window_size: Vector2<f32>) -> Vector2<f32> {
    let spacing = AbilityUi::get_ability_spacing(window_size);
    
    AbilityUi::e_ability_position(window_size)+Vector2::new(spacing, 0.0)  
  }
  
  pub fn right_mouse_ability_position(window_size: Vector2<f32>) -> Vector2<f32> {
    let space_from_edge = AbilityUi::get_ability_space_from_edge(window_size);
    Vector2::new(window_size.x-space_from_edge, space_from_edge)
  }
  
  pub fn middle_mouse_ability_position(window_size: Vector2<f32>) -> Vector2<f32> {
    let spacing = AbilityUi::get_ability_spacing(window_size);
    AbilityUi::right_mouse_ability_position(window_size)-Vector2::new(spacing, 0.0)
  }
  
  pub fn left_mouse_ability_position(window_size: Vector2<f32>) -> Vector2<f32> {
    let spacing = AbilityUi::get_ability_spacing(window_size);
    AbilityUi::middle_mouse_ability_position(window_size)-Vector2::new(spacing, 0.0)
  }
  
  pub fn realign_positions(window_dim: Vector2<f32>) -> Vec<Vector2<f32>> {
    let q_ability = AbilityUi::q_ability_position(window_dim);
    let w_ability = AbilityUi::w_ability_position(window_dim);
    let e_ability = AbilityUi::e_ability_position(window_dim);
    let r_ability = AbilityUi::r_ability_position(window_dim);
    let left_mouse_ability = AbilityUi::left_mouse_ability_position(window_dim);
    let middle_mouse_ability = AbilityUi::middle_mouse_ability_position(window_dim);
    let right_mouse_ability = AbilityUi::right_mouse_ability_position(window_dim);
    
    vec!(q_ability, q_ability, w_ability, e_ability, r_ability, left_mouse_ability, middle_mouse_ability,
         right_mouse_ability,  q_ability, w_ability, e_ability, r_ability, left_mouse_ability, 
         middle_mouse_ability, right_mouse_ability)
  }
}

impl Ui for AbilityUi {
 fn data(&self) -> &UiData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut UiData {
    &mut self.data
  }
  
  fn check_if_needs_reenabling(&mut self) {
    
  }
  
  fn update_ui(&mut self, mouse_pos: Vector2<f32>, left_mouse: bool, _escape_pressed: bool, window_size: Vector2<f32>, should_close: &mut bool, _should_resize: &mut Option<(Vector2<f32>, bool)>, should_next_scene: &mut bool, _delta_time: f32) {
    let positions = AbilityUi::realign_positions(window_size);
    let size = AbilityUi::get_ability_size(window_size);
    
    let mut cooldowns;
    {
      let ship = self.ship_abilities.lock();
      cooldowns = ship.get_ability_cooldowns();
    }
    
    if !self.data().widgets[RADIO_BUTTON_INDEX].check_mouse_collision(mouse_pos) {
      let mut ui_disabled = false;
      let mut new_ability_index = -1;
      if let Some(uis) = &mut self.mut_data().uis {
        if uis[ABILITY_SWITCH_UI_INDEX].enabled() {
          if left_mouse && !uis[ABILITY_SWITCH_UI_INDEX].check_mouse_in_ui_space(mouse_pos) { 
            uis[ABILITY_SWITCH_UI_INDEX].disable();
            ui_disabled = true;
          }
          new_ability_index = uis[ABILITY_SWITCH_UI_INDEX].external_option_value();
        }
      }
      
      if new_ability_index != -1 {
        let textures;
        {
          let mut ship = self.ship_abilities.lock();
          
          match self.data().widgets[RADIO_BUTTON_INDEX].external_option_value() {
            0 => {
              ship.set_ability_one_by_index(new_ability_index as usize);
            },
            1 => {
              ship.set_ability_two_by_index(new_ability_index as usize);
            },
            2 => {
              ship.set_ability_three_by_index(new_ability_index as usize);
            },
            3 => {
              ship.set_ability_four_by_index(new_ability_index as usize);
            },
            4 => {
              ship.set_ability_lm_by_index(new_ability_index as usize);
            },
            5 => {
              ship.set_ability_mm_by_index(new_ability_index as usize);
            },
            6 => {
              ship.set_ability_rm_by_index(new_ability_index as usize);
            },
            _ => {
              
            }
          }
          
          textures = ship.get_ability_textures();
        }
        
        for i in 0..textures.len() {
          self.mut_data().widgets[i+1].set_texture(textures[i].to_string());
        }
      }
      
      if ui_disabled {
        self.mut_data().widgets[RADIO_BUTTON_INDEX].reset_external_option_value();
      }
    }
    
    let mut selected_option = self.data().widgets[RADIO_BUTTON_INDEX].external_option_value();
    if selected_option != -1 {
      let mutex_ship_abilities = Arc::clone(&self.ship_abilities);
      let current_selected_option = self.current_selected_option;
      if let Some(uis) = &mut self.mut_data().uis {
        if !uis[ABILITY_SWITCH_UI_INDEX].enabled() || selected_option != current_selected_option {
          uis[ABILITY_SWITCH_UI_INDEX] = AbilityUi::create_ability_switch_ui(positions[selected_option as usize+1], mutex_ship_abilities, window_size);
          uis[ABILITY_SWITCH_UI_INDEX].enable();
          self.current_selected_option = selected_option;
        }
      }
    } else {
      if let Some(uis) = &mut self.mut_data().uis {
        uis[ABILITY_SWITCH_UI_INDEX].disable();
        self.current_selected_option = -1;
      }
    }
    
    for i in 0..self.data().widgets.len() {
      self.mut_data().widgets[i].set_position(positions[i]);
      if i >= 8 && i < 15 {
        self.mut_data().widgets[i].set_size(Vector2::new(size.x, size.y*cooldowns[(i-1)%7]));
      } else {
        self.mut_data().widgets[i].set_size(size);
      }
    }
  }
  
  fn custom_draw(&self, _draw_calls: &mut Vec<DrawCall>) {
    
  }
}
