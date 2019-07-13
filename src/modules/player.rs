use crate::modules::entities::{BoxEntity};
use crate::modules::abilities::{Ability, NoAbility, BoxAbility, SingleShot, DoubleShot, Move, 
                                Shatter, Shield, Dash, Laser, Haste};

use crate::cgmath::{Vector2, InnerSpace};

use std::sync::{Arc, Mutex};

pub enum _AbilityPosition {
  _LeftClick,
  _RightClick,
  _MiddleClick,
  _Ability1, // q
  _Ability2, // w
  _Ability3, // e
  _Ability4, // r
}

pub struct Input {
  abilities: Vec<BoxAbility>,
  left_click_ability: usize,
  middle_click_ability: usize,
  right_click_ability: usize,
  ability_one: usize,
  ability_two: usize,
  ability_three: usize,
  ability_four: usize,
}

impl Input {
  pub fn new() -> Input {
    let mut double_shot = Box::new(DoubleShot::new());
    double_shot.add_passive(Box::new(Shatter::new()));
    
    let mut abilities: Vec<BoxAbility> = vec!(Box::new(NoAbility::new()),
      Box::new(Move::new()),
      Box::new(Shield::new()),
      double_shot,
      Box::new(Dash::new()),
      Box::new(Laser::new()),
      Box::new(Haste::new()),
      Box::new(SingleShot::new()),
    );
    
    let mut input = Input {
      abilities,
      left_click_ability: 1,
      middle_click_ability: 2,
      right_click_ability: 3,
      ability_one: 4,
      ability_two: 5,
      ability_three: 6,
      ability_four: 7,
    };
    
    
    input
  }
  
  pub fn get_ability_cooldowns(&self) -> Vec<f32> {
    let mut cd: [f32; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    
    cd[0] = self.abilities[self.ability_one].percentage_cooldown_left();
    cd[1] = self.abilities[self.ability_two].percentage_cooldown_left();
    cd[2] = self.abilities[self.ability_three].percentage_cooldown_left();
    cd[3] = self.abilities[self.ability_four].percentage_cooldown_left();
    cd[4] = self.abilities[self.left_click_ability].percentage_cooldown_left();
    cd[5] = self.abilities[self.middle_click_ability].percentage_cooldown_left();
    cd[6] = self.abilities[self.right_click_ability].percentage_cooldown_left();
    
    cd.to_vec()
  }
  
  
  pub fn get_ability_textures(&self) -> Vec<String> {
    let mut textures: [String; 7] = ["NoAbility".to_string(), "NoAbility".to_string(), 
                                     "NoAbility".to_string(), "NoAbility".to_string(), 
                                     "NoAbility".to_string(), "NoAbility".to_string(),
                                     "NoAbility".to_string()];
    
    textures[0] = self.abilities[self.ability_one].texture();
    textures[1] = self.abilities[self.ability_two].texture();
    textures[2] = self.abilities[self.ability_three].texture();
    textures[3] = self.abilities[self.ability_four].texture();
    textures[4] = self.abilities[self.left_click_ability].texture();
    textures[5] = self.abilities[self.middle_click_ability].texture();
    textures[6] = self.abilities[self.right_click_ability].texture();
    
    textures.to_vec()
  }
  
  pub fn all_abilities(&self) -> Vec<BoxAbility> {
    self.abilities.clone()
  }
  
  pub fn ability(&self, index: usize) -> &BoxAbility {
    if index+1 < self.abilities.len() {
      &self.abilities[index+1]
    } else {
      &self.abilities[0]
    }
  }
  
  pub fn mut_ability(&mut self, index: usize) -> &mut BoxAbility {
    if index+1 < self.abilities.len() {
      &mut self.abilities[index+1]
    } else {
      &mut self.abilities[0]
    }
  }
  
  pub fn update_abilities(&mut self, delta_time: f32) {
    for ability in &mut self.abilities {
      ability.update(delta_time);
    }
  }
  
  pub fn set_ability_one_by_index(&mut self, index: usize) {
    self.ability_one = index;
  }
  
  pub fn set_ability_two_by_index(&mut self, index: usize) {
    self.ability_two = index;
  }
  
  pub fn set_ability_three_by_index(&mut self, index: usize) {
    self.ability_three = index;
  }
  
  pub fn set_ability_four_by_index(&mut self, index: usize) {
    self.ability_four = index;
  }
  
  pub fn set_ability_lm_by_index(&mut self, index: usize) {
    self.left_click_ability = index;
  }
  
  pub fn set_ability_mm_by_index(&mut self, index: usize) {
    self.middle_click_ability = index;
  }
  
  pub fn set_ability_rm_by_index(&mut self, index: usize) {
    self.right_click_ability = index;
  }
  
  pub fn update(&mut self, ship: &mut BoxEntity, left_stick_position: Vector2<f32>, a_button_pressed: bool, right_trigger_pressed: bool, mouse_pos: Vector2<f32>, left_mouse: bool, middle_mouse: bool, right_mouse: bool, q_pressed: bool, w_pressed: bool, e_pressed: bool, r_pressed: bool, window_size: Vector2<f32>, delta_time: f32) {
    
    let mut target = mouse_pos;
    let ship_offset = ship.position()-window_size*0.5;
    target += ship_offset;
    
    let hostility = ship.hostility().clone();
    
    self.update_abilities(delta_time);
    
    if q_pressed {
      self.abilities[self.ability_one].activate(ship, target, window_size, &hostility);
    }
    
    if w_pressed {
      self.abilities[self.ability_two].activate(ship, target, window_size, &hostility);
    }
    
    if e_pressed {
      self.abilities[self.ability_three].activate(ship, target, window_size, &hostility);
    }
    
    if r_pressed {
      self.abilities[self.ability_four].activate(ship, target, window_size, &hostility);
    }
    
    if left_mouse {
      self.abilities[self.left_click_ability].activate(ship, target, window_size, &hostility);
    }
    
    if middle_mouse {
      self.abilities[self.middle_click_ability].activate(ship, target, window_size, &hostility);
    }
    
    if right_mouse {
      self.abilities[self.right_click_ability].activate(ship, target, window_size, &hostility);
    }
    
    /*
    if let Some(ability) = &mut self.left_click_ability {
      ability.update(delta_time);
      if left_mouse {
        ability.activate(ship, target, window_size, &hostility);
      }
      
      let dead_zone = 0.01;
      if left_stick_position.magnitude().abs() > dead_zone {
        let x = if left_stick_position.x.abs() > dead_zone { left_stick_position.x } else { 0.0 };
        let y = if left_stick_position.y.abs() > dead_zone { left_stick_position.y } else { 0.0 };
        
        let _radius = 50.0;
        target = ship.position()+Vector2::new(x, y);
        
        ability.activate(ship, target, window_size, &hostility);
      }
    }
    
    if let Some(ability) = &mut self.middle_click_ability {
      ability.update(delta_time);
      if middle_mouse {
        ability.activate(ship, target, window_size, &hostility);
      }
    }
    
    if let Some(ability) = &mut self.right_click_ability {
      ability.update(delta_time);
      if right_mouse || right_trigger_pressed {
        ability.activate(ship, target, window_size, &hostility);
      }
    }
    
    if let Some(ability) = &mut self.ability_one {
      ability.update(delta_time);
      if q_pressed || a_button_pressed {
        ability.activate(ship, target, window_size, &hostility);
      }
    }*/
    
    ship.set_facing(target);
  }
}


