use maat_graphics::{DrawCall, math};
use maat_graphics::camera::OrthoCamera;
use maat_graphics::imgui;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;

use crate::modules::buffs::{BoxBuff};
use crate::modules::entities::{MutexEntity, BoxEntity, Ship};
use crate::modules::projectiles::{MutexProjectile, BoxProjectile};
use crate::modules::areas::{BoxArea, BenchmarkArea};
use crate::modules::player;
use crate::modules::ui::{BoxUi, PauseUi, AbilityUi, ShipModuleViewer};

use crate::cgmath::{Vector2, Vector4};

use hlua::Lua;

use parking_lot::Mutex;
use std::sync::Arc;

use crate::modules::spatial_hash::SpatialHash;
use crate::modules::collisions;
use maat_graphics::ThreadPool;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::channel;

use crate::modules::kdtree::Node;

enum UiIndex {
  AbilityUi,
  ModuleViewer,
  PauseUi,
}

impl UiIndex {
  pub fn n(self) -> usize {
    self as usize
  }
}

pub struct BenchmarkScreen {
  data: SceneData,
  areas: Vec<BoxArea>,
  input: Arc<Mutex<player::Input>>,
  ship: MutexEntity,
  buffs: Vec<BoxBuff>,
  projectiles: Vec<MutexProjectile>,
  zoom: f32,
  camera: OrthoCamera,
  uis: Vec<BoxUi>,
  escape_pressed_last_frame: bool, 
  i_pressed_last_frame: bool, 
  spatial_hash: Arc<Mutex<SpatialHash>>,
  collision_checks: u64,
  _kdtree: Option<Box<Node>>,
  total_delta_time: f32,
  virtual_input: Vec<f32>,
  all_fps: Vec<f64>,
  mouse_angle: f32,
  thread_pool: ThreadPool,
  tx: mpsc::Sender<()>,
  rx: mpsc::Receiver<()>,
}

impl BenchmarkScreen {
  pub fn new(window_size: Vector2<f32>) -> BenchmarkScreen {
    let benchmark: BoxArea = Box::new(BenchmarkArea::new(Vector2::new(0.0, 0.0), Vector2::new(20000.0, 20000.0), 10));
    
    let player_input = Arc::new(Mutex::new(player::Input::new()));
    
    let ship: MutexEntity = Arc::new(Mutex::new(Box::new(Ship::new(Vector2::new(0.0, 0.0)).with_health(1500000000.0))));
    
    let ability_ui = AbilityUi::new(Arc::clone(&player_input), window_size);
    
    let mut module_viewer = ShipModuleViewer::new(window_size, &ship);
    
    let (tx, rx) = mpsc::channel();
    let thread_pool = ThreadPool::new(5);
    let fake_tx = tx.clone();
     thread_pool.execute(move || {
          fake_tx.send(()).unwrap();
    });
    
    BenchmarkScreen {
      data: SceneData::new(window_size, Vec::new()),
      areas: vec!(benchmark),
      input: player_input,
      ship,
      buffs: Vec::new(),
      projectiles: Vec::new(),
      zoom: 0.75,
      camera: OrthoCamera::new(window_size.x, window_size.y),
      uis: vec!(Box::new(ability_ui), Box::new(module_viewer), Box::new(PauseUi::new(window_size))),
      escape_pressed_last_frame: false,
      i_pressed_last_frame: false, 
      spatial_hash: Arc::new(Mutex::new(SpatialHash::new(100.0))),
      collision_checks: 0,
      _kdtree: None,
      total_delta_time: 0.0,
      virtual_input: (0..100).into_iter().map(|x| x as f32*0.5).collect::<Vec<f32>>(),
      all_fps: Vec::new(),
      mouse_angle: 0.0,
      thread_pool,
      tx,
      rx,
    }
  }
  
  pub fn recreate(window_size: Vector2<f32>, camera: OrthoCamera, areas: Vec<BoxArea>, ship: MutexEntity, input: Arc<Mutex<player::Input>>, buffs: Vec<BoxBuff>, projectiles: Vec<MutexProjectile>, zoom: f32, all_fps: Vec<f64>, mouse_angle: f32) -> BenchmarkScreen {
    let (tx, rx) = mpsc::channel();
    BenchmarkScreen {
      data: SceneData::new(window_size, Vec::new()), 
      areas,
      input,
      ship,
      buffs,
      projectiles,
      zoom,
      camera,
      uis: vec!(Box::new(PauseUi::new(window_size))),
      escape_pressed_last_frame: false,
      i_pressed_last_frame: false,
      spatial_hash: Arc::new(Mutex::new(SpatialHash::new(100.0))),
      collision_checks: 0,
      _kdtree: None,
      total_delta_time: 0.0,
      virtual_input: (0..100).into_iter().map(|x| x as f32*0.5).collect::<Vec<f32>>(),
      all_fps,
      mouse_angle,
      thread_pool: ThreadPool::new(5),
      tx,
      rx,
    }
  }
  
  pub fn update_pause(&mut self, dim: Vector2<f32>, escape_pressed: bool, delta_time: f32) -> bool {
    let mouse_pos = self.data().mouse_pos;
    let left_mouse = self.data().left_mouse;
    let scroll_delta = self.data().scroll_delta;
    
    // Pause Ui
    if escape_pressed {
      if self.uis[UiIndex::PauseUi.n()].enabled() {
        self.uis[UiIndex::PauseUi.n()].disable();
      } else {
        self.uis[UiIndex::PauseUi.n()].enable();
      }
    }
    
    let mut should_close = false;
    let mut should_resize = None;
    let mut should_next_scene = false;
    self.uis[UiIndex::PauseUi.n()].update(mouse_pos, left_mouse, escape_pressed, dim, &mut should_close, 
                                          &mut should_resize, &mut should_next_scene, scroll_delta, delta_time);
    if should_resize.is_some() {
      self.mut_data().should_resize_window = should_resize;
    }
    if should_close {
      self.mut_data().should_close = true;
    }
      
    if should_next_scene {
      self.mut_data().next_scene = true;
      self.mut_data().window_resized = false;
    }
    
    self.uis[UiIndex::PauseUi.n()].enabled()
  }
  
  pub fn update_ui(&mut self, dim: Vector2<f32>, escape_pressed: bool, i_pressed: bool, delta_time: f32) {
    let mouse_pos = self.data().mouse_pos;
    let left_mouse = self.data().left_mouse;
    let scroll_delta = self.data().scroll_delta;
    
    // Module Viewer Ui
    if i_pressed {
      if self.uis[UiIndex::ModuleViewer.n()].enabled() {
        self.uis[UiIndex::ModuleViewer.n()].disable();
      } else {
        self.uis[UiIndex::ModuleViewer.n()].enable();
      }
    }
    
    // UI
    let mut should_close = false;
    let mut should_resize = None;
    let mut should_next_scene = false;
    for i in 0..self.uis.len() {
      if i == UiIndex::PauseUi.n() {
        continue;
      }
      
      self.uis[i].update(mouse_pos, left_mouse, escape_pressed, dim, &mut should_close, &mut should_resize, 
                         &mut should_next_scene, scroll_delta, delta_time);
    }
    
    if should_resize.is_some() {
      self.mut_data().should_resize_window = should_resize;
    }
    
    if should_close {
      self.mut_data().should_close = true;
    }
      
    if should_next_scene || { let ship = self.ship.lock(); !ship.should_exist() } {
      self.mut_data().next_scene = true;
    }
    
    
    if self.uis[UiIndex::PauseUi.n()].enabled() {
      return;
    }
  }
  
  pub fn update_player(&mut self, dim: Vector2<f32>, delta_time: f32) -> Vec<BoxProjectile> {
    // Player
    let left_stick_position =  self.data().controller.left_stick_position();
    let xbox_a_button = self.data().controller.a_button_pressed();
    let right_trigger_pressed = self.data().controller.right_trigger_pressed();
    
    let mouse_pos = self.data().mouse_pos;
    let left_mouse = self.data.left_mouse;
    let middle_mouse = self.data.middle_mouse;
    let right_mouse = self.data.right_mouse;
    
    let q_pressed = self.data.keys.q_pressed();
    let w_pressed = self.data.keys.w_pressed();
    let e_pressed = self.data.keys.e_pressed();
    let r_pressed = self.data.keys.r_pressed();
    
    let mut mouse_pos = self.data.mouse_pos;
    
    self.total_delta_time += delta_time;
    
    let mut step_number = 0;
    for i in 0..self.virtual_input.len() {
      if self.total_delta_time > self.virtual_input[i] {
        step_number += 1;
      }
    }
    
    if step_number <= 50 {
      self.all_fps.push(self.data().fps_last_frame);
      self.all_fps.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }
    
    let scroll_delta = self.data.scroll_delta;
    let mut left_mouse = self.data.left_mouse;
    let middle_mouse = self.data.middle_mouse;
    let mut right_mouse = self.data.right_mouse;
    let mut q_pressed = self.data.keys.q_pressed();
    let escape_pressed = self.data.keys.escape_pressed() && !self.escape_pressed_last_frame;
    
    self.mouse_angle += 60.0*delta_time;
    
    match step_number {
      //shoot
      0..=3 | 37 |  8 | 9 | 31 | 39 | 10 | 11 | 33 | 12 | 13 | 35 | 32 | 38 | 34 | 36 => {
        mouse_pos = dim*0.5+Vector2::new(100.0*(math::to_radians(self.mouse_angle)).cos(), 
                                         100.0*(math::to_radians(self.mouse_angle)).sin());
        right_mouse = true;
      },
      // dash
      40 => {
        mouse_pos = dim*0.5+Vector2::new(-10.0, 50.0);
        q_pressed = true;
      },
      // reverse dash
      50 => {
        mouse_pos = dim*0.5+Vector2::new(10.0, -50.0);
        q_pressed = true;
      },
      // idle
      4..=7 | 13..=30 | 
      42..=49 => {
          mouse_pos = dim*0.5+Vector2::new(100.0*(math::to_radians(self.mouse_angle)).cos(), 
                                         100.0*(math::to_radians(self.mouse_angle)).sin());
          right_mouse = false;
          left_mouse = false;
      },
      _ => {},
    }
    
    
    let mut ship = self.ship.lock();
    let mut player_input = self.input.lock();
    player_input.update(&mut *ship, left_stick_position, xbox_a_button, right_trigger_pressed, 
                      mouse_pos, left_mouse, middle_mouse, right_mouse, q_pressed, w_pressed,
                      e_pressed, r_pressed, dim, delta_time);
    
    let (buffs, new_projectiles) = ship.update(delta_time);
    
    let mut offset = 0;
    for i in 0..self.buffs.len() {
      self.buffs[i-offset].update(&mut *ship, delta_time);
      if !self.buffs[i-offset].should_exist() {
        self.buffs[i-offset].unapply_buff(&mut *ship);
        self.buffs.remove(i-offset);
        offset += 1;
      }
    }
    
    for buff in buffs {
      buff.apply_buff(&mut *ship);
      self.buffs.push(buff);
    }
    
    new_projectiles
  }
  
  pub fn update_areas(&mut self, dim: Vector2<f32>, delta_time: f32) -> Vec<BoxProjectile> {
    let mut new_projectiles = Vec::new();
    
    let mut ship = self.ship.lock();
    for area in &mut self.areas {
      let projectiles = area.update(&mut *ship, dim, delta_time);
      for projectile in projectiles {
        new_projectiles.push(projectile);
      }
    }
    
    new_projectiles
  }
  
  pub fn update_projectiles(&mut self, player_projectiles: Vec<BoxProjectile>, entity_projectiles: Vec<BoxProjectile>, delta_time: f32) {
    // Projectiles 
    for new_projectile in player_projectiles {
      self.projectiles.push(Arc::new(Mutex::new(new_projectile)));
    }
    
    for new_projectile in entity_projectiles {
      self.projectiles.push(Arc::new(Mutex::new(new_projectile)));
    }
    
    let mut offset = 0;
    for i in 0..self.projectiles.len() {
      if i < offset {
        break;
      }
      
      let projectile_should_exist;
      {
        let mut projectile = self.projectiles[i-offset].lock();
        projectile.update(delta_time);
        projectile_should_exist = projectile.should_exist();
      }
      if !projectile_should_exist {
        self.projectiles.remove(i-offset);
        offset += 1;
      }
    }
  }
  
  pub fn update_camera(&mut self, dim: Vector2<f32>) {
    let ship_pos = {let ship = self.ship.lock(); ship.position() };
    self.camera.window_resized(dim.x, dim.y);
    let camera_target = ship_pos*self.zoom - Vector2::new(dim.x*0.5, dim.y*0.5);
    self.camera.lerp_to_position(camera_target,  Vector2::new(0.05, 0.05));
  }
  /*
  pub fn kdtree_collision(&mut self, all_entities: Vec<MutexEntity>) {
    let mut all_positions: Vec<Vector2<f32>> = Vec::with_capacity(10000);
    
    for mutex_entity in &all_entities {
      let entity = mutex_entity.lock();
      all_positions.push(entity.position());
    }
    
    self.kdtree = Node::create_kdtree(all_positions, 2, 0, 3);
    
    if let Some(kdtree) = &self.kdtree {
      let boundry = kdtree.get_boundries();
      for i in 0..self.projectiles.len() {
        let mut projectile = self.projectiles[i].lock();
        let projectile_pos = projectile.position();
        if projectile_pos.x < boundry.x && projectile_pos.y < boundry.y || projectile_pos.x > boundry.z && projectile_pos.y < boundry.w  {
          continue;
        }
        
        let projectile_index = kdtree.get_depth_index(projectile.position(), 0, (0,0));
        
        if projectile.should_exist() {
          // player collision
          if projectile.can_hit(self.ship.hostility()) {
            let ship_index = kdtree.get_depth_index(self.ship.position(), 0, (0,0));
            if projectile_index == ship_index {
              if self.ship.should_exist() {
                self.collision_checks += 1;
                projectile.collide_with(&mut self.ship);
              }
            }
          }
          
          // enemy collision 
          for enemy_mutex in &all_entities {
            if !projectile.should_exist() {
              break;
            }
            
            let mut enemy = enemy_mutex.lock();
            let enemy_index = kdtree.get_depth_index(enemy.position(), 0, (0,0));
            if enemy_index == projectile_index {
              if projectile.can_hit(enemy.hostility()) {
                if enemy.should_exist() {
                  self.collision_checks += 1;
                  projectile.collide_with(&mut *enemy);
                }
              }
            }
          }
        }
      }
    }
  }
  
  pub fn spatial_hash_collision_projectiles(&mut self, all_entities: Vec<MutexEntity>, all_projectiles: Vec<MutexProjectile>) {
    let mut spatial_hash = self.spatial_hash.lock();
    for projectile in all_projectiles {
      spatial_hash.insert_object_for_point(Arc::clone(&projectile));
    }
    
    // player collision
    if self.ship.should_exist() {
      let mut projectiles = spatial_hash.retrieve_objects(&self.ship);
      for mutex_projectile in &projectiles {
        let mut projectile = mutex_projectile.lock();
        if projectile.should_exist() {
          if projectile.can_hit(self.ship.hostility()) {
            self.collision_checks += 1;
            projectile.collide_with(&mut self.ship);
          }
        }
      }
    }
    
    for i in 0..all_entities.len() {
      let mut entity = all_entities[i].lock();
      if entity.should_exist() {
        // enemy collision 
        let mut projectiles = spatial_hash.retrieve_objects(&*entity);
        for mutex_projectile in &projectiles {
          let mut projectile = mutex_projectile.lock();
          if !projectile.should_exist() {
            continue;
          }
          
          if projectile.can_hit(entity.hostility()) {
            self.collision_checks += 1;
            projectile.collide_with(&mut *entity);
          }
        }
      }
    }
    
    spatial_hash.clear();
  }*/
  
  pub fn spatial_hash_collision(&self) {
    /*
    let _all_entities: Vec<MutexEntity> = Vec::new();
    
    let mut spatial_hash = self.spatial_hash.lock();
    spatial_hash.clear();
    for area in &self.areas {
      for mutex_entity in &area.entities() {
        spatial_hash.insert_object_for_point(Arc::clone(&mutex_entity));
      }
    }
    spatial_hash.insert_object_for_point(Arc::clone(&self.ship));
    
    for i in 0..self.projectiles.len() {
      let mut projectile = self.projectiles[i].lock();
      if projectile.should_exist() {
        // entity collision 
        let mut entities = spatial_hash.retrieve_objects(&*projectile);
        for entity_mutex in &mut entities {
          if !projectile.should_exist() {
            break;
          }
          
          let mut entity = entity_mutex.lock();
          if entity.should_exist() {
            if projectile.can_hit(entity.hostility()) {
              projectile.collide_with(&mut *entity);
            }
          }
        }
      }
    }
    
    let entity_groups = spatial_hash.retrieve_possible_entity_collisions();
    for group in &entity_groups {
      for i in 0..group.len() {
        for j in i..group.len() {
          if i == j {
            continue;
          }
          let mut entity_one = group[i].lock();
          let mut entity_two = group[j].lock();
          
          if entity_one.should_exist() && entity_two.should_exist() {
            entity_one.collide_with(&mut *entity_two);
            entity_two.collide_with(&mut *entity_one);
          }
        }
      }
    }
    
    spatial_hash.clear();*/
  }
  /*
  pub fn brute_force_collision(&mut self, all_entities: Vec<MutexEntity>) {
    for i in 0..self.projectiles.len() {
      let mut projectile = self.projectiles[i].lock();
      if projectile.should_exist() {
        // player collision
        if projectile.can_hit(self.ship.hostility()) {
          if self.ship.should_exist() {
            self.collision_checks += 1;
            projectile.collide_with(&mut self.ship);
          }
        }
        
        // enemy collision 
        for enemy_mutex in &all_entities {
          if !projectile.should_exist() {
            break;
          }
          
          let mut enemy = enemy_mutex.lock();
          if projectile.can_hit(enemy.hostility()) {
            if enemy.should_exist() {
              self.collision_checks += 1;
              projectile.collide_with(&mut *enemy);
            }
          }
        }
      }
    }
  }*/
}

impl Scene for BenchmarkScreen {
  fn data(&self) -> &SceneData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut SceneData {
    &mut self.data
  }
  
  fn future_scene(&mut self, window_size: Vector2<f32>) -> Box<Scene> {
    Box::new(BenchmarkScreen::recreate(window_size, self.camera.clone(), self.areas.clone(), self.ship.clone(), self.input.clone(), self.buffs.clone(), self.projectiles.clone(), self.zoom, self.all_fps.clone(), self.mouse_angle))
  }
  
  fn update(&mut self, _ui: Option<&imgui::Ui>, _lua: Option<&mut Lua>, delta_time: f32) {
    self.mut_data().controller.update();
    
    let dim = self.data().window_dim;
    
    self.mut_data().controller.update();
    
    let dim = self.data().window_dim;
    
    let escape_pressed = self.data.keys.escape_pressed() && !self.escape_pressed_last_frame;
    let i_pressed = self.data.keys.i_pressed() && !self.i_pressed_last_frame;
    
    self.escape_pressed_last_frame = self.data().keys.escape_pressed();
    self.i_pressed_last_frame = self.data().keys.i_pressed();
    
    self.update_camera(dim);
    
    if self.update_pause(dim, escape_pressed, delta_time) {
      return;
    }
    
    self.update_ui(dim, escape_pressed, i_pressed, delta_time);
    
    let player_projectiles = self.update_player(dim, delta_time);
    let entity_projectiles = self.update_areas(dim, delta_time);
    self.update_projectiles(player_projectiles, entity_projectiles, delta_time);
    
    if self.rx.try_recv().is_ok() {
      let mut entities: Vec<MutexEntity> = Vec::new();
      let mut projectiles: Vec<MutexProjectile> = Vec::new();
      for area in &self.areas {
        for mutex_entity in &area.entities() {
          entities.push(Arc::clone(&mutex_entity));
        }
      }
      entities.push(Arc::clone(&self.ship));
      
      for i in 0..self.projectiles.len() {
        projectiles.push(Arc::clone(&self.projectiles[i]));
      }
      
      let tx = self.tx.clone();
      self.thread_pool.execute(move || {
        collisions::collisions(entities, projectiles);
        tx.send(()).unwrap();
      });
    }
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let dim = self.data().window_dim;
    let (width, height) = (dim.x as f32, dim.y as f32);
    
    draw_calls.push(DrawCall::set_texture_scale(self.zoom));
    
    draw_calls.push(DrawCall::replace_ortho_camera(self.camera.clone()));
    
    let bg_width = 1920.0;
    let bg_height = 1080.0;
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
      let projectile = mutex_projectile.lock();
      projectile.draw(draw_calls);
    }
    
    for area in &self.areas {
      area.draw(draw_calls);
    }
    
    let ship = self.ship.lock();
    ship.draw(draw_calls);
    
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
    
    ship.draw_ship_ui(draw_calls);
    
    draw_calls.push(DrawCall::set_texture_scale(1.0));
    draw_calls.push(DrawCall::reset_ortho_camera());
    
    for ui in &self.uis {
      ui.draw(draw_calls);
    }
    
    if self.total_delta_time > 25.0 {
      let mut lowest_fps = {
        let mut temp_low = 0.0;
        let mut i = 0;
        while temp_low == 0.0 {
          if i >= self.all_fps.len() {
            break;
          }
          temp_low = self.all_fps[i];
          i+=1;
        }
        
        temp_low.to_string()
      };
      let mut highest_fps = self.all_fps[self.all_fps.len()-1].to_string();
      let median_index = (self.all_fps.len() as f32*0.5).floor() as usize;
      let  mut fps = self.all_fps[median_index].to_string();
      fps.truncate(6);
      lowest_fps.truncate(6);
      highest_fps.truncate(6);
      draw_calls.push(DrawCall::draw_text_basic_centered(Vector2::new(width*0.5, height*0.5+50.0), 
                                             Vector2::new(128.0, 128.0), 
                                             Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                             "Lowest Benckmark fps: ".to_string() + &lowest_fps, 
                                             "Arial".to_string()));
      draw_calls.push(DrawCall::draw_text_basic_centered(Vector2::new(width*0.5, height*0.5), 
                                             Vector2::new(128.0, 128.0), 
                                             Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                             "Average Benckmark fps: ".to_string() + &fps, 
                                             "Arial".to_string()));
      draw_calls.push(DrawCall::draw_text_basic_centered(Vector2::new(width*0.5, height*0.5-50.0), 
                                             Vector2::new(128.0, 128.0), 
                                             Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                             "Highest Benckmark fps: ".to_string() + &highest_fps, 
                                             "Arial".to_string()));
    }
  }
}

    /*
      let min_width = 0.0;
      let min_height = 0.0;
      let max_width = 19200.0*0.5;
      let max_height = 19200.0*0.5;
      let colour = crate::cgmath::Vector4::new(1.0, 1.0, 1.0, 1.0);
      draw_calls.push(DrawCall::draw_coloured(Vector2::new(max_width*0.5, max_height), Vector2::new(max_width, 10.0), colour, 0.0));
      draw_calls.push(DrawCall::draw_coloured(Vector2::new(max_width*0.5, min_height), Vector2::new(max_width, 10.0), colour, 0.0));
      
      draw_calls.push(DrawCall::draw_coloured(Vector2::new(max_width, max_height*0.5), Vector2::new(10.0, max_height), colour, 0.0));
      draw_calls.push(DrawCall::draw_coloured(Vector2::new(min_width, max_height*0.5), Vector2::new(10.0, max_height), colour, 0.0));
      Node::draw_kdtree(self.kdtree.clone(), 0, draw_calls, max_height, max_width, max_height);*/
