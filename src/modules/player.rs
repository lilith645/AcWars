use maat_graphics::DrawCall;

use crate::modules::entities::Entity;
use crate::modules::abilities::{Ability, Laser, SingleShot, DoubleShot, Move, ProjectileSpeed, 
                                Shatter, Shield, Dash};

use crate::cgmath::Vector2;

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
  left_click_ability: Option<Box<Ability>>,
  middle_click_ability: Option<Box<Ability>>,
  right_click_ability: Option<Box<Ability>>,
  ability_one: Option<Box<Ability>>,
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
  
  pub fn update(&mut self, ship: &mut Box<Entity>, mouse_pos: Vector2<f32>, left_mouse: bool, middle_mouse: bool, right_mouse: bool, q_pressed: bool, window_size: Vector2<f32>, delta_time: f32) {
    let mut target = mouse_pos;
    let ship_offset = ship.position()-window_size*0.5;
    target += ship_offset;
    
    if let Some(ability) = &mut self.left_click_ability {
      ability.update(delta_time);
      if left_mouse {
        ability.activate(ship, target, window_size);
      }
    }
    
    if let Some(ability) = &mut self.middle_click_ability {
      ability.update(delta_time);
      if middle_mouse {
        ability.activate(ship, target, window_size);
      }
    }
    
    if let Some(ability) = &mut self.right_click_ability {
      ability.update(delta_time);
      if right_mouse {
        ability.activate(ship, target, window_size);
      }
    }
    
    if let Some(ability) = &mut self.ability_one {
      ability.update(delta_time);
      if q_pressed {
        ability.activate(ship, target, window_size);
      }
    }
    
    ship.set_facing(target);
  }
  
  pub fn return_abilities(&self) -> (Option<Box<Ability>>, Option<Box<Ability>>, Option<Box<Ability>>, 
                                     Option<Box<Ability>>, Option<Box<Ability>>, Option<Box<Ability>>, 
                                     Option<Box<Ability>>) {
    (self.left_click_ability.clone(), self.middle_click_ability.clone(), self.right_click_ability.clone(), 
     self.ability_one.clone(), None, None, None)
  }
}


