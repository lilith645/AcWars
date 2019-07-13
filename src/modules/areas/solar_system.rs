use crate::modules::areas::{Area, AreaData};
use crate::modules::entities::{FullEntity, Sun, Astroid, Brew, Wall};
use crate::modules::controllers::{IdleAi, AbilitySpamAi};
use crate::modules::abilities::{Ability, SingleShot, SunDamage, Haste, ProjectileSpeed};

use crate::cgmath::Vector2;



#[derive(Clone)]
pub struct SolarSystem {
  data: AreaData,
}

impl SolarSystem {
  pub fn new(position: Vector2<f32>, size: Vector2<f32>) -> SolarSystem {
    let sun = FullEntity::new(Box::new(IdleAi::new().with_ability(Box::new(SunDamage::new()))), 
                              Box::new(Sun::new(Vector2::new(840.0, 1500.0)).as_neutral()));
    let wall = FullEntity::new(Box::new(IdleAi::new()), 
                              Box::new(Wall::new(Vector2::new(0.0, 1500.0)).as_neutral()));
    let astroid = FullEntity::new(Box::new(AbilitySpamAi::new()), 
                                  Box::new(Astroid::new(Vector2::new(-500.0, -1500.0), 
                                                        Vector2::new(100.0, 100.0)).as_hostile()));
    
    let e1_single_shot = Box::new(SingleShot::new());
    let e1_haste = Box::new(Haste::new());
    let mut e2_ability = Box::new(SingleShot::new());
    let e3_ability = Box::new(SingleShot::new());
    e2_ability.add_passive(Box::new(ProjectileSpeed::new()));
    
     let e1 =   FullEntity::new(Box::new(AbilitySpamAi::new().with_ability(e1_single_shot).with_ability(e1_haste)), 
                        Box::new(Brew::new(position).as_hostile().with_position(position+Vector2::new(position.x-100.0, position.y))));
     let e2 = FullEntity::new(Box::new(AbilitySpamAi::new().with_ability(e2_ability)), 
                        Box::new(Brew::new(position).as_hostile().with_position(position+Vector2::new(position.x+300.0, position.y-300.0))));
    let e3 = FullEntity::new(Box::new(AbilitySpamAi::new().with_ability(e3_ability.clone())), 
                        Box::new(Brew::new(position).as_hostile().with_position(position+Vector2::new(position.x-840.0, position.y-1500.0))));
    SolarSystem {
      data: AreaData::new(position, size)
                      .with_entity(sun)
                      .with_entity(astroid)
                      .with_entity(e1)
                      .with_entity(e2)
                      .with_entity(e3)
                      .with_entity(wall),
    }
  }
}

impl Area for SolarSystem {
  fn data(&self) -> &AreaData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut AreaData {
    &mut self.data
  }
  
  fn update_area(&mut self, _delta_time: f32) {
    
  }
}
