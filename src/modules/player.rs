use crate::modules::entities::Entity;
use crate::modules::abilities::{Ability, Laser, SingleShot, DoubleShot, Move, ProjectileSpeed};

use cgmath::Vector2;

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
}

impl Input {
  pub fn new() -> Input {
    let mut input = Input {
      left_click_ability: Some(Box::new(SingleShot::new())),
      middle_click_ability: Some(Box::new(DoubleShot::new())),
      right_click_ability: Some(Box::new(Move::new())),
    };
    
    if let Some(middle_click_ability) = &mut input.middle_click_ability {
      middle_click_ability.add_passive(Box::new(ProjectileSpeed::new()));
    }
    
    input
  }
  
  pub fn update(&mut self, ship: &mut Box<Entity>, mouse_pos: Vector2<f32>, left_mouse: bool, middle_mouse: bool, right_mouse: bool, window_size: Vector2<f32>, delta_time: f32) {
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
    
    ship.set_facing(target);
  }
}
