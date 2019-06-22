pub use ftpl::Ftpl;

mod ftpl;

use maat_graphics::DrawCall;

use cgmath::{Vector2, Vector3};

#[derive(Clone)]
pub struct ProjectileData {
  position: Vector2<f32>,
  rotation: f32,
  size: Vector2<f32>,
  texture: String,
  velocity: Vector2<f32>,
  acceleration: Vector2<f32>,
  animation_sprite_rows: i32,
  animation_timer: f32,
  animation_frame: i32,
  animation_total_time: f32,
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
      animation_sprite_rows: 1,
      animation_timer: 1.0,
      animation_frame: 0,
      animation_total_time: 0.0,
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
      animation_sprite_rows: sprite_rows,
      animation_timer,
      animation_frame: 0,
      animation_total_time: 0.0,
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
  
  pub fn with_acceleration(mut self, acc: Vector2<f32>) -> ProjectileData {
    self.acceleration = acc;
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
  
  fn update(&mut self, delta_time: f32);
  
  fn physics(&mut self, delta_time: f32) {
    self.mut_data().position += self.data().velocity*delta_time;
    self.mut_data().velocity += self.data().acceleration*delta_time*delta_time;
  }
  
  fn animate(&mut self, delta_time: f32) {
    self.mut_data().animation_total_time += delta_time;
    
    if self.data().animation_total_time > self.data().animation_timer {
      self.mut_data().animation_frame += 1;
      if self.data().animation_frame >= self.data().animation_sprite_rows*self.data().animation_sprite_rows {
        self.mut_data().animation_frame = 0
      }
      self.mut_data().animation_total_time -= self.data().animation_timer;
    }
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let sprite_sheet_rows = self.data().animation_sprite_rows;
    
    let x = self.data().animation_frame % sprite_sheet_rows;
    let mut y = 0;
    if self.data().animation_frame-x == sprite_sheet_rows {
      y = 1;
    }
    if self.data().animation_frame-x == sprite_sheet_rows*2 {
      y = 2;
    }
    
  /*  draw_calls.push(DrawCall::draw_sprite_sheet(self.data().position, self.data().size, self.data().rotation, 
                                                self.data().texture.to_string(), Vector3::new(x,y, sprite_sheet_rows)));
    */
     draw_calls.push(DrawCall::add_instanced_sprite_sheet(self.data().position, self.data().size, self.data().rotation, 
                                                self.data().texture.to_string(), Vector3::new(x,y, sprite_sheet_rows)));
  }
}
