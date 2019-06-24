pub use ftpl::Ftpl;
pub use gob::Gob;
pub use laser_beam::LaserBeam;

mod gob;
mod ftpl;
mod laser_beam;

use maat_graphics::DrawCall;
use maat_graphics::math;

use crate::modules::Animation;
use crate::modules::entities::Entity;

use cgmath::{Vector2, Vector3, Vector4};

#[derive(Clone)]
pub struct ProjectileData {
  position: Vector2<f32>,
  rotation: f32,
  size: Vector2<f32>,
  texture: String,
  velocity: Vector2<f32>,
  acceleration: Vector2<f32>,
  animation: Animation,
  damage: f32,
  hostile: bool,
  lifetime_left: f32,
  should_exist: bool,
}

impl ProjectileData {
  pub fn new_empty() -> ProjectileData {
    ProjectileData {
      position: Vector2::new(0.0, 0.0),
      rotation: 0.0,
      size: Vector2::new(1.0, 1.0),
      texture: "".to_string(),
      velocity: Vector2::new(0.0, 0.0),
      acceleration: Vector2::new(0.0, 0.0),
      animation: Animation::new(1, 1.0),
      damage: 1.0,
      hostile: false,
      lifetime_left: 5.0,
      should_exist: true,
    }
  }
  
  pub fn new(position: Vector2<f32>, size: Vector2<f32>, texture: String, sprite_rows: i32, animation_timer: f32) -> ProjectileData {
    ProjectileData {
      position,
      rotation: 0.0,
      size,
      texture: texture.to_string(),
      velocity: Vector2::new(0.0, 0.0),
      acceleration: Vector2::new(0.0, 0.0),
      animation: Animation::new(sprite_rows, animation_timer),
      damage: 1.0,
      hostile: false,
      lifetime_left: 5.0,
      should_exist: true,
    }
  }
  
  pub fn with_rotation(mut self, rotation: f32) -> ProjectileData {
    self.rotation = rotation;
    self
  }
  
  pub fn with_velocity(mut self, vel: Vector2<f32>) -> ProjectileData {
    self.velocity = vel;
    self
  }
  
  pub fn with_damage(mut self, dmg: f32) -> ProjectileData {
    self.damage = dmg;
    self
  }
  
  pub fn with_total_frames(mut self, total_frames: i32) -> ProjectileData {
    self.animation = self.animation.with_total_frames(total_frames);
    self
  }
  
  pub fn with_acceleration(mut self, acc: Vector2<f32>) -> ProjectileData {
    self.acceleration = acc;
    self
  }
  
  pub fn animate_backwards(mut self) -> ProjectileData {
    self.animation = self.animation.animate_backwards();
    self
  }
  
  pub fn animate_forwards_then_backwards(mut self) -> ProjectileData {
    self.animation = self.animation.animate_forwards_then_backwards();
    self
  }
  
  pub fn animate_backwards_then_forwards(mut self) -> ProjectileData {
    self.animation = self.animation.animate_backwards_then_forwards();
    self
  }
}

pub trait ProjectileClone {
  fn clone_weapon(&self) -> Box<Projectile>;
}

impl<T: 'static + Projectile + Clone> ProjectileClone for T {
  fn clone_weapon(&self) -> Box<Projectile> {
    Box::new(self.clone())
  }
}

impl Clone for Box<Projectile> {
  fn clone(&self) -> Box<Projectile> {
    self.clone_weapon()
  }
}

pub trait Projectile: ProjectileClone {
  fn data(&self) -> &ProjectileData;
  fn mut_data(&mut self) -> &mut ProjectileData;
  
  // Vec2<offset>, radius
  fn collision_information(&self) -> Vec<(Vector2<f32>, f32)>;
  
  fn update(&mut self, delta_time: f32);
  
  fn should_exist(&self) -> bool {
    self.data().should_exist
  }
  
  fn hostile(&self) -> bool {
    self.data().hostile
  }
  
  fn collision_circles(&self) -> Vec<Vector3<f32>> {
    let information = self.collision_information();
    
    let mut projectile_circles = Vec::new();
    
    let projectile_position = self.data().position;
    
    for (offset, radius) in information {
      let projectile_radius = radius;
      let projectile_circle = (projectile_position+offset).extend(projectile_radius);
      projectile_circles.push(projectile_circle);
    }
    
    projectile_circles
  }
  
  fn multiply_velocity(&mut self, factor: f32) {
    self.mut_data().velocity *= factor;
  }
  
  fn make_hostile(&mut self) {
    self.mut_data().hostile = true;
  }
  
  fn lifetime_decay(&mut self, delta_time: f32) {
    self.mut_data().lifetime_left -= delta_time;
    if self.data().lifetime_left <= 0.0 {
      self.mut_data().should_exist = false;
    }
  }
  
  fn collide_with(&mut self, entity: &mut Box<Entity>) {
    let entity_circles = entity.collision_circles();
    let projectile_circles = self.collision_circles();
    
    let mut collided = false;
    
    for e_circle in entity_circles {
      for p_circle in &projectile_circles {
        if math::circle_collision(e_circle, *p_circle) {
          entity.hit(self.data().damage);
          self.mut_data().should_exist = false;
          collided = true;
          break;
        }
      }
      if collided { break; }
    }
  }
  
  fn physics(&mut self, delta_time: f32) {
    let velocity = self.data().velocity;
    let acceleration = self.data().acceleration;
    self.mut_data().position += velocity*delta_time;
    self.mut_data().velocity += acceleration*delta_time*delta_time;
  }

  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    self.data().animation.draw(self.data().position, self.data().size, 
                               self.data().rotation, self.data().texture.to_string(), draw_calls);
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
