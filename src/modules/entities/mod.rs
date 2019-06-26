pub use self::ship::Ship;
pub use self::brew::Brew;

mod ship;
mod brew;

use maat_graphics::DrawCall;

use crate::modules::projectiles::Projectile;
use crate::modules::buffs::Buff;

use std::f32::consts::PI;

use crate::cgmath::{Vector2, Vector3, Vector4, InnerSpace};

#[derive(Clone)]
pub struct EntityData {
  position: Vector2<f32>,
  rotation: f32,
  size: Vector2<f32>,
  texture: String,
  velocity: Vector2<f32>,
  max_velocity: f32,
  acceleration: Vector2<f32>,
  inertia: f32,
  health: f32,
  shield: f32,
  initial_health: f32,
  projectiles: Vec<Box<Projectile>>,
  buffs: Vec<Box<Buff>>,
  hostile: bool,
  should_exist: bool,
}

impl EntityData {
  pub fn new_empty() -> EntityData {
     EntityData {
      position: Vector2::new(0.0, 0.0),
      rotation: 0.0,
      size: Vector2::new(1.0, 1.0),
      texture: "".to_string(),
      velocity: Vector2::new(0.0, 0.0),
      max_velocity: 1.0,
      acceleration: Vector2::new(0.0, 0.0),
      inertia: 0.33,
      health: 100.0,
      shield: 0.0,
      initial_health: 100.0,
      projectiles: Vec::new(),
      buffs: Vec::new(),
      hostile: false,
      should_exist: true,
    }
  }
  
  pub fn new(position: Vector2<f32>, size: Vector2<f32>, texture: String) -> EntityData {
     EntityData {
      position,
      rotation: 0.0,
      size,
      texture: texture.to_string(),
      velocity: Vector2::new(0.0, 0.0),
      max_velocity: 500.0,
      acceleration: Vector2::new(0.0, 0.0),
      inertia: 0.33,
      health: 100.0,
      shield: 0.0,
      initial_health: 100.0,
      projectiles: Vec::new(),
      buffs: Vec::new(),
      hostile: false,
      should_exist: true,
    }
  }
  
  pub fn with_rotation(mut self, rot: f32) -> EntityData {
    self.rotation = rot;
    self
  }
  
  pub fn with_velocity(mut self, vel: Vector2<f32>) -> EntityData {
    self.velocity = vel;
    self
  }
  
  pub fn with_max_velocity(mut self, max_vel: f32) -> EntityData {
    self.max_velocity = max_vel;
    self
  }
  
  pub fn with_inertia(mut self, inertia: f32) -> EntityData {
    self.inertia = inertia;
    self
  }
  
  pub fn with_health(mut self, health: f32) -> EntityData {
    self.health = health;
    self.initial_health = health;
    self
  }
  
  pub fn as_hostile(mut self) -> EntityData {
    self.hostile = true;
    self
  }
}

pub trait EntityClone {
  fn clone_entity(&self) -> Box<Entity>;
}

impl<T: 'static + Entity + Clone> EntityClone for T {
  fn clone_entity(&self) -> Box<Entity> {
    Box::new(self.clone())
  }
}

impl Clone for Box<Entity> {
  fn clone(&self) -> Box<Entity> {
    self.clone_entity()
  }
}

pub trait Entity: EntityClone {
  fn data(&self) -> &EntityData;
  fn mut_data(&mut self) -> &mut EntityData;
  
  fn collision_information(&self) -> Vec<(Vector2<f32>, f32)>;
  
  fn update(&mut self, delta_time: f32) -> (Vec<Box<Buff>>, Vec<Box<Projectile>>) {
    self.physics(delta_time);
    
    (self.return_buffs(), self.return_projectiles())
  }
  
  fn position(&self) -> Vector2<f32> {
    self.data().position
  }
  
  fn size(&self) -> Vector2<f32> {
    self.data().size
  }
  
  fn rotation(&self) -> f32 {
    self.data().rotation
  }
  
  fn velocity(&self) -> Vector2<f32> {
    self.data().velocity
  }
  
  fn max_velocity(&self) -> f32 {
    self.data().max_velocity
  }
  
  fn should_exist(&self) -> bool {
    self.data().should_exist
  }
  
  fn collision_circles(&self) -> Vec<Vector3<f32>> {
    let information = self.collision_information();
    
    let mut entity_circles = Vec::new();
    
    let entity_position = self.data().position;
    
    for (offset, radius) in information {
      let entity_radius = radius;
      let entity_circle = (entity_position+offset).extend(entity_radius);
      entity_circles.push(entity_circle);
    }
    
    entity_circles
  }
  
  fn gain_shield(&mut self, shield_value: f32) {
    self.mut_data().shield += shield_value;
  }
  
  fn hit(&mut self, damage: f32) {
    if self.data().shield > 0.0 {
      if self.data().shield < damage {
        self.mut_data().shield -= damage;
        self.mut_data().health += self.data().shield;
        self.mut_data().shield = 0.0;
      } else {
        self.mut_data().shield -= damage;
      }
    } else {
      self.mut_data().health -= damage;
    }
    
    if self.data().health <= 0.0 {
      self.mut_data().should_exist = false;
      self.mut_data().health = 0.0;
    }
  }
  
  fn set_velocity(&mut self, vel: Vector2<f32>) {
    self.mut_data().velocity = vel;
  }
  
  fn set_max_velocty(&mut self, max_vel: f32) {
    self.mut_data().max_velocity = max_vel;
  }
  
  fn set_rotation(&mut self, rot: f32) {
    self.mut_data().rotation = rot;
  }
  
  fn set_facing(&mut self, target: Vector2<f32>) {
    let direction = target-self.data().position;
    let rotation = direction.x.atan2(direction.y);
    
    let rot_degree = 360.0-(rotation*180.0)/PI;
    self.mut_data().rotation = rot_degree;
  }
  
  fn set_velocity_magnitude(&mut self, vel: f32) {
    let vel_dir = self.data().velocity.normalize();
    self.mut_data().velocity = vel_dir*vel;
  }
  
  fn apply_acceleration_in_direction(&mut self, direction: Vector2<f32>) {
    self.mut_data().acceleration = direction.normalize();
  }
  
  fn fire_projectile(&mut self, mut projectile: Box<Projectile>) {
    if self.data().hostile {
      projectile.make_hostile(); 
    }
    
    self.mut_data().projectiles.push(projectile);
  }
  
  fn activate_buff(&mut self, buff: Box<Buff>) {
    self.mut_data().buffs.push(buff);
  }
  
  fn physics(&mut self, delta_time: f32) {
    let velocity = self.data().velocity;
    let max_velocity = self.data().max_velocity;
    let acceleration = self.data().acceleration;
    let inertia = self.data().inertia;
    self.mut_data().position += velocity*delta_time;
    self.mut_data().velocity -= velocity*(1.0-inertia)*delta_time;
    self.mut_data().velocity += acceleration*max_velocity*(1.0-inertia)*delta_time;
    
    if self.data().velocity.magnitude() > self.data().max_velocity {
      self.mut_data().velocity = velocity.normalize()*max_velocity;
    }
    
    self.mut_data().acceleration = Vector2::new(0.0, 0.0);
  }
  
  fn return_projectiles(&mut self) -> Vec<Box<Projectile>> {
    let projectiles = self.data().projectiles.clone();
    self.mut_data().projectiles.clear();
    
    projectiles
  }
  
  fn return_buffs(&mut self) -> Vec<Box<Buff>> {
    let buffs = self.data().buffs.clone();
    self.mut_data().buffs.clear();
    
    buffs
  }
  
  fn draw_ship_ui(&self, draw_calls: &mut Vec<DrawCall>) {
    let ship_size = self.data().size;
    
    let position = self.data().position + Vector2::new(0.0, ship_size.y*0.5);
    let size = Vector2::new(40.0*(self.data().health / self.data().initial_health), 10.0);
    let colour = Vector4::new(0.0, 1.0, 0.0, 0.5);
    draw_calls.push(DrawCall::draw_coloured(position, size, colour, 0.0));
    
    let size = Vector2::new(40.0*(self.data().shield / self.data().initial_health), 15.0);
    let colour = Vector4::new(0.0, 0.0, 1.0, 0.5);
    draw_calls.push(DrawCall::draw_coloured(position, size, colour, 0.0));
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::add_instanced_sprite_sheet(self.data().position, self.data().size, 
                                                         self.data().rotation, 
                                                         self.data().texture.to_string(), 
                                                         Vector3::new(0,0, 1)));
  }
  
  fn draw_collision_circles(&self, draw_calls: &mut Vec<DrawCall>) {
    let circles = self.collision_circles();
    let colour = if self.data().hostile {
      Vector4::new(1.0, 0.0, 0.0, 1.0)
    } else {
      Vector4::new(0.0, 0.0, 1.0, 1.0)
    };
    for circle in &circles {
      draw_calls.push(DrawCall::draw_coloured(circle.xy(), Vector2::new(circle.z, circle.z), colour, 0.0));
    }
  }
}
