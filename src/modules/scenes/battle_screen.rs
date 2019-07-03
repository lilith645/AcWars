use maat_graphics::DrawCall;
use maat_graphics::camera::OrthoCamera;
use maat_graphics::imgui;
use maat_graphics::ThreadPool;

use maat_gui;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;

use crate::modules::buffs::{Buff, BoxBuff};
use crate::modules::entities::{Entity, BoxEntity, MutexEntity, Ship, Brew};
use crate::modules::projectiles::{Projectile, BoxProjectile, MutexProjectile};
use crate::modules::controllers::{EntityController, AbilitySpamAi};
use crate::modules::areas::{Area, BoxArea, SolarSystem, AstroidField};
use crate::modules::player;
use crate::modules::ui::{Ui,BoxUi, PauseUi, AbilityUi};

use crate::cgmath::{Vector2};

use hlua::Lua;

use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use crate::modules::spatial_hash::SpatialHash;

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
  areas: Vec<BoxArea>,
  input: player::Input,
  ship: BoxEntity,
  buffs: Vec<BoxBuff>,
  projectiles: Vec<MutexProjectile>,
  zoom: f32,
  camera: OrthoCamera,
  ability_ui: AbilityUi,
  uis: Vec<BoxUi>,
  escape_pressed_last_frame: bool, 
  spatial_hash: Arc<Mutex<SpatialHash>>,
  pool: ThreadPool,
  tx: mpsc::Sender<usize>,
  rx: mpsc::Receiver<usize>,
  thread_finished: bool,
}

impl BattleScreen {
  pub fn new(window_size: Vector2<f32>) -> BattleScreen {
    let (tx, rx) = mpsc::channel();
    
    let solar_system: BoxArea = Box::new(SolarSystem::new(Vector2::new(-1500.0, 1500.0), Vector2::new(2000.0, 2000.0)));
    let astroid_field: BoxArea = Box::new(AstroidField::new(Vector2::new(1500.0, -1500.0), Vector2::new(500.0, 1000.0)));
    
    BattleScreen {
      data: SceneData::new(window_size, Vec::new()),
      areas: vec!(solar_system,astroid_field),
      input: player::Input::new(),
      ship: Box::new(Ship::new()),
      buffs: Vec::new(),
      projectiles: Vec::new(),
      zoom: 0.75,
      camera: OrthoCamera::new(window_size.x, window_size.y),
      ability_ui: AbilityUi::new(),
      uis: vec!(Box::new(PauseUi::new(window_size))),
      escape_pressed_last_frame: false,
      spatial_hash: Arc::new(Mutex::new(SpatialHash::new(50.0))),
      pool: ThreadPool::new(5),
      tx,
      rx,
      thread_finished: false,
    }
  }
  
  pub fn recreate(window_size: Vector2<f32>, camera: OrthoCamera, areas: Vec<BoxArea>, ship: BoxEntity, buffs: Vec<BoxBuff>, projectiles: Vec<MutexProjectile>, zoom: f32) -> BattleScreen {
    let (tx, rx) = mpsc::channel();
    
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
      spatial_hash: Arc::new(Mutex::new(SpatialHash::new(50.0))),
      pool: ThreadPool::new(5),
      tx,
      rx,
      thread_finished: false,
    }
  }
  
  pub fn new_collision_thread(&mut self, mutex_entity: Vec<MutexEntity>) {
   /* self.thread_finished = false;
    
    let (mutex_all_entities, mutex_projectiles, mutex_spatial_hash, tx) = (mutex_entity.clone(), self.projectiles.clone(), self.spatial_hash.clone(), self.tx.clone());
    
    self.pool.execute(move || {
      let mut spatial_hash = mutex_spatial_hash.lock().unwrap();
      
      for entity in mutex_all_entities {
        spatial_hash.insert_object_for_point(Arc::clone(&entity));
      }
      
      for i in 0..mutex_projectiles.len() {
        let mut projectile = mutex_projectiles[i].lock().unwrap();
        if projectile.should_exist() {
          // player collision
          if projectile.can_hit(self.ship.hostility()) {
            if self.ship.should_exist() {
              projectile.collide_with(&mut self.ship);
            }
          }
          
          // enemy collision 
          let mut enemies = spatial_hash.retrieve_objects(&*projectile);
          for enemy_mutex in &mut enemies {
            if !projectile.should_exist() {
              break;
            }
            
            let mut enemy = enemy_mutex.lock().unwrap();
            if projectile.can_hit(enemy.hostility()) {
              if enemy.should_exist() {
                projectile.collide_with(&mut *enemy);
              }
            }
          }
        }
      }
      
      spatial_hash.clear();
    
      tx.send(1).unwrap();
    });*/
  }
  
  pub fn check_collision_thread(&mut self) {
    /*match self.rx.try_recv() {
      Ok(i) => {
        self.thread_finished = true;
      },
      Err(_e) => { },
    }*/
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
      self.projectiles.push(Arc::new(Mutex::new(new_projectile)));
    }
    
    offset = 0;
    for i in 0..self.projectiles.len() {
      if i < offset {
        break;
      }
      
      let mut projectile_should_exist = true;
      {
        let mut projectile = self.projectiles[i-offset].lock().unwrap();
        //self.projectiles[i-offset].update(delta_time);
        projectile.update(delta_time);
        projectile_should_exist = projectile.should_exist();
      }
      if !projectile_should_exist {
        self.projectiles.remove(i-offset);
        offset += 1;
      }
    }
    
    // Check collisions 
    
    // Spatial Collisions
    
    let mut all_entities: Vec<MutexEntity> = Vec::new();
    for area in &mut self.areas {
      for entity in &mut area.entities().into_iter() {
        all_entities.push(Arc::clone(&entity));
      }
    }
    /*
    self.check_collision_thread();
    if self.thread_finished {
      self.new_collision_thread(all_entities);
    }*/
    
    let mut spatial_hash = self.spatial_hash.lock().unwrap();
    for entity in all_entities {
      spatial_hash.insert_object_for_point(Arc::clone(&entity));
    }
    
    for i in 0..self.projectiles.len() {
      let mut projectile = self.projectiles[i].lock().unwrap();
      if projectile.should_exist() {
        // player collision
        if projectile.can_hit(self.ship.hostility()) {
          if self.ship.should_exist() {
            projectile.collide_with(&mut self.ship);
          }
        }
        
        // enemy collision 
        let mut enemies = spatial_hash.retrieve_objects(&*projectile);
        for enemy_mutex in &mut enemies {
          if !projectile.should_exist() {
            break;
          }
          
          let mut enemy = enemy_mutex.lock().unwrap();
          if projectile.can_hit(enemy.hostility()) {
            if enemy.should_exist() {
              projectile.collide_with(&mut *enemy);
            }
          }
        }
      }
    }
    
    spatial_hash.clear();
    
    
    /*
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
    }*/
    
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
    
    for mutex_projectile in &self.projectiles {
      let mut projectile = mutex_projectile.lock().unwrap();
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
