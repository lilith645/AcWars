use crate::modules::entities::sections::{ShipSection, ShipSectionData};

#[derive(Clone)]
pub struct Thruster {
  data: ShipSectionData,
}

impl Thruster {
  pub fn new() -> Thruster {
    Thruster {
      data: ShipSectionData::new(),
    }
  }
}

impl ShipSection for Thruster {
  fn data(&self) -> &ShipSectionData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut ShipSectionData {
    &mut self.data
  }
}


