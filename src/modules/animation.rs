use maat_graphics::DrawCall;

use cgmath::{Vector2, Vector3};

#[derive(Clone)]
pub enum AnimationDirection {
  Forward,
  Backward,
  ForwardThenBackward,
  BackwardThenForward,
}

#[derive(Clone)]
pub struct Animation {
  sprite_rows: i32,
  timer: f32,
  current_frame: i32,
  total_frames: i32,
  total_time: f32,
  current_direction: AnimationDirection,
  direction: AnimationDirection,
}

impl Animation {
  pub fn new(sprite_rows: i32, timer: f32) -> Animation {
    Animation {
      sprite_rows,
      timer,
      current_frame: 0,
      total_frames: sprite_rows*sprite_rows,
      total_time: 0.0,
      current_direction: AnimationDirection::Forward,
      direction: AnimationDirection::Forward,
    }
  }
  
  pub fn with_total_frames(mut self, total_frames: i32) -> Animation {
    self.total_frames = total_frames;
    self
  }
  
  pub fn animate_backwards(mut self) -> Animation {
    self.direction = AnimationDirection::Backward;
    self
  }
  
  pub fn animate_forwards_then_backwards(mut self) -> Animation {
    self.direction = AnimationDirection::ForwardThenBackward;
    self
  }
  
  pub fn animate_backwards_then_forwards(mut self) -> Animation {
    self.direction = AnimationDirection::BackwardThenForward;
    self
  }
  
  pub fn update(&mut self, delta_time: f32) {
    self.total_time += delta_time;
    
    if self.total_time > self.timer {
      match self.current_direction { 
        AnimationDirection::Forward => {
          self.current_frame += 1;
        },
        AnimationDirection::Backward => {
          self.current_frame -= 1;
        },
        _ => {},
      }
      
      match self.direction {
        AnimationDirection::Forward => {
          if self.current_frame >= self.total_frames {
            self.current_frame = 0;
          }
        },
        AnimationDirection::Backward => {
          if self.current_frame <= 0 {
            self.current_frame = self.total_frames;
          }
        },
        AnimationDirection::ForwardThenBackward | AnimationDirection::BackwardThenForward => {
          if self.current_frame >= self.total_frames-1 {
            self.current_direction = AnimationDirection::Backward;
          }
          if self.current_frame <= 0 {
            self.current_direction = AnimationDirection::Forward;
          }
        }
      }
      
      self.total_time -= self.timer;
    }
  }
  
  pub fn draw(&self, position: Vector2<f32>, size: Vector2<f32>, rotation: f32, texture: String, draw_calls: &mut Vec<DrawCall>) {
    let sprite_sheet_rows = self.sprite_rows;
    
    let x = self.current_frame % sprite_sheet_rows;
    let mut y = 0;
    for i in 1..sprite_sheet_rows {
      if self.current_frame-x == sprite_sheet_rows*i {
        y+=1;
      }
    }
    
    draw_calls.push(DrawCall::add_instanced_sprite_sheet(position, size, 
                                                         rotation, texture.to_string(),
                                                         Vector3::new(x,y, sprite_sheet_rows)));
  }
}
