use crate::modules::areas::{Area, AreaData};
use crate::modules::entities::{FullEntity, Brew};
use crate::modules::controllers::{IdleAi, AbilitySpamAi};
use crate::modules::abilities::{Ability, SingleShot, Shatter, SunDamage};

use crate::cgmath::Vector2;

use std::sync::Arc;

#[derive(Clone)]
pub struct BenchmarkArea {
  data: AreaData,
}

impl BenchmarkArea {
  pub fn new(position: Vector2<f32>, size: Vector2<f32>, iterations: usize) -> BenchmarkArea {
    let mut area = AreaData::new(position, size);
    
    for i in 0..iterations {
      for j in 0..iterations {
        let position = position-size*0.5 + Vector2::new(size.x*(1.0/iterations as f32)*i as f32,
                                                         size.y*(1.0/iterations as f32)*j as f32);
        let brew = FullEntity::new(Box::new(AbilitySpamAi::new().with_ability(Box::new(SingleShot::new()))), 
                                   Box::new(Brew::new(position).as_hostile().with_health(15000000000000.0)));
        area = area.with_entity(brew);
      }
    }
    
    BenchmarkArea {
      data: area,
    }
  }
}

impl Area for BenchmarkArea {
  fn data(&self) -> &AreaData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut AreaData {
    &mut self.data
  }
  
  fn update_area(&mut self, delta_time: f32) {
    
  }
}
