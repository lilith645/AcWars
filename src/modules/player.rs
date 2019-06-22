use crate::modules::Ship;
use crate::modules::abilities::{Ability, SingleShot, Move};

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
    Input {
      left_click_ability: Some(Box::new(SingleShot::new())),
      middle_click_ability: None,
      right_click_ability: Some(Box::new(Move::new())),
    }
  }
  
  pub fn update(&mut self, ship: &mut Ship, mouse_pos: Vector2<f32>, left_mouse: bool, middle_mouse: bool, right_mouse: bool, window_size: Vector2<f32>, delta_time: f32) {
    if let Some(ability) = &mut self.left_click_ability {
      ability.update(delta_time);
      if left_mouse {
        ability.activate(ship, mouse_pos, window_size);
      }
    }
    
    if let Some(ability) = &mut self.right_click_ability {
      ability.update(delta_time);
      if right_mouse {
        ability.activate(ship, mouse_pos, window_size);
      }
    }
  }
}
