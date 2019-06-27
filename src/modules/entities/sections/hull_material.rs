use crate::modules::entities::sections::{ShipSection, ShipSectionData};

#[derive(Clone)]
pub struct HullMaterial {
  data: ShipSectionData,
}

impl HullMaterial {
  pub fn new() -> HullMaterial {
    HullMaterial {
      data: ShipSectionData::new(),
    }
  }
}

impl ShipSection for HullMaterial {
  fn data(&self) -> &ShipSectionData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut ShipSectionData {
    &mut self.data
  }
}


