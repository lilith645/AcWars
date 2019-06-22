use maat_graphics::DrawCall;

use crate::modules::projectiles::Projectile;
use crate::modules::abilities::{Ability, AbilityType};

use cgmath::{Vector2, InnerSpace};

// Bulbz - Basic Utility Laser Battleship zero

#[derive(Clone)]
pub struct Ship {
  position: Vector2<f32>,
  rotation: f32,
  size: Vector2<f32>,
  texture: String,
  velocity: Vector2<f32>,
  max_velocity: f32,
  acceleration: Vector2<f32>,
  inertia: f32,
  projectiles: Vec<Box<Projectile>>,
}

impl Ship {
  pub fn new() -> Ship {
    Ship {
      position: Vector2::new(640.0, 520.0),
      rotation: 0.0,
      size: Vector2::new(90.0, 90.0),
      texture: "Bulbz".to_string(),
      velocity: Vector2::new(0.0, 0.0),
      max_velocity: 300.0,
      acceleration: Vector2::new(0.0, 0.0),
      inertia: 0.33,
      projectiles: Vec::new(),
    }
  }
  
  pub fn position(&self) -> Vector2<f32> {
    self.position
  }
  
  pub fn size(&self) -> Vector2<f32> {
    self.size
  }
  
  pub fn apply_velocity_in_direction(&mut self, direction: Vector2<f32>) {
    self.acceleration = direction.normalize();
  }
  
  pub fn fire_projectile(&mut self, projectile: Box<Projectile>) {
    self.projectiles.push(projectile);
  }
  
  pub fn update(&mut self, delta_time: f32) -> Vec<Box<Projectile>> {
    self.position += self.velocity*delta_time;
    self.velocity -= self.velocity*(1.0-self.inertia)*delta_time;
    self.velocity += self.acceleration*self.max_velocity*(1.0-self.inertia)*delta_time;
    
    if self.velocity.magnitude() > self.max_velocity {
      self.velocity = self.velocity.normalize()*self.max_velocity;
    }
    
    self.acceleration = Vector2::new(0.0, 0.0);
    
    let mut projectiles = self.projectiles.clone();
    self.projectiles.clear();
    
    projectiles
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::draw_textured(self.position, self.size, self.rotation, self.texture.to_string()));
  }
}
