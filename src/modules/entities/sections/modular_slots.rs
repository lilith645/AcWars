use crate::modules::abilities::BoxAbility;

#[derive(Clone)]
pub struct ModularSlot {
  id: i32,
  linked_id: Option<i32>,
  ability: Option<BoxAbility>,
}

impl ModularSlot {
  pub fn new(id: i32, linked_id: Option<i32>) -> ModularSlot {
    ModularSlot {
      id,
      linked_id,
      ability: None,
    }
  }
  
  pub fn ability(&self) -> Option<BoxAbility> {
    self.ability.clone()
  }
  
  pub fn is_linked(&self) -> bool {
    self.linked_id.is_some()
  }
  
  pub fn gain_ability(&mut self, ability: BoxAbility) {
    self.ability = Some(ability);
  }
  
  pub fn remove_ability(&mut self) {
    self.ability = None;
  }
}
