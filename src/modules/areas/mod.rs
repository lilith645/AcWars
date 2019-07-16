pub use self::solar_system::SolarSystem;
pub use self::astroid_field::AstroidField;
pub use self::benchmark::BenchmarkArea;

mod solar_system;
mod astroid_field;
mod benchmark;

use maat_graphics::DrawCall;
use crate::modules::projectiles::{BoxProjectile};
use crate::modules::entities::{MutexEntity, FullEntity, BoxEntity};

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
    AreaData {
      position,
      size,
      entities: Vec::new(),
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
  
  fn entities(&self) -> Vec<MutexEntity> {
    let mut arc_entities = Vec::new();
    for full_entity in &self.data().entities {
      arc_entities.push(Arc::clone(&full_entity.entity))
    }
    
    arc_entities
  }
  /*
  fn collide_with(&mut self, projectile: &mut Box<Projectile>) {
    for object in &mut self.mut_data().entities {
      let mut entity = object.entity.lock();
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
        
        let mut entity_i = self.data().entities[i].entity.lock();
        let mut entity_j = self.data().entities[j].entity.lock();
        entity_i.collide_with(&mut *entity_j);
        //self.mut_data().entities[j].entity = entity;
      }
    }
    
    for i in 0..self.data().entities.len() {
      let mut entity = self.mut_data().entities[i].entity.lock();
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
      let mut entity = object.entity.lock();
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
      
      let should_exist;
      
      let mut object_buffs;
      {
        let mut entity = self.data().entities[i-offset].entity.lock();
        should_exist = entity.should_exist();
        let (temp_object_buffs, object_proj) = entity.update(delta_time);
        object_buffs = temp_object_buffs;
        
        for projectile in object_proj {
          new_projectiles.push(projectile);
        }
      }
      
      for buff in object_buffs {
        {
          let mut entity = self.data().entities[i-offset].entity.lock();
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
      let entity = object.entity.lock();
      entity.draw_ship_ui(draw_calls);
    }
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    for object in &self.data().entities {
      let entity = object.entity.lock();
      entity.draw(draw_calls);
    }
  }
  
  fn draw_collision_circles(&self, draw_calls: &mut Vec<DrawCall>) {
    for object in &self.data().entities {
      let entity = object.entity.lock();
      entity.draw_collision_circles(draw_calls);
    }
  }
}
