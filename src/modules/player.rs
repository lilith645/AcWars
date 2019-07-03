use maat_graphics::DrawCall;

use crate::modules::entities::{Entity, BoxEntity};
use crate::modules::abilities::{Ability, BoxAbility, Laser, SingleShot, DoubleShot, Move, ProjectileSpeed, 
                                Shatter, Shield, Dash};

use crate::cgmath::{Vector2, InnerSpace};

pub enum AbilityPosition {
  LeftClick,
  RightClick,
  MiddleClick,
  Ability1, // q
  Ability2, // w
  Ability3, // e
  Ability4, // r
}

pub struct Input {
  left_click_ability: Option<BoxAbility>,
  middle_click_ability: Option<BoxAbility>,
  right_click_ability: Option<BoxAbility>,
  ability_one: Option<BoxAbility>,
}

impl Input {
  pub fn new() -> Input {
    let mut input = Input {
      left_click_ability: Some(Box::new(Move::new())),
      middle_click_ability: Some(Box::new(Shield::new())),
      right_click_ability: Some(Box::new(DoubleShot::new())),
      ability_one: Some(Box::new(Dash::new())),
    };
    
    if let Some(right_click_ability) = &mut input.right_click_ability {
      right_click_ability.add_passive(Box::new(Shatter::new()));
    }
    
    input
  }
  
  pub fn update(&mut self, ship: &mut BoxEntity, left_stick_position: Vector2<f32>, a_button_pressed: bool, right_trigger_pressed: bool, mouse_pos: Vector2<f32>, left_mouse: bool, middle_mouse: bool, right_mouse: bool, q_pressed: bool, window_size: Vector2<f32>, delta_time: f32) {
    let mut target = mouse_pos;
    let ship_offset = ship.position()-window_size*0.5;
    target += ship_offset;
    
    let hostility = ship.hostility().clone();
    
    if let Some(ability) = &mut self.left_click_ability {
      ability.update(delta_time);
      if left_mouse {
        ability.activate(ship, target, window_size, &hostility);
      }
      
      let dead_zone = 0.01;
      if left_stick_position.magnitude().abs() > dead_zone {
        let x = if left_stick_position.x.abs() > dead_zone { left_stick_position.x } else { 0.0 };
        let y = if left_stick_position.y.abs() > dead_zone { left_stick_position.y } else { 0.0 };
        
        let mut radius = 50.0;
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
    }
    
    ship.set_facing(target);
  }
  
  pub fn return_abilities(&self) -> (Option<BoxAbility>, Option<BoxAbility>, Option<BoxAbility>, 
                                     Option<BoxAbility>, Option<BoxAbility>, Option<BoxAbility>, 
                                     Option<BoxAbility>) {
    (self.left_click_ability.clone(), self.middle_click_ability.clone(), self.right_click_ability.clone(), 
     self.ability_one.clone(), None, None, None)
  }
}


