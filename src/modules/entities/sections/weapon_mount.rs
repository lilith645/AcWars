use crate::modules::entities::sections::{ShipSection, ShipSectionData};

#[derive(Clone)]
pub struct WeaponMount {
  data: ShipSectionData,
}

impl WeaponMount {
  pub fn new() -> WeaponMount {
    WeaponMount {
      data: ShipSectionData::new(),
    }
  }
}

impl ShipSection for WeaponMount {
  fn data(&self) -> &ShipSectionData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut ShipSectionData {
    &mut self.data
  }
}


