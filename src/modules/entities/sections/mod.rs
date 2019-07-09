pub use self::modular_slots::ModularSlot;
pub use self::hull::Hull;
pub use self::thruster::Thruster;
pub use self::weapon_mount::WeaponMount;
pub use self::wing::Wing;
pub use self::repair_bay::RepairBay;
pub use self::hull_material::HullMaterial;

mod modular_slots;
mod hull;
mod thruster;
mod weapon_mount;
mod wing;
mod repair_bay;
mod hull_material;
use maat_graphics::DrawCall;

use crate::modules::abilities::Ability;

use crate::cgmath::Vector2;

#[derive(Clone)]
pub struct ShipSectionData {
  modular_slots: Vec<ModularSlot>,
  offset: Vector2<f32>,
  size: Vector2<f32>,
}

impl ShipSectionData {
  pub fn new(offset: Vector2<f32>, size: Vector2<f32>) -> ShipSectionData {
    ShipSectionData {
      modular_slots: Vec::new(),
      offset,
      size,
    }
  }
}

pub trait ShipSectionClone {
  fn clone_ship_section(&self) -> Box<ShipSection>;
}

impl<T: 'static + ShipSection + Clone> ShipSectionClone for T {
  fn clone_ship_section(&self) -> Box<ShipSection> {
    Box::new(self.clone())
  }
}

impl Clone for Box<ShipSection> {
  fn clone(&self) -> Box<ShipSection> {
    self.clone_ship_section()
  }
}

pub trait ShipSection: ShipSectionClone {
  fn data(&self) -> &ShipSectionData;
  fn mut_data(&mut self) -> &mut ShipSectionData;
  
  fn offset(&self) -> Vector2<f32> {
    self.data().offset
  }
  
  fn size(&self) -> Vector2<f32> {
    self.data().size
  }
  
  fn active_abilities(&self) -> Vec<Box<Ability>> {
    let mut abilities = Vec::new();
    for slot in &self.data().modular_slots {
      if let Some(ability) = slot.ability() {
        abilities.push(ability);
      }
    }
    
    abilities
  }
  
  fn modular_slots(&self) -> Vec<ModularSlot> {
    self.data().modular_slots.clone()
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>);
}
