pub use self::solar_system::SolarSystem;

mod solar_system;

use maat_graphics::DrawCall;
use crate::modules::projectiles::Projectile;
use crate::modules::entities::{FullEntity, Entity, Brew};
use crate::modules::controllers::AbilitySpamAi;
use crate::modules::abilities::{Ability, SingleShot, ProjectileSpeed, Haste};

use crate::cgmath::Vector2;

#[derive(Clone)]
pub struct AreaData {
  position: Vector2<f32>,
  size: Vector2<f32>,
  entities: Vec<FullEntity>,
  // events
}

impl AreaData {
  pub fn new(position: Vector2<f32>, size: Vector2<f32>) -> AreaData {
    let e1_single_shot = Box::new(SingleShot::new());
    let e1_haste = Box::new(Haste::new());
    let mut e2_ability = Box::new(SingleShot::new());
    let e3_ability = Box::new(SingleShot::new());
    
    e2_ability.add_passive(Box::new(ProjectileSpeed::new()));
    
    AreaData {
      position,
      size,
      entities: vec!(
        FullEntity { 
          ai: Box::new(AbilitySpamAi::new().with_ability(e1_single_shot).with_ability(e1_haste)), 
          entity: Box::new(Brew::new().as_hostile().with_position(position+Vector2::new(-100.0, 0.0))),
          buffs: Vec::new(),
        },
        FullEntity { 
          ai: Box::new(AbilitySpamAi::new().with_ability(e2_ability)), 
          entity: Box::new(Brew::new().as_hostile().with_position(position+Vector2::new(300.0, -300.0))),
          buffs: Vec::new(),
        },
        FullEntity { 
          ai: Box::new(AbilitySpamAi::new().with_ability(e3_ability)), 
          entity: Box::new(Brew::new().as_hostile().with_position(position+Vector2::new(-840.0, -1500.0))),
          buffs: Vec::new(),
        },
      ),
    }
  }
  
  pub fn with_entity(mut self, entity: FullEntity) -> AreaData {
    self.entities.push(entity);
    self
  }
}

pub trait AreaClone {
  fn clone_area(&self) -> Box<Area>;
}

impl<T: 'static + Area + Clone> AreaClone for T {
  fn clone_area(&self) -> Box<Area> {
    Box::new(self.clone())
  }
}

impl Clone for Box<Area> {
  fn clone(&self) -> Box<Area> {
    self.clone_area()
  }
}

pub trait Area: AreaClone {
  fn data(&self) -> &AreaData;
  fn mut_data(&mut self) -> &mut AreaData;
  
  fn update_area(&mut self, delta_time: f32);
  
  fn collide_with(&mut self, projectile: &mut Box<Projectile>) {
    for object in &mut self.mut_data().entities {
      if object.entity.should_exist() {
        if projectile.can_hit(object.entity.hostility()) {
          projectile.collide_with(&mut object.entity);
        }
      }
    }
  }
  
  fn update(&mut self, ship: &mut Box<Entity>, window_size: Vector2<f32>, delta_time: f32) -> Vec<Box<Projectile>> {
    self.update_area(delta_time);
    
    let mut new_projectiles: Vec<Box<Projectile>> = Vec::new();
    
    // entities
    let ship_pos = ship.position();
    for object in &mut self.mut_data().entities {
      object.ai.update(&mut object.entity, ship_pos, window_size, delta_time);
      
      let mut offset = 0;
      for i in 0..object.buffs.len() {
        if offset > i {
          break;
        }
        object.buffs[i-offset].update(&mut object.entity, delta_time);
        if !object.buffs[i-offset].should_exist() {
          object.buffs[i-offset].unapply_buff(&mut object.entity);
          object.buffs.remove(i-offset);
          offset += 1;
        }
      }
    }
    
    let mut offset = 0;
    for i in 0..self.data().entities.len() {
      if i < offset {
        break;
      }
      
      let (object_buffs, object_proj) = self.mut_data().entities[i-offset].entity.update(delta_time);
      for buff in object_buffs {
        buff.apply_buff(&mut self.mut_data().entities[i-offset].entity);
        self.mut_data().entities[i-offset].buffs.push(buff);
      }
      for projectile in object_proj {
        new_projectiles.push(projectile);
      }
      
      if !self.data().entities[i-offset].entity.should_exist() {
        self.mut_data().entities.remove(i-offset);
        offset += 1;
      }
    }
    
    new_projectiles
  }
  
  fn draw_ship_ui(&self, draw_calls: &mut Vec<DrawCall>) {
    for object in &self.data().entities {
      object.entity.draw_ship_ui(draw_calls);
    }
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    for object in &self.data().entities {
      object.entity.draw(draw_calls);
    }
  }
  
  fn draw_collision_circles(&self, draw_calls: &mut Vec<DrawCall>) {
     for object in &self.data().entities {
      object.entity.draw_collision_circles(draw_calls);
    }
  }
}
