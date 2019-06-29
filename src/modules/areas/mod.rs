pub use self::solar_system::SolarSystem;

mod solar_system;

use maat_graphics::DrawCall;
use crate::modules::projectiles::Projectile;
use crate::modules::entities::{FullEntity, Entity, Brew};
use crate::modules::controllers::AbilitySpamAi;

use crate::cgmath::Vector2;

#[derive(Clone)]
pub struct AreaData {
  position: Vector2<f32>,
  size: Vector2<f32>,
  hostiles: Vec<FullEntity>,
  // events
}

impl AreaData {
  pub fn new(position: Vector2<f32>, size: Vector2<f32>) -> AreaData {
    AreaData {
      position,
      size,
      hostiles: vec!(
        FullEntity { 
          ai: Box::new(AbilitySpamAi::new()), 
          entity: Box::new(Brew::new().as_hostile().with_position(Vector2::new(640.0, 1500.0))),
          buffs: Vec::new(),
        },
        FullEntity { 
          ai: Box::new(AbilitySpamAi::new()), 
          entity: Box::new(Brew::new().as_hostile().with_position(Vector2::new(740.0, 1500.0))),
          buffs: Vec::new(),
        },
        FullEntity { 
          ai: Box::new(AbilitySpamAi::new()), 
          entity: Box::new(Brew::new().as_hostile().with_position(Vector2::new(840.0, 1500.0))),
          buffs: Vec::new(),
        },
      ),
    }
  }
  
  pub fn with_entity(mut self, entity: FullEntity) -> AreaData {
    self.hostiles.push(entity);
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
    for hostile in &mut self.mut_data().hostiles {
      if hostile.entity.should_exist() {
        projectile.collide_with(&mut hostile.entity);
      }
    }
  }
  
  fn update(&mut self, ship: &mut Box<Entity>, window_size: Vector2<f32>, delta_time: f32) -> Vec<Box<Projectile>> {
    self.update_area(delta_time);
    
    let mut new_projectiles: Vec<Box<Projectile>> = Vec::new();
    
    // hostiles
    let ship_pos = ship.position();
    for hostile in &mut self.mut_data().hostiles {
      hostile.ai.update(&mut hostile.entity, ship_pos, window_size, delta_time);
      
      let mut offset = 0;
      for i in 0..hostile.buffs.len() {
        if offset > i {
          break;
        }
        hostile.buffs[i-offset].update(&mut hostile.entity, delta_time);
        if !hostile.buffs[i-offset].should_exist() {
          hostile.buffs[i-offset].unapply_buff(&mut hostile.entity);
          hostile.buffs.remove(i-offset);
          offset += 1;
        }
      }
    }
    
    let mut offset = 0;
    for i in 0..self.data().hostiles.len() {
      if i < offset {
        break;
      }
      
      let (hostile_buffs, hostile_proj) = self.mut_data().hostiles[i-offset].entity.update(delta_time);
      for buff in hostile_buffs {
        buff.apply_buff(&mut self.mut_data().hostiles[i-offset].entity);
        self.mut_data().hostiles[i-offset].buffs.push(buff);
      }
      for projectile in hostile_proj {
        new_projectiles.push(projectile);
      }
      
      if !self.data().hostiles[i-offset].entity.should_exist() {
        self.mut_data().hostiles.remove(i-offset);
        offset += 1;
      }
    }
    
    new_projectiles
  }
  
  fn draw_ship_ui(&self, draw_calls: &mut Vec<DrawCall>) {
    for hostile in &self.data().hostiles {
      hostile.entity.draw_ship_ui(draw_calls);
    }
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    for hostile in &self.data().hostiles {
      hostile.entity.draw(draw_calls);
    }
  }
}
