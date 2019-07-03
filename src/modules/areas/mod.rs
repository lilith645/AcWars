pub use self::solar_system::SolarSystem;
pub use self::astroid_field::AstroidField;

mod solar_system;
mod astroid_field;

use maat_graphics::DrawCall;
use crate::modules::projectiles::{Projectile, BoxProjectile};
use crate::modules::entities::{MutexEntity, FullEntity, Entity, BoxEntity, Brew};
use crate::modules::controllers::AbilitySpamAi;
use crate::modules::abilities::{Ability, SingleShot, ProjectileSpeed, Haste};

use crate::cgmath::Vector2;

use std::sync::Arc;

pub type BoxArea = Box<Area>;

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
        FullEntity::new(Box::new(AbilitySpamAi::new().with_ability(e1_single_shot).with_ability(e1_haste)), 
                        Box::new(Brew::new().as_hostile().with_position(position+Vector2::new(position.x-100.0, position.y)))),
        FullEntity::new(Box::new(AbilitySpamAi::new().with_ability(e2_ability)), 
                        Box::new(Brew::new().as_hostile().with_position(position+Vector2::new(position.x+300.0, position.y-300.0)))),
        FullEntity::new(Box::new(AbilitySpamAi::new().with_ability(e3_ability.clone())), 
                        Box::new(Brew::new().as_hostile().with_position(position+Vector2::new(position.x-840.0, position.y-1500.0)))),
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
  
  fn entities(&mut self) -> Vec<MutexEntity> {
    let mut arc_entities = Vec::new();
    for full_entity in &mut self.mut_data().entities {
      arc_entities.push(Arc::clone(&full_entity.entity))
    }
    
    arc_entities
  }
  /*
  fn collide_with(&mut self, projectile: &mut Box<Projectile>) {
    for object in &mut self.mut_data().entities {
      let mut entity = object.entity.lock().unwrap();
      if entity.should_exist() {
        if projectile.can_hit(entity.hostility()) {
          projectile.collide_with(&mut *entity);
        }
      }
    }
  }*/
  
  // TODO: Do for Spatial Hash
  /*
  fn internal_collisions(&mut self, ship: &mut Box<Entity>) {
    for i in 0..self.data().entities.len() {
      for j in 0..self.data().entities.len() {
        if i == j {
          continue;
        }
        
        let mut entity_i = self.data().entities[i].entity.lock().unwrap();
        let mut entity_j = self.data().entities[j].entity.lock().unwrap();
        entity_i.collide_with(&mut *entity_j);
        //self.mut_data().entities[j].entity = entity;
      }
    }
    
    for i in 0..self.data().entities.len() {
      let mut entity = self.mut_data().entities[i].entity.lock().unwrap();
      entity.collide_with(ship);
    }
  }*/
  
  fn update(&mut self, ship: &mut BoxEntity, window_size: Vector2<f32>, delta_time: f32) -> Vec<BoxProjectile> {
    self.update_area(delta_time);
    
    let mut new_projectiles: Vec<BoxProjectile> = Vec::new();
    
    // entities
    let ship_pos = ship.position();
    let area_pos = self.data().position;
    let area_size = self.data().size;
    for object in &mut self.mut_data().entities {
      let mut entity = object.entity.lock().unwrap();
      object.ai.update(&mut *entity, ship_pos, area_pos, area_size, window_size, delta_time);
      
      let mut offset = 0;
      for i in 0..object.buffs.len() {
        if offset > i {
          break;
        }
        object.buffs[i-offset].update(&mut *entity, delta_time);
        if !object.buffs[i-offset].should_exist() {
          object.buffs[i-offset].unapply_buff(&mut *entity);
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
      
      let mut should_exist = true;
      
      let mut object_buffs = Vec::new();
      {
        let mut entity = self.data().entities[i-offset].entity.lock().unwrap();
        should_exist = entity.should_exist();
        let (temp_object_buffs, object_proj) = entity.update(delta_time);
        object_buffs = temp_object_buffs;
        
        for projectile in object_proj {
          new_projectiles.push(projectile);
        }
      }
      
      for buff in object_buffs {
        {
          let mut entity = self.data().entities[i-offset].entity.lock().unwrap();
          buff.apply_buff(&mut *entity);
        }
        self.mut_data().entities[i-offset].buffs.push(buff);
      }
      
      if !should_exist {
        self.mut_data().entities.remove(i-offset);
        offset += 1;
      }
    }
    
    new_projectiles
  }
  
  fn draw_ship_ui(&self, draw_calls: &mut Vec<DrawCall>) {
    for object in &self.data().entities {
      let entity = object.entity.lock().unwrap();
      entity.draw_ship_ui(draw_calls);
    }
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    for object in &self.data().entities {
      let entity = object.entity.lock().unwrap();
      entity.draw(draw_calls);
    }
  }
  
  fn draw_collision_circles(&self, draw_calls: &mut Vec<DrawCall>) {
    for object in &self.data().entities {
      let entity = object.entity.lock().unwrap();
      entity.draw_collision_circles(draw_calls);
    }
  }
}
