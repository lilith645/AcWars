use maat_graphics::DrawCall;
use maat_graphics::imgui::*;

use hlua::Lua;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;
use crate::modules::scenes::{BattleScreen};

use crate::modules::ui::{Ui,BoxUi, ShipSelectUi};
use crate::modules::entities::{Entity, BoxEntity, Astroid, Brew, Ship, Sun};

use crate::cgmath::{Vector2, Vector4};

pub struct ShipSelectScreen {
  data: SceneData,
  select_ui: BoxUi,
  escape_pressed_last_frame: bool,
  possible_ships: Vec<BoxEntity>,
}

impl ShipSelectScreen {
  pub fn new(window_size: Vector2<f32>) -> ShipSelectScreen {
    let position = Vector2::new(0.0, 0.0);
    let size = Vector2::new(300.0, 300.0);
    let possible_ships: Vec<BoxEntity> = vec!(
      Box::new(Astroid::new(position, size)),
      Box::new(Sun::new(position)),
      Box::new(Brew::new(position)),
      Box::new(Ship::new(position)),
    );
    
    let mut textures = Vec::new();
    for ship in &possible_ships {
      textures.push(ship.texture());
    }
    
    ShipSelectScreen {
      data: SceneData::new_default(),
      select_ui: Box::new(ShipSelectUi::new(window_size, textures)),
      escape_pressed_last_frame: false, 
      possible_ships,
    }
  }
}

impl Scene for ShipSelectScreen {
  fn data(&self) -> &SceneData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut SceneData {
    &mut self.data
  }
  
  fn future_scene(&mut self, window_size: Vector2<f32>) -> Box<Scene> {
    let mut index = self.select_ui.external_option_value() as usize;
    Box::new(BattleScreen::new(window_size, self.possible_ships[index].clone()))
  }
  
  fn update(&mut self, _ui: Option<&maat_graphics::imgui::Ui>, _lua: Option<&mut Lua>, delta_time: f32) {
    let dim = self.data().window_dim;
    let mouse_pos = self.data.mouse_pos;
    
    let scroll_delta = self.data.scroll_delta;
    let left_mouse = self.data.left_mouse;
    let escape_pressed = self.data.keys.escape_pressed() && !self.escape_pressed_last_frame;
    
    let mut should_close = false;
    let mut should_resize = None;
    let mut should_next_scene = false;
    self.select_ui.update(mouse_pos, left_mouse, escape_pressed, dim, &mut should_close, &mut should_resize, &mut should_next_scene, scroll_delta, delta_time);
    self.mut_data().should_resize_window = should_resize;
    if should_close {
      self.mut_data().should_close = true;
    }
    
    if should_next_scene {
      self.mut_data().next_scene = true;
    }
    
    self.escape_pressed_last_frame = self.data().keys.escape_pressed();
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let dim = self.data().window_dim;
    let (width, height) = (dim.x as f32, dim.y as f32);
    
    self.select_ui.draw(draw_calls);
  }
}
