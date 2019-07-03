use maat_graphics::DrawCall;
use maat_graphics::camera::OrthoCamera;
use maat_graphics::imgui;

use maat_gui;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;

use crate::modules::buffs::Buff;
use crate::modules::entities::{Entity, Ship, Brew};
use crate::modules::projectiles::{Projectile};
use crate::modules::controllers::{EntityController, AbilitySpamAi};
use crate::modules::areas::{Area, SolarSystem, AstroidField};
use crate::modules::player;
use crate::modules::ui::{Ui, PauseUi, AbilityUi};

use crate::cgmath::{Vector2};

use hlua::Lua;

enum UiIndex {
  PauseUi,
  AbilityUi,
}

impl UiIndex {
  pub fn n(self) -> usize {
    self as usize
  }
}

pub struct BattleScreen {
  data: SceneData,
  areas: Vec<Box<Area>>,
  input: player::Input,
  ship: Box<Entity>,
  buffs: Vec<Box<Buff>>,
  projectiles: Vec<Box<Projectile>>,
  zoom: f32,
  camera: OrthoCamera,
  ability_ui: AbilityUi,
  uis: Vec<Box<Ui>>,
  escape_pressed_last_frame: bool, 
}

impl BattleScreen {
  pub fn new(window_size: Vector2<f32>) -> BattleScreen {
    BattleScreen {
      data: SceneData::new(window_size, Vec::new()),
      areas: vec!(Box::new(SolarSystem::new(Vector2::new(-1500.0, 1500.0), Vector2::new(2000.0, 2000.0))),
                  Box::new(AstroidField::new(Vector2::new(1500.0, -1500.0), Vector2::new(500.0, 1000.0)))),
      input: player::Input::new(),
      ship: Box::new(Ship::new()),
      buffs: Vec::new(),
      projectiles: Vec::new(),
      zoom: 0.75,
      camera: OrthoCamera::new(window_size.x, window_size.y),
      ability_ui: AbilityUi::new(),
      uis: vec!(Box::new(PauseUi::new(window_size))),
      escape_pressed_last_frame: false,
    }
  }
  
  pub fn recreate(window_size: Vector2<f32>, camera: OrthoCamera, areas: Vec<Box<Area>>, ship: Box<Entity>, buffs: Vec<Box<Buff>>, projectiles: Vec<Box<Projectile>>, zoom: f32) -> BattleScreen {
    
    BattleScreen {
      data: SceneData::new(window_size, Vec::new()), 
      areas,
      input: player::Input::new(),
      ship,
      buffs,
      projectiles,
      zoom,
      camera,
      ability_ui: AbilityUi::new(),
      uis: vec!(Box::new(PauseUi::new(window_size))),
      escape_pressed_last_frame: false,
    }
  }
}

impl Scene for BattleScreen {
  fn data(&self) -> &SceneData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut SceneData {
    &mut self.data
  }
  
  fn future_scene(&mut self, window_size: Vector2<f32>) -> Box<Scene> {
    Box::new(BattleScreen::recreate(window_size, self.camera.clone(), self.areas.clone(), self.ship.clone(), self.buffs.clone(), self.projectiles.clone(), self.zoom))
  }
  
  fn update(&mut self, _ui: Option<&imgui::Ui>, _lua: Option<&mut Lua>, delta_time: f32) {
    self.mut_data().controller.update();
    
    let dim = self.data().window_dim;
    let mouse_pos = self.data.mouse_pos;
    
    let left_mouse = self.data.left_mouse;
    let middle_mouse = self.data.middle_mouse;
    let right_mouse = self.data.right_mouse;
    let q_pressed = self.data.keys.q_pressed();
    let escape_pressed = self.data.keys.escape_pressed() && !self.escape_pressed_last_frame;
    
    // Key presses
    if escape_pressed {
      if self.uis[UiIndex::PauseUi.n()].enabled() {
        self.uis[UiIndex::PauseUi.n()].disable();
      } else {
        self.uis[UiIndex::PauseUi.n()].enable();
      }
    }
    
    // UI
    let mut should_close = false;
    for ui in &mut self.uis {
      ui.update(mouse_pos, left_mouse, escape_pressed, dim, &mut should_close, delta_time);
    }
    if should_close {
      self.mut_data().should_close = true;
    }
    
    self.escape_pressed_last_frame = self.data().keys.escape_pressed();
    if self.uis[UiIndex::PauseUi.n()].enabled() {
      return;
    }
    
    // Player
    let left_stick_position =  self.data().controller.left_stick_position();
    let xbox_a_button = self.data().controller.a_button_pressed();
    let right_trigger_pressed = self.data().controller.right_trigger_pressed();
    self.input.update(&mut self.ship, left_stick_position, xbox_a_button, right_trigger_pressed, mouse_pos, left_mouse, middle_mouse, right_mouse, q_pressed, dim, delta_time);
    
    let (mut buffs, mut new_projectiles) = self.ship.update(delta_time);
    
    let mut offset = 0;
    for i in 0..self.buffs.len() {
      self.buffs[i-offset].update(&mut self.ship, delta_time);
      if !self.buffs[i-offset].should_exist() {
        self.buffs[i-offset].unapply_buff(&mut self.ship);
        self.buffs.remove(i-offset);
        offset += 1;
      }
    }
    
    for buff in buffs {
      buff.apply_buff(&mut self.ship);
      self.buffs.push(buff);
    }
    
    for area in &mut self.areas {
      let projectiles = area.update(&mut self.ship, dim, delta_time);
      for projectile in projectiles {
        new_projectiles.push(projectile);
      }
    }
    
    // Projectiles
    for new_projectile in new_projectiles {
      self.projectiles.push(new_projectile);
    }
    
    offset = 0;
    for i in 0..self.projectiles.len() {
      if i < offset {
        break;
      }
      
      self.projectiles[i-offset].update(delta_time);
      if !self.projectiles[i-offset].should_exist() {
        self.projectiles.remove(i-offset);
        offset += 1;
      }
    }
    
    // Check collisions 
    for i in 0..self.projectiles.len() {
      if self.projectiles[i].should_exist() {
        for area in &mut self.areas {
          area.collide_with(&mut self.projectiles[i]);
        }
        if self.projectiles[i].can_hit(self.ship.hostility()) {
          if self.ship.should_exist() {
            self.projectiles[i].collide_with(&mut self.ship);
          }
        }
      }
    }
    
    for area in &mut self.areas {
      area.internal_collisions(&mut self.ship);
    }
    
    self.ability_ui.update(dim);
    
    self.camera.window_resized(dim.x, dim.y);
    let camera_target = self.ship.position()*self.zoom - Vector2::new(dim.x*0.5, dim.y*0.5);
    self.camera.lerp_to_position(camera_target,  Vector2::new(0.05, 0.05));
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let dim = self.data().window_dim;
    let (width, height) = (dim.x as f32, dim.y as f32);
    
    draw_calls.push(DrawCall::set_texture_scale(self.zoom));
    
    draw_calls.push(DrawCall::replace_ortho_camera(self.camera.clone()));
    
    let bg_width = 1600.0;
    let bg_height = 1200.0;
    for i in 0..10 {
      for j in 0..10 {
        draw_calls.push(
          DrawCall::draw_textured(Vector2::new(bg_width*0.5+bg_width*(i as f32-4.0), bg_height*0.5+bg_height*(j as f32-4.0)),
                                  Vector2::new(bg_width*1.0, bg_height*1.0),
                                  0.0,
                                  "bg_space".to_string())
        );
      }
    }
    
    for projectile in &self.projectiles {
      projectile.draw(draw_calls);
    }
    
    for area in &self.areas {
      area.draw(draw_calls);
    }
    
    self.ship.draw(draw_calls);
    
    draw_calls.push(DrawCall::draw_instanced("Astroid".to_string(), "Astroid".to_string()));
    draw_calls.push(DrawCall::draw_instanced("Sun".to_string(), "Sun".to_string()));
    draw_calls.push(DrawCall::draw_instanced("Ftpl".to_string(), "Ftpl".to_string()));
    draw_calls.push(DrawCall::draw_instanced("Gob".to_string(), "Gob".to_string()));
    draw_calls.push(DrawCall::draw_instanced("Brew".to_string(), "Brew".to_string()));
    draw_calls.push(DrawCall::draw_instanced("LaserBeam".to_string(), "LaserBeam".to_string()));
    draw_calls.push(DrawCall::draw_instanced("Bulbz".to_string(), "Bulbz".to_string()));
    
    for area in &self.areas {
      area.draw_ship_ui(draw_calls);
    }
    
    self.ship.draw_ship_ui(draw_calls);
    
    /*
    for projectile in &self.projectiles {
      projectile.draw_collision_circles(draw_calls);
    }
    for area in &self.areas {
      area.draw_collision_circles(draw_calls);
    }
    self.ship.draw_collision_circles(draw_calls);
    */
    draw_calls.push(DrawCall::set_texture_scale(1.0));
    draw_calls.push(DrawCall::reset_ortho_camera());
    
    let (abl, abm, abr, ab1, ab2, ab3, ab4) = self.input.return_abilities();
    self.ability_ui.draw(abl, abm, abr, ab1, ab2, ab3, ab4, draw_calls);
    
    for ui in &self.uis {
      ui.draw(draw_calls);
    }
    
    //maat_gui::test_drawing("NoAbilityIcon".to_string(), "Arial".to_string(), draw_calls);
  }
}
