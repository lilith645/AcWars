use crate::modules::entities::{MutexEntity};
use crate::modules::projectiles::{BoxProjectile};

use crate::cgmath::{Vector2,Vector4};

use std::sync::Arc;

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
  
  pub fn has_multiple_entities(&self) -> bool {
    self.objects.len() > 1
  }
}

struct ObjectContainer {
  contents: Vec<Content>,
}

impl ObjectContainer {
  pub fn new() -> ObjectContainer {
    ObjectContainer {
      contents: Vec::with_capacity(100000),
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
  
  pub fn retrieve_colliding_objects(&self) -> Vec<Vec<MutexEntity>> {
    let mut return_objects = Vec::new();
    for content in &self.contents {
      if content.has_multiple_entities() {
        let mut new_objects = Vec::with_capacity(content.objects.len());
        for object in &content.objects {
          new_objects.push(Arc::clone(object));
        }
        return_objects.push(new_objects);
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
    let position = object.position();
    let offset = object.size().x.max(object.size().y)*0.5;
    let min = position - object.size()*0.5;//Vector2::new(offset, offset);
    let max = position + object.size()*0.5;//Vector2::new(offset, offset);
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
    
    let position = object.position();
    let offset = object.size().x.max(object.size().y)*0.5;
    let min = position - Vector2::new(offset, offset);
    let max = position + Vector2::new(offset, offset);
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
  
  pub fn retrieve_possible_entity_collisions(&self) -> Vec<Vec<MutexEntity>> {
    self.contents.retrieve_colliding_objects()
  }
}
