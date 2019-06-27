use crate::modules::entities::sections::{ShipSection, ShipSectionData};

#[derive(Clone)]
pub struct Wing {
  data: ShipSectionData,
}

impl Wing {
  pub fn new() -> Wing {
    Wing {
      data: ShipSectionData::new(),
    }
  }
}

impl ShipSection for Wing {
  fn data(&self) -> &ShipSectionData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut ShipSectionData {
    &mut self.data
  }
}


