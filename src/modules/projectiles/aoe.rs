
use crate::modules::projectiles::{Projectile, ProjectileData};

use crate::cgmath::{Vector2};

#[derive(Clone)]
pub struct Aoe {
  data: ProjectileData
}

impl Aoe {
  pub fn new(pos: Vector2<f32>, radius: f32) -> Aoe {
    let size = Vector2::new(radius, radius);//Vector2::new(50.0, 500.0);
    
    let position = pos-size*0.5;
    let texture = "Aoe".to_string();
    let sprite_rows = 1;
    let animation_timer = 0.0;
    
    Aoe {
      data: ProjectileData::new(position, size, texture.to_string(), sprite_rows, animation_timer)
                            .with_damage(5.0)
                            .with_life_time(10.01),
    }
  }
}

impl Projectile for Aoe {
  fn data(&self) -> &ProjectileData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut ProjectileData {
    &mut self.data
  }
  
  fn collision_information(&self) -> Vec<(Vector2<f32>, f32)> {
    let radius = self.data().size.x;
    vec!((Vector2::new(radius*0.5, radius*0.5), radius))
  }
  
  
  fn update(&mut self, delta_time: f32) {
    self.lifetime_decay(delta_time);
  }
}

