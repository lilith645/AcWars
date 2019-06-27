use crate::modules::abilities::Ability;

#[derive(Clone)]
pub struct ModularSlot {
  id: i32,
  linked_id: Option<i32>,
  ability: Option<Box<Ability>>,
}

impl ModularSlot {
  pub fn new(id: i32, linked_id: Option<i32>) -> ModularSlot {
    ModularSlot {
      id,
      linked_id,
      ability: None,
    }
  }
  
  pub fn ability(&self) -> Option<Box<Ability>> {
    self.ability.clone()
  }
  
  pub fn is_linked(&self) -> bool {
    self.linked_id.is_some()
  }
  
  pub fn gain_ability(&mut self, ability: Box<Ability>) {
    self.ability = Some(ability);
  }
  
  pub fn remove_ability(&mut self) {
    self.ability = None;
  }
}
