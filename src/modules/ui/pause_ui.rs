use maat_graphics::DrawCall;
use maat_gui::widgets::{Widget, Image, Button, Text};

use crate::modules::ui::{Ui, UiData, OptionsUi};

use crate::cgmath::{Vector2, Vector4};

enum UiIndex {
  OptionsUi,
}

impl UiIndex {
  pub fn n(self) -> usize {
    self as usize
  }
}


const BACKGROUND: usize = 0;
const RESUME: usize =  1;
//const RESUME_TEXT: usize = 2;
const OPTIONS: usize = 3;
//const OPTIONS_TEXT: usize = 4;
const QUIT: usize =  5;
//const QUIT_TEXT: usize =  6;

#[derive(Clone)]
pub struct PauseUi {
  data: UiData,
  options_ui_opened: bool,
}

impl PauseUi {
  pub fn new(window_size: Vector2<f32>) -> PauseUi {
    let pause_width = 200.0;
    let pause_height = 300.0;
    
    let positions = PauseUi::realign_widget_positions(window_size, pause_height);
    
    let button_width = pause_width*0.6;
    let button_height = 40.0;
    let button_colour = Vector4::new(0.8, 0.8, 0.8, 1.0);
    let font =  "Arial".to_string();
    let background = Box::new(Image::new(positions[BACKGROUND], Vector2::new(pause_width, pause_height))
                                     .with_primary_colour(Vector4::new(0.2, 0.2, 0.3, 1.0)));
    
    let resume_position = positions[RESUME];
    let resume_text: Box<Widget> = Box::new(Text::new(resume_position, 64.0, &font, &"Resume".to_string()).center_text());
    let resume = Box::new(Button::new(resume_position, Vector2::new(button_width, button_height))
                                     .with_primary_colour(button_colour));
    
    let options_position = positions[OPTIONS];
    let options_text: Box<Widget> = Box::new(Text::new(options_position, 64.0, &font, &"Options".to_string()).center_text());
    let options = Box::new(Button::new(options_position, Vector2::new(button_width, button_height))
                                     .with_primary_colour(button_colour));
                                     
    let quit_position = positions[QUIT];
    let quit_text: Box<Widget> = Box::new(Text::new(quit_position, 64.0, &font, &"Quit".to_string()).center_text());
    let quit = Box::new(Button::new(quit_position, Vector2::new(button_width, button_height))
                                     .with_primary_colour(button_colour));
    
    let mut options_ui: Box<Ui> = Box::new(OptionsUi::new(window_size));
    options_ui.disable();
    PauseUi {
      data: UiData::new()
                    .with_widget(background)
                    .with_widget(resume)
                    .with_widget(resume_text)
                    .with_widget(options)
                    .with_widget(options_text)
                    .with_widget(quit)
                    .with_widget(quit_text)
                    .with_ui(options_ui)
                    .disable(),
      options_ui_opened: false,
    }
  }
  
  fn realign_widget_positions(window_size: Vector2<f32>, pause_height: f32) -> Vec<Vector2<f32>> {
    let background_position = window_size*0.5;
    let options_position = window_size*0.5;
    let resume_position = window_size*0.5 + Vector2::new(0.0, pause_height*0.5*0.5);
    let quit_position = window_size*0.5 - Vector2::new(0.0, pause_height*0.5*0.5);
    
    // Backgound pos, resume pos, resume text pos, options pos, options text pos, quit pos, quit text pos
    vec!(background_position, resume_position, resume_position, options_position, options_position, quit_position, quit_position)
  }
}

impl Ui for PauseUi {
  fn data(&self) -> &UiData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut UiData {
    &mut self.data
  }
  
  fn check_if_needs_reenabling(&mut self) {
    if self.options_ui_opened {
      if let Some(uis) = &self.data().uis {
        if !uis[UiIndex::OptionsUi.n()].enabled() {
          self.enable();
          self.options_ui_opened = false;
        }
      }
    }
  }
  
  fn update_ui(&mut self, _mouse_pos: Vector2<f32>, _left_mouse: bool, _escape_pressed: bool, window_size: Vector2<f32>, _should_close: &mut bool, _should_resize: &mut Option<(Vector2<f32>, bool)>, should_next_scene: &mut bool, _delta_time: f32) {
    
    let new_positions = PauseUi::realign_widget_positions(window_size,
                                                          self.data().widgets[BACKGROUND].size().y);
    for i in 0..new_positions.len() {
      self.mut_data().widgets[i].set_position(new_positions[i]);
    }
    
    if self.data().widgets[RESUME].pressed() {
      self.mut_data().enabled = false;
    }
    
    if self.data().widgets[OPTIONS].pressed() {
      if let Some(ui) = &mut self.mut_data().uis {
        ui[UiIndex::OptionsUi.n()].enable();
        self.mut_data().enabled = false;
        self.options_ui_opened = true;
      }
    }
    
    if self.data().widgets[QUIT].pressed() {
      *should_next_scene = true;
    }
  }
  
  fn custom_draw(&self, _draw_calls: &mut Vec<DrawCall>) {
    
  }
}


