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

enum WidgetIndex {
  Background,
  Resume,
  ResumeText,
  Options,
  OptionsText,
  Quit,
  QuiText,
}

impl WidgetIndex {
  pub fn n(self) -> usize {
    self as usize
  }
}

pub struct PauseUi {
  data: UiData,
}

impl PauseUi {
  pub fn new(window_size: Vector2<f32>) -> PauseUi {
    let pause_width = 200.0;
    let pause_height = 300.0;
    
    let positions = PauseUi::realign_widget_positions(window_size, pause_height);
    
    let button_width = pause_width*0.6;
    let button_height = 40.0;
    let button_colour = Vector4::new(0.0, 0.0, 0.0, 1.0);
    let font =  "Arial".to_string();
    let background = Box::new(Image::new(positions[WidgetIndex::Background.n()], Vector2::new(pause_width, pause_height))
                                     .with_colour(Vector4::new(0.0, 0.0, 0.4, 1.0)));
    
    let resume_position = positions[WidgetIndex::Resume.n()];
    let resume_text: Box<Widget> = Box::new(Text::new(resume_position, 64.0, &font, &"Resume".to_string()).center_text());
    let resume = Box::new(Button::new(resume_position, Vector2::new(button_width, button_height))
                                     .with_colour(button_colour));
    
    let options_position = positions[WidgetIndex::Options.n()];
    let options_text: Box<Widget> = Box::new(Text::new(options_position, 64.0, &font, &"Options".to_string()).center_text());
    let options = Box::new(Button::new(options_position, Vector2::new(button_width, button_height))
                                     .with_colour(button_colour));
                                     
    let quit_position = positions[WidgetIndex::Quit.n()];
    let quit_text: Box<Widget> = Box::new(Text::new(quit_position, 64.0, &font, &"Quit".to_string()).center_text());
    let quit = Box::new(Button::new(quit_position, Vector2::new(button_width, button_height))
                                     .with_colour(button_colour));
    
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
  
  fn update_ui(&mut self, mouse_pos: Vector2<f32>, left_mouse: bool, window_size: Vector2<f32>, mut should_close: &mut bool, _delta_time: f32) {
    
    let new_positions = PauseUi::realign_widget_positions(window_size,
                                                          self.data().widgets[WidgetIndex::Background.n()].size().y);
    for i in 0..new_positions.len() {
      self.mut_data().widgets[i].set_position(new_positions[i]);
    }
    
    if self.data().widgets[WidgetIndex::Resume.n()].pressed() {
      self.mut_data().enabled = false;
    }
    
    if self.data().widgets[WidgetIndex::Options.n()].pressed() {
      if let Some(ui) = &mut self.mut_data().uis {
        ui[UiIndex::OptionsUi.n()].enable();
      }
    }
    
    if self.data().widgets[WidgetIndex::Quit.n()].pressed() {
      *should_close = true;
    }
  }
}


