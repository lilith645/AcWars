use maat_graphics::DrawCall;
use maat_graphics::imgui::*;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;

use crate::modules::Ship;
use crate::modules::projectiles::Projectile;
use crate::modules::projectiles::Ftpl;
use crate::modules::player;

use hlua::Lua;

use cgmath::{Vector2, Vector4};

pub struct BattleScreen {
  data: SceneData,
  input: player::Input,
  ship: Ship,
  projectiles: Vec<Box<Projectile>>,
}

impl BattleScreen {
  pub fn new(window_size: Vector2<f32>) -> BattleScreen {
    BattleScreen {
      data: SceneData::new(window_size, Vec::new()),
      input: player::Input::new(),
      ship: Ship::new(),
      projectiles: Vec::new(),
    }
  }
  
  pub fn recreate(window_size: Vector2<f32>, ship: Ship, projectiles: Vec<Box<Projectile>>) -> BattleScreen {
    BattleScreen {
      data: SceneData::new(window_size, Vec::new()), 
      input: player::Input::new(),
      ship,
      projectiles,
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
    Box::new(BattleScreen::recreate(window_size, self.ship.clone(), self.projectiles.clone()))
  }
  
  fn update(&mut self, _ui: Option<&Ui>, _lua: Option<&mut Lua>, delta_time: f32) {
    let dim = self.data().window_dim;
    let mouse_pos = self.data.mouse_pos;
    
    let left_mouse = self.data.left_mouse;
    let middle_mouse = self.data.middle_mouse;
    let right_mouse = self.data.right_mouse;
    
    self.input.update(&mut self.ship, mouse_pos, left_mouse, middle_mouse, right_mouse, dim, delta_time);
    
    let new_projectiles = self.ship.update(delta_time);
    
    for new_projectile in new_projectiles {
      self.projectiles.push(new_projectile);
    }
    
    for projectile in &mut self.projectiles {
      projectile.update(delta_time);
    }
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let dim = self.data().window_dim;
    let (width, height) = (dim.x as f32, dim.y as f32);
    
    let camera_target = self.ship.position() - Vector2::new(width*0.5, height*0.5);
    draw_calls.push(DrawCall::lerp_ortho_camera_to_pos(camera_target, Vector2::new(0.05, 0.05)));
    
    
    
    draw_calls.push(
        DrawCall::draw_textured(Vector2::new(width*0.5, height*0.5),
                                Vector2::new(width*1.0, height*1.0),
                                270.0,
                                "bg_space".to_string())
    );
    /*
    draw_calls.push(
      DrawCall::draw_textured(Vector2::new(200.0, 200.0), 
                              Vector2::new(50.0, 50.0),
                              90.0,
                              String::from("Bulbz"))
    );*/
    
    
    for projectile in &self.projectiles {
      projectile.draw(draw_calls);
    }
    
    draw_calls.push(DrawCall::draw_instanced("Ftpl".to_string(), "Ftpl".to_string()));
    
    self.ship.draw(draw_calls);
  }
}
