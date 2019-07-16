/*
pub fn spatial_hash_collision(&self) {
    let mut spatial_hash = self.spatial_hash.lock().unwrap();
    spatial_hash.clear();
    for area in &self.areas {
      for mutex_entity in &area.entities() {
        spatial_hash.insert_object_for_point(Arc::clone(&mutex_entity));
      }
    }
    spatial_hash.insert_object_for_point(Arc::clone(&self.ship));
    
    for i in 0..self.projectiles.len() {
      let mut projectile = self.projectiles[i].lock().unwrap();
      if projectile.should_exist() {
        // entity collision 
        let mut entities = spatial_hash.retrieve_objects(&*projectile);
        for entity_mutex in &mut entities {
          if !projectile.should_exist() {
            break;
          }
          
          let mut entity = entity_mutex.lock().unwrap();
          if entity.should_exist() {
            if projectile.can_hit(entity.hostility()) {
              projectile.collide_with(&mut *entity);
            }
          }
        }
      }
    }
    
    let entity_groups = spatial_hash.retrieve_possible_entity_collisions();
    for group in &entity_groups {
      for i in 0..group.len() {
        for j in i..group.len() {
          if i == j {
            continue;
          }
          let mut entity_one = group[i].lock().unwrap();
          let mut entity_two = group[j].lock().unwrap();
          
          if entity_one.should_exist() && entity_two.should_exist() &&
             !entity_one.is_in_phase_mode() && !entity_two.is_in_phase_mode() {
            entity_one.collide_with(&mut *entity_two);
            entity_two.collide_with(&mut *entity_one);
          }
        }
      }
    }
    
    spatial_hash.clear();*/

use crate::modules::spatial_hash::SpatialHash;
use crate::modules::entities::MutexEntity;
use crate::modules::projectiles::MutexProjectile;

use std::sync::Arc;

pub fn collisions(mut entities: Vec<MutexEntity>, mut projectiles: Vec<MutexProjectile>) {
  let mut spatial_hash = SpatialHash::new(30.0);
  
  for mutex_entity in &entities {
    spatial_hash.insert_object_for_point(Arc::clone(&mutex_entity));
  }
  
   for i in 0..projectiles.len() {
    if let Some(projectile) = &mut projectiles[i].try_lock() {
      if projectile.should_exist() {
        // entity collision 
        let mut entities = spatial_hash.retrieve_objects(&*projectile);
        for entity_mutex in &mut entities {
          if !projectile.should_exist() {
            break;
          }
          
          let mut entity = entity_mutex.lock();
          if entity.should_exist() {
            if projectile.can_hit(entity.hostility()) {
              projectile.collide_with(&mut *entity);
            }
          }
        }
      }
    }
  }
  
  let entity_groups = spatial_hash.retrieve_possible_entity_collisions();
    for group in &entity_groups {
      for i in 0..group.len() {
        for j in i..group.len() {
          if i == j {
            continue;
          }
          if let Some(mut entity_one) = group[i].try_lock() {
          if let Some(mut entity_two) = group[j].try_lock() {
          
          if entity_one.should_exist() && entity_two.should_exist() &&
             !entity_one.is_in_phase_mode() && !entity_two.is_in_phase_mode() {
            entity_one.collide_with(&mut *entity_two);
            entity_two.collide_with(&mut *entity_one);
          }
          }
          }
        }
      }
    }
}
