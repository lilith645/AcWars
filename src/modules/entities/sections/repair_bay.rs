use crate::modules::entities::sections::{ShipSection, ShipSectionData};

#[derive(Clone)]
pub struct RepairBay {
  data: ShipSectionData,
}

impl RepairBay {
  pub fn new() -> RepairBay {
    RepairBay {
      data: ShipSectionData::new(),
    }
  }
}

impl ShipSection for RepairBay {
  fn data(&self) -> &ShipSectionData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut ShipSectionData {
    &mut self.data
  }
}


