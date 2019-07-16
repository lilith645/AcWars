use crate::modules::areas::{Area, AreaData};
use crate::modules::entities::{FullEntity, Astroid};
use crate::modules::controllers::{FloatingAi};

use crate::cgmath::Vector2;



#[derive(Clone)]
pub struct AstroidField {
  data: AreaData,
}

impl AstroidField {
  pub fn new(position: Vector2<f32>, size: Vector2<f32>) -> AstroidField {
    let reasonable_spawn_field = size*0.8;
    
    let astroid_size = (reasonable_spawn_field.x/32.0).max(25.0).min(100.0);
    let astroid_size = Vector2::new(astroid_size, astroid_size);
    
    
    
    let mut astroids = Vec::new();
    
    let initial_pos = position-reasonable_spawn_field*0.5;
    
    let iterations = 20;
    
    for i in 0..iterations {
      for j in 0..iterations {
        let pos = initial_pos + Vector2::new(reasonable_spawn_field.x*(1.0/iterations as f32)*i as f32,
                                             reasonable_spawn_field.y*(1.0/iterations as f32)*j as f32);
        astroids.push(FullEntity::new(Box::new(FloatingAi::new()), 
                                      Box::new(Astroid::new(pos, astroid_size).as_misc())));
      }
    }
    
    let mut data = AreaData::new(position, size);
    
    for astroid in astroids {
      data = data.with_entity(astroid);
    }
    
    AstroidField {
      data,
    }
  }
}

impl Area for AstroidField {
  fn data(&self) -> &AreaData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut AreaData {
    &mut self.data
  }
  
  fn update_area(&mut self, _delta_time: f32) {
    
  }
}
