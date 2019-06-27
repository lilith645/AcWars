use crate::modules::entities::sections::{ShipSection, ShipSectionData};

#[derive(Clone)]
pub struct Hull {
  data: ShipSectionData,
}

impl Hull {
  pub fn new() -> Hull {
    Hull {
      data: ShipSectionData::new(),
    }
  }
}

impl ShipSection for Hull {
  fn data(&self) -> &ShipSectionData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut ShipSectionData {
    &mut self.data
  }
}


