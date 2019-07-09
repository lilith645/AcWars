use maat_graphics::math;

use crate::modules::projectiles::{Projectile, ProjectileData};

use std::f32::consts::PI;

use crate::cgmath::{Vector2};

// Ftpl - Fancy torpedo phaser laser

#[derive(Clone)]
pub struct Ftpl {
  data: ProjectileData
}

impl Ftpl {
  pub fn new(center_pos: Vector2<f32>, offset: Vector2<f32>, direction: Vector2<f32>) -> Ftpl {
    
    let position = Vector2::new(center_pos.x+offset.x*direction.x, 
                                center_pos.y+offset.y*direction.y);
    
    let size = Vector2::new(50.0, 50.0);
    let velocity = math::normalise_vector2(direction)*1200.0;
    let texture = "Ftpl".to_string();
    let sprite_rows = 3;
    let animation_timer = 0.06;
    
    let rotation = direction.x.atan2(direction.y);
    
    let rot_degree = 360.0-(rotation*180.0)/PI;
    
    Ftpl {
      data: ProjectileData::new(position, size, texture.to_string(), sprite_rows, animation_timer)
                            .with_velocity(velocity)
                            .with_rotation(rot_degree)
                            .with_damage(1.5),
    }
  }
}

impl Projectile for Ftpl {
  fn data(&self) -> &ProjectileData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut ProjectileData {
    &mut self.data
  }
  
  fn collision_information(&self) -> Vec<(Vector2<f32>, f32)> {
    let radius = self.data().size.x.min(self.data().size.y)*0.5 * 0.2;
    
    vec!((Vector2::new(0.0, 0.0), radius))
  }
  
  
  fn update(&mut self, delta_time: f32) {
    self.lifetime_decay(delta_time);
    self.physics(delta_time);
    self.mut_data().animation.update(delta_time);
  }
}

