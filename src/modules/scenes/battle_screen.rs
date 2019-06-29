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


#[derive(Clone)]
pub struct FullEntity {
  pub ai: Box<EntityController>,
  pub entity: Box<Entity>,
  pub buffs: Vec<Box<Buff>>
}

pub struct BattleScreen {
  data: SceneData,
  input: player::Input,
  ship: Box<Entity>,
  buffs: Vec<Box<Buff>>,
  hostiles: Vec<FullEntity>,
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
      input: player::Input::new(),
      ship: Box::new(Ship::new()),
      buffs: Vec::new(),
      hostiles: vec!(
        FullEntity { 
          ai: Box::new(AbilitySpamAi::new()), 
          entity: Box::new(Brew::new().as_hostile().with_position(Vector2::new(640.0, 1500.0))),
          buffs: Vec::new(),
        },
        FullEntity { 
          ai: Box::new(AbilitySpamAi::new()), 
          entity: Box::new(Brew::new().as_hostile().with_position(Vector2::new(740.0, 1500.0))),
          buffs: Vec::new(),
        },
        FullEntity { 
          ai: Box::new(AbilitySpamAi::new()), 
          entity: Box::new(Brew::new().as_hostile().with_position(Vector2::new(840.0, 1500.0))),
          buffs: Vec::new(),
        },
      ),
      projectiles: Vec::new(),
      zoom: 0.75,
      camera: OrthoCamera::new(window_size.x, window_size.y),
      ability_ui: AbilityUi::new(),
      uis: vec!(Box::new(PauseUi::new(window_size))),
      escape_pressed_last_frame: false,
    }
  }
  
  pub fn recreate(window_size: Vector2<f32>, camera: OrthoCamera, ship: Box<Entity>, buffs: Vec<Box<Buff>>, hostiles: Vec<FullEntity>, projectiles: Vec<Box<Projectile>>, zoom: f32) -> BattleScreen {
    
    BattleScreen {
      data: SceneData::new(window_size, Vec::new()), 
      input: player::Input::new(),
      ship,
      buffs,
      hostiles,
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
    Box::new(BattleScreen::recreate(window_size, self.camera.clone(), self.ship.clone(), self.buffs.clone(), self.hostiles.clone(), self.projectiles.clone(), self.zoom))
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
    
    // hostiles
    
    let ship_pos = self.ship.position();
    for hostile in &mut self.hostiles {
      hostile.ai.update(&mut hostile.entity, ship_pos, dim, delta_time);
      
      let mut offset = 0;
      for i in 0..hostile.buffs.len() {
        if offset > i {
          break;
        }
        hostile.buffs[i-offset].update(&mut hostile.entity, delta_time);
        if !hostile.buffs[i-offset].should_exist() {
          hostile.buffs[i-offset].unapply_buff(&mut hostile.entity);
          hostile.buffs.remove(i-offset);
          offset += 1;
        }
      }
    }
    
    let mut offset = 0;
    for i in 0..self.hostiles.len() {
      if i < offset {
        break;
      }
      
      let (hostile_buffs, hostile_proj) = self.hostiles[i-offset].entity.update(delta_time);
      for buff in hostile_buffs {
        buff.apply_buff(&mut self.hostiles[i-offset].entity);
        self.hostiles[i-offset].buffs.push(buff);
      }
      for projectile in hostile_proj {
        new_projectiles.push(projectile);
      }
      
      if !self.hostiles[i-offset].entity.should_exist() {
        self.hostiles.remove(i-offset);
        offset += 1;
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
        match self.projectiles[i].hostile() {
          false => {
            for hostile in &mut self.hostiles {
              if hostile.entity.should_exist() {
                self.projectiles[i].collide_with(&mut hostile.entity);
              }
            }
          },
          true => {
            if self.ship.should_exist() {
              self.projectiles[i].collide_with(&mut self.ship);
            }
          }
        }
      }
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
    
    for hostile in &self.hostiles {
      hostile.entity.draw(draw_calls);
    }
    
    self.ship.draw(draw_calls);
    
    draw_calls.push(DrawCall::draw_instanced("Ftpl".to_string(), "Ftpl".to_string()));
    draw_calls.push(DrawCall::draw_instanced("Gob".to_string(), "Gob".to_string()));
    draw_calls.push(DrawCall::draw_instanced("Brew".to_string(), "Brew".to_string()));
    draw_calls.push(DrawCall::draw_instanced("LaserBeam".to_string(), "LaserBeam".to_string()));
    draw_calls.push(DrawCall::draw_instanced("Bulbz".to_string(), "Bulbz".to_string()));
    
    for hostile in &self.hostiles {
      hostile.entity.draw_ship_ui(draw_calls);
    }
    
    self.ship.draw_ship_ui(draw_calls);
    
    /*
    for projectile in &self.projectiles {
      projectile.draw_collision_circles(draw_calls);
    }
    for hostile in &self.hostiles {
      hostile.entity.draw_collision_circles(draw_calls);
    }
    self.ship.draw_collision_circles(draw_calls);*/
    
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
