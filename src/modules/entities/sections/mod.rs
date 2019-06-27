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

use crate::modules::abilities::Ability;

#[derive(Clone)]
pub struct ShipSectionData {
  modular_slots: Vec<ModularSlot>,
}

impl ShipSectionData {
  pub fn new() -> ShipSectionData {
    ShipSectionData {
      modular_slots: Vec::new(),
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
}
