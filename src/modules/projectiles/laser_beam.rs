use maat_graphics::math;

use crate::modules::projectiles::{Projectile, ProjectileData};

use std::f32::consts::PI;

use crate::cgmath::{Vector2, InnerSpace};

#[derive(Clone)]
pub struct LaserBeam {
  data: ProjectileData
}

impl LaserBeam {
  pub fn new(center_pos: Vector2<f32>, offset: Vector2<f32>, direction: Vector2<f32>) -> LaserBeam {
    let mut size = Vector2::new(50.0, 500.0);//Vector2::new(50.0, 500.0);
    
    let position = Vector2::new(center_pos.x+offset.x*direction.x+size.y*0.5*direction.x, 
                                center_pos.y+offset.y*direction.y+size.y*0.5*direction.y);
    let velocity = direction.normalize()*0.0;
    let texture = "LaserBeam".to_string();
    let sprite_rows = 1;
    let animation_timer = 0.06;
    
    let rotation = direction.x.atan2(direction.y);
    
    let rot_degree = 360.0-(rotation*180.0)/PI;
    
    LaserBeam {
      data: ProjectileData::new(position, size, texture.to_string(), sprite_rows, animation_timer)
                            .with_velocity(velocity)
                            .with_rotation(rot_degree)
                            .with_damage(1.5),
    }
  }
}

impl Projectile for LaserBeam {
  fn data(&self) -> &ProjectileData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut ProjectileData {
    &mut self.data
  }
  
  fn collision_information(&self) -> Vec<(Vector2<f32>, f32)> {
    let radius = self.data().size.x.min(self.data().size.y)*0.5;
    
    let goal = self.data().size.x.max(self.data().size.y)*0.5;
    
    let mut laser_circles = Vec::new();
    laser_circles.push((Vector2::new(0.0, 0.0), radius));
    
    let pos = self.data().position;
    let rotation = self.data().rotation+90.0;
    
    let num_circles = (goal / radius).floor() as i32 -1;
    
    for i in 0..num_circles {
      
      let offset = Vector2::new(radius*(i+1) as f32*(math::to_radians(rotation)).cos(),
                                radius*(i+1) as f32*(math::to_radians(rotation)).sin());
      
      laser_circles.push((offset, radius));
      laser_circles.push((-offset, radius));
    }
    
    laser_circles
  }
  
  
  fn update(&mut self, delta_time: f32) {
    self.lifetime_decay(delta_time);
    self.physics(delta_time);
    self.mut_data().animation.update(delta_time);
  }
}

