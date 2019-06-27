
use maat_graphics::DrawCall;

use crate::modules::abilities::{Ability, NoAbility};

use crate::cgmath::Vector2;

pub struct AbilityUi {
  positions: [Vector2<f32>; 7],
}

impl AbilityUi {
  pub fn new() -> AbilityUi {
    let position = Vector2::new(0.0, 0.0);
    AbilityUi {
      positions: [position, position, position, position, position, position, position],
    }
  }
  
  pub fn update(&mut self, window_size: Vector2<f32>) {
    let space_from_edge = 100.0;
    let spacing = 75.0;
    self.positions[0] = Vector2::new(window_size.x-space_from_edge-spacing*2.0, space_from_edge);
    self.positions[1] = Vector2::new(window_size.x-space_from_edge-spacing,     space_from_edge);
    self.positions[2] = Vector2::new(window_size.x-space_from_edge,             space_from_edge);
    self.positions[3] = Vector2::new(space_from_edge,             space_from_edge);
    self.positions[4] = Vector2::new(space_from_edge+spacing,     space_from_edge);
    self.positions[5] = Vector2::new(space_from_edge+spacing*2.0, space_from_edge);
    self.positions[6] = Vector2::new(space_from_edge+spacing*3.0, space_from_edge);
  }
  
  fn draw_ability(&self, some_ability: Option<Box<Ability>>, i: usize, draw_calls: &mut Vec<DrawCall>) {
    if let Some(ability) = some_ability {
      ability.draw(self.positions[i], draw_calls);
    } else {
      let no_ability = NoAbility::new();
      no_ability.draw(self.positions[i], draw_calls);
    }
  }
  
  pub fn draw(&self, l_ability: Option<Box<Ability>>, m_ability: Option<Box<Ability>>, 
                         r_ability: Option<Box<Ability>>, ability_1: Option<Box<Ability>>, 
                         ability_2: Option<Box<Ability>>, ability_3: Option<Box<Ability>>, 
                         ability_4: Option<Box<Ability>>, draw_calls: &mut Vec<DrawCall>) {
    self.draw_ability(l_ability, 0, draw_calls);
    self.draw_ability(m_ability, 1, draw_calls);
    self.draw_ability(r_ability, 2, draw_calls);
    self.draw_ability(ability_1, 3, draw_calls);
    self.draw_ability(ability_2, 4, draw_calls);
    self.draw_ability(ability_3, 5, draw_calls);
    self.draw_ability(ability_4, 6, draw_calls);
    
  }
}
