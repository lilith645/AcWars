use crate::modules::entities::{Entity, MutexEntity};
use crate::modules::projectiles::{Projectile, MutexProjectile, BoxProjectile};

use crate::cgmath::Vector2;

use std::sync::Arc;
use std::sync::Mutex;


struct Content {
  pub index: (i32, i32),
  pub objects: Vec<MutexEntity>,
}

impl Content {
  pub fn new(index: (i32, i32), objects: Vec<MutexEntity>) -> Content {
    Content {
      index,
      objects,
    }
  }
}

struct ObjectContainer {
  contents: Vec<Content>,
}

impl ObjectContainer {
  pub fn new() -> ObjectContainer {
    ObjectContainer {
      contents: Vec::new(),
    }
  }
  
  pub fn clear(&mut self) {
    self.contents.clear();
  }
  
  pub fn insert_object(&mut self, point: (i32, i32), object: MutexEntity) {
    let mut found_place = false;
    for content in &mut self.contents {
      if content.index == point {
        content.objects.push(Arc::clone(&object));
        found_place = true;
        break;
      }
    }
    
    if !found_place {
      self.contents.push(Content::new(point, vec!(Arc::clone(&object))));
    }
  }
  
  pub fn retrieve_objects(&self, point: (i32, i32)) -> Vec<MutexEntity> {
    let mut return_objects = Vec::new();
    for content in &self.contents {
      if content.index == point {
        for object in &content.objects {
          return_objects.push(Arc::clone(&object));
        }
        break;
      }
    }
    
    return_objects
  }
}

pub struct SpatialHash {
  cell_size: f32,
  contents: ObjectContainer,
}

impl SpatialHash {
  pub fn new(cell_size: f32) -> SpatialHash {
    SpatialHash {
      cell_size,
      contents: ObjectContainer::new(),
    }
  }
  
  pub fn clear(&mut self) {
    self.contents.clear();
  }
  
  fn hash(&self, point: Vector2<f32>) -> (i32, i32) {
    ((point.x/self.cell_size) as i32, (point.y/self.cell_size) as i32)
  }
  
  pub fn insert_object_for_point(&mut self, mutex_object: MutexEntity) {
    let object = mutex_object.lock().unwrap();
    let mut position = object.position();
    let min = position - object.size()*0.5;
    let max = position + object.size()*0.5;
    let min_hash = self.hash(min);
    let max_hash = self.hash(max);
    for i in min_hash.0..max_hash.0 {
      for j in min_hash.1..max_hash.1 {
        self.contents.insert_object((i,j), Arc::clone(&mutex_object));
      }
    }
  }
  
  pub fn retrieve_objects(&self, object: &BoxProjectile) -> Vec<MutexEntity> {
    let mut objects = Vec::new();
    
    let mut position = object.position();
    let min = position - object.size()*0.5;
    let max = position + object.size()*0.5;
    let min_hash = self.hash(min);
    let max_hash = self.hash(max);
    for i in min_hash.0..max_hash.0 {
      for j in min_hash.1..max_hash.1 {
        for object in self.contents.retrieve_objects((i,j)) {
          objects.push(Arc::clone(&object));
        }
      }
    }
    
    objects
  }
}
