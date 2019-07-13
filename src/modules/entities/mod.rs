pub use self::ship::Ship;
pub use self::brew::Brew;
pub use self::sun::Sun;
pub use self::astroid::Astroid;
pub use self::wall::Wall;

pub mod sections;

mod ship;
mod brew;
mod sun;
mod astroid;
mod wall;

use maat_graphics::DrawCall;
use maat_graphics::math;

use crate::modules::entities::sections::{ShipSection, RepairBay, HullMaterial};
use crate::modules::projectiles::{Projectile, BoxProjectile};
use crate::modules::buffs::{Buff, BoxBuff};
use crate::modules::controllers::{EntityController, BoxEntityController};
use crate::modules::abilities::{BoxAbility, NoAbility};

use std::f32::consts::PI;

use crate::cgmath::{Vector2, Vector3, Vector4, InnerSpace};

use std::sync::{Arc, Mutex};

pub type MutexEntity = Arc<Mutex<BoxEntity>>;
pub type BoxEntity = Box<Entity>;

#[derive(Clone, PartialEq)]
pub enum Hostility {
  Hostile,
  Neutral,
  Friendly,
}

impl Hostility {
  pub fn is_hostile(&self) -> bool {
    *self == Hostility::Hostile
  }
  
  pub fn is_neutral(&self) -> bool {
    *self == Hostility::Neutral
  }
  
  pub fn is_friendly(&self) -> bool {
    *self == Hostility::Friendly
  }
  
  pub fn make_hostile(&mut self) {
    *self = Hostility::Hostile;
  }
  
  pub fn make_neutral(&mut self) {
    *self = Hostility::Neutral;
  }
  
  pub fn make_friendly(&mut self) {
    *self = Hostility::Friendly;
  }
  
  pub fn check_can_hit(&self, hostility: &Hostility) -> bool {
    if hostility == self {
      if self.is_neutral() {
        true
      } else {
        false
      }
    } else {
      true
    }
  }
  
  pub fn check_can_hurt(&self, hostility: &Hostility) -> bool {
    if self.check_can_hit(hostility) {
      match *self {
        Hostility::Friendly => {
          if hostility.is_hostile() {
            true
          } else {
            false
          }
        },
        Hostility::Neutral => {
          true
        },
        Hostility::Hostile => {
          if hostility.is_friendly() {
            true
          } else {
            false
          }
        }
      }
    } else {
      false
    }
  }
}

#[derive(Clone)]
pub struct FullEntity {
  pub ai: BoxEntityController,
  pub entity: MutexEntity,
  pub buffs: Vec<BoxBuff>
}

impl FullEntity {
  pub fn new(controller: Box<EntityController>, entity: BoxEntity) -> FullEntity { 
    FullEntity {
      ai: controller,
      entity: Arc::new(Mutex::new(entity)),
      buffs: Vec::new(),
    }
  }
}

unsafe impl Send for EntityData {
}
unsafe impl Sync for EntityData {
}


#[derive(Clone)]
pub struct EntityData {
  position: Vector2<f32>,
  rotation: f32,
  size: Vector2<f32>,
  texture: String,
  velocity: Vector2<f32>,
  max_velocity: f32,
  acceleration: Vector2<f32>,
  inertia: f32,
  health: f32,
  health_regen: f32, // per second
  max_health: f32,
  shield: f32,
  phase_mode: bool,
  projectiles: Vec<BoxProjectile>,
  buffs: Vec<Box<Buff>>,
  hostility: Hostility,
  should_exist: bool,
  ship_sections: Vec<Box<ShipSection>>,
  hull_material: Box<ShipSection>,
  repair_bay: Box<ShipSection>,
}

impl EntityData {
  pub fn new_empty() -> EntityData {
     EntityData {
      position: Vector2::new(0.0, 0.0),
      rotation: 0.0,
      size: Vector2::new(1.0, 1.0),
      texture: "".to_string(),
      velocity: Vector2::new(0.0, 0.0),
      max_velocity: 1.0,
      acceleration: Vector2::new(0.0, 0.0),
      inertia: 0.33,
      health: 100.0,
      health_regen: 0.0,
      max_health: 100.0,
      shield: 0.0,
      phase_mode: false,
      projectiles: Vec::new(),
      buffs: Vec::new(),
      hostility: Hostility::Friendly,
      should_exist: true,
      ship_sections: Vec::new(),
      hull_material: Box::new(HullMaterial::new(Vector2::new(0.0, 0.0), Vector2::new(50.0, 50.0))),
      repair_bay: Box::new(RepairBay::new(Vector2::new(0.0, 0.0), Vector2::new(50.0, 50.0))),
    }
  }
  
  pub fn new(position: Vector2<f32>, size: Vector2<f32>, texture: String) -> EntityData {
     EntityData {
      position,
      rotation: 0.0,
      size,
      texture: texture.to_string(),
      velocity: Vector2::new(0.0, 0.0),
      max_velocity: 500.0,
      acceleration: Vector2::new(0.0, 0.0),
      inertia: 0.33,
      health: 100.0,
      health_regen: 0.0,
      max_health: 100.0,
      shield: 0.0,
      phase_mode: false,
      projectiles: Vec::new(),
      buffs: Vec::new(),
      hostility: Hostility::Friendly,
      should_exist: true,
      ship_sections: Vec::new(),
      hull_material: Box::new(HullMaterial::new(Vector2::new(0.0, 0.0), Vector2::new(50.0, 50.0))),
      repair_bay: Box::new(RepairBay::new(Vector2::new(0.0, 0.0), Vector2::new(50.0, 50.0))),
    }
  }
  
  pub fn with_rotation(mut self, rot: f32) -> EntityData {
    self.rotation = rot;
    self
  }
  
  pub fn with_velocity(mut self, vel: Vector2<f32>) -> EntityData {
    self.velocity = vel;
    self
  }
  
  pub fn with_max_velocity(mut self, max_vel: f32) -> EntityData {
    self.max_velocity = max_vel;
    self
  }
  
  pub fn with_inertia(mut self, inertia: f32) -> EntityData {
    self.inertia = inertia;
    self
  }
  
  pub fn with_health(mut self, health: f32) -> EntityData {
    self.health = health;
    self.max_health = health;
    self
  }
  
  pub fn as_hostile(mut self) -> EntityData {
    self.hostility.make_hostile();
    self
  }
  
  pub fn as_neutral(mut self) -> EntityData {
    self.hostility.make_neutral();
    self
  }
  
  pub fn with_ship_section(mut self, section: Box<ShipSection>) -> EntityData {
    self.ship_sections.push(section);
    self
  }
  
  pub fn with_health_regen(mut self, regen: f32) -> EntityData {
    self.health_regen = regen;
    self
  }
}

pub trait EntityClone {
  fn clone_entity(&self) -> Box<Entity>;
}

impl<T: 'static + Entity + Clone + Send + Sync> EntityClone for T {
  fn clone_entity(&self) -> Box<Entity> {
    Box::new(self.clone())
  }
}

impl Clone for Box<Entity> {
  fn clone(&self) -> Box<Entity> {
    self.clone_entity()
  }
}

pub trait Entity: EntityClone {
  fn data(&self) -> &EntityData;
  fn mut_data(&mut self) -> &mut EntityData;
  
  fn collision_information(&self) -> Vec<(Vector2<f32>, f32)>;
  fn collide_with(&mut self, entity: &mut Box<Entity>);
  
  fn update(&mut self, delta_time: f32) -> (Vec<BoxBuff>, Vec<BoxProjectile>) {
    self.physics(delta_time);
    self.mut_data().health += self.data().health_regen*delta_time;
    if self.data().health >= self.data().max_health {
      self.mut_data().health = self.data().max_health;
    }
    
    (self.return_buffs(), self.return_projectiles())
  }
  
  fn texture(&self) -> String {
    self.data().texture.to_string()
  }
  
  fn position(&self) -> Vector2<f32> {
    self.data().position
  }
  
  fn size(&self) -> Vector2<f32> {
    self.data().size
  }
  
  fn rotation(&self) -> f32 {
    self.data().rotation
  }
  
  fn velocity(&self) -> Vector2<f32> {
    self.data().velocity
  }
  
  fn hostility(&self) -> &Hostility {
    &self.data().hostility
  }
  
  fn max_velocity(&self) -> f32 {
    self.data().max_velocity
  }
  
  fn should_exist(&self) -> bool {
    self.data().should_exist
  }
  
  fn is_in_phase_mode(&self) -> bool {
    self.data().phase_mode
  }
  
  fn ship_sections(&self) -> &Vec<Box<ShipSection>> {
    &self.data().ship_sections
  }
  
  fn collision_circles(&self) -> Vec<Vector3<f32>> {
    let information = self.collision_information();
    
    let mut entity_circles = Vec::new();
    
    let entity_position = self.data().position;
    
    for (offset, radius) in information {
      let entity_radius = radius;
      let entity_circle = (entity_position+offset).extend(entity_radius);
      entity_circles.push(entity_circle);
    }
    
    entity_circles
  }
  
  fn gain_shield(&mut self, shield_value: f32) {
    self.mut_data().shield += shield_value;
  }
  
  fn hit(&mut self, damage: f32) {
    if self.data().shield > 0.0 {
      if self.data().shield < damage {
        self.mut_data().shield -= damage;
        self.mut_data().health += self.data().shield;
        self.mut_data().shield = 0.0;
      } else {
        self.mut_data().shield -= damage;
      }
    } else {
      self.mut_data().health -= damage;
    }
    
    if self.data().health <= 0.0 {
      self.mut_data().should_exist = false;
      self.mut_data().health = 0.0;
    }
  }
  
  fn set_phase_mode(&mut self, should_phase: bool) {
    self.mut_data().phase_mode = should_phase;
  }
  
  fn set_position(&mut self, position: Vector2<f32>) {
    self.mut_data().position = position;
  }
  
  fn set_velocity(&mut self, vel: Vector2<f32>) {
    self.mut_data().velocity = vel;
  }
  
  fn set_max_velocty(&mut self, max_vel: f32) {
    self.mut_data().max_velocity = max_vel;
  }
  
  fn set_rotation(&mut self, rot: f32) {
    self.mut_data().rotation = rot;
  }
  
  fn set_facing(&mut self, target: Vector2<f32>) {
    let direction = target-self.data().position;
    let rotation = direction.x.atan2(direction.y);
    
    let rot_degree = 360.0-(rotation*180.0)/PI;
    self.mut_data().rotation = rot_degree;
  }
  
  fn set_velocity_magnitude(&mut self, vel: f32) {
    let vel_dir = math::normalise_vector2(self.data().velocity);
    self.mut_data().velocity = vel_dir*vel;
  }
  
  fn set_acceleration_magnitude(&mut self, acc: f32) {
    let acc_dir = math::normalise_vector2(self.data().acceleration);
    self.mut_data().velocity = acc_dir*acc;
  }
  
  fn add_acceleration(&mut self, acc: Vector2<f32>) {
    self.mut_data().acceleration += acc;
  }
  
  fn apply_acceleration_in_direction(&mut self, direction: Vector2<f32>) {
    self.mut_data().acceleration = math::normalise_vector2(direction);
  }
  
  fn fire_projectile(&mut self, mut projectile: Box<Projectile>) {
    if self.data().hostility.is_hostile() {
      projectile.make_hostile(); 
    } else if self.data().hostility.is_neutral() {
      projectile.make_neutral();
    }
    
    self.mut_data().projectiles.push(projectile);
  }
  
  fn activate_buff(&mut self, buff: Box<Buff>) {
    self.mut_data().buffs.push(buff);
  }
  
  fn physics(&mut self, delta_time: f32) {
    let velocity = self.data().velocity;
    let max_velocity = self.data().max_velocity;
    let acceleration = self.data().acceleration;
    let inertia = self.data().inertia;
    self.mut_data().position += velocity*delta_time;
    self.mut_data().velocity -= velocity*(1.0-inertia)*delta_time;
    self.mut_data().velocity += acceleration*max_velocity*(1.0-inertia)*delta_time;
    
    if self.data().velocity.magnitude() > self.data().max_velocity {
      self.mut_data().velocity = math::normalise_vector2(velocity)*max_velocity;
    }
    
    self.mut_data().acceleration = Vector2::new(0.0, 0.0);
  }
  
  fn entity_collision(&mut self, entity: &mut Box<Entity>, damage: f32, velocity: f32) {
    let entity_circles = entity.collision_circles();
    let astroid_circles = self.collision_circles();
    
    let mut collided = false;
    
    for e_circle in entity_circles {
      for p_circle in &astroid_circles {
        if math::circle_collision(e_circle, *p_circle) {
          entity.hit(damage);
          self.hit(damage);
          
          let center = (self.position()+entity.position())*0.5;
          
          let astroid_direction = math::normalise_vector2(self.position()-center);
          let _current_vel = self.velocity();
          self.set_velocity(astroid_direction*velocity);
          self.apply_acceleration_in_direction(astroid_direction);
          
          let entity_direction =  -1.0*astroid_direction;
          let _current_vel = entity.velocity();
          entity.set_velocity(entity_direction*velocity);
          entity.apply_acceleration_in_direction(entity_direction);
          
          collided = true;
          break;
        }
      }
      if collided { break; }
    }
  }
  
  fn return_projectiles(&mut self) -> Vec<BoxProjectile> {
    let projectiles = self.data().projectiles.clone();
    self.mut_data().projectiles.clear();
    
    projectiles
  }
  
  fn return_buffs(&mut self) -> Vec<Box<Buff>> {
    let buffs = self.data().buffs.clone();
    self.mut_data().buffs.clear();
    
    buffs
  }
  
  fn draw_ship_ui(&self, draw_calls: &mut Vec<DrawCall>) {
    if self.data().hostility.is_neutral() {
      return;
    }
    
    let ship_size = self.data().size;
    
    let position = self.data().position + Vector2::new(0.0, ship_size.y*0.5);
    let size = Vector2::new(40.0*(self.data().health / self.data().max_health), 10.0);
    let colour = Vector4::new(0.0, 1.0, 0.0, 0.5);
    draw_calls.push(DrawCall::draw_coloured(position, size, colour, 0.0));
    
    let size = Vector2::new(40.0*(self.data().shield / self.data().max_health), 15.0);
    let colour = Vector4::new(0.0, 0.0, 1.0, 0.5);
    draw_calls.push(DrawCall::draw_coloured(position, size, colour, 0.0));
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::add_instanced_sprite_sheet(self.data().position, self.data().size, 
                                                         self.data().rotation, 
                                                         self.data().texture.to_string(), 
                                                         Vector3::new(0,0, 1)));
  }
  
  fn draw_collision_circles(&self, draw_calls: &mut Vec<DrawCall>) {
    let circles = self.collision_circles();
    let colour = if self.data().hostility.is_hostile() {
      Vector4::new(1.0, 0.0, 0.0, 1.0)
    } else if self.data().hostility.is_friendly() {
      Vector4::new(0.0, 0.0, 1.0, 1.0)
    } else {
      Vector4::new(0.0, 1.0, 0.0, 1.0)
    };
    
    for circle in &circles {
      draw_calls.push(DrawCall::draw_coloured(circle.xy(), Vector2::new(circle.z*2.0, circle.z*2.0), colour, 0.0));
    }
  }
}
