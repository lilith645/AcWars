use maat_graphics::DrawCall;
use maat_graphics::Settings;

use crate::modules::ui::{Ui, UiData};
use maat_gui::widgets::{Widget, Image, Text, Button, CheckBox};

use crate::cgmath::{Vector2, Vector4};

const BUTTON_WIDTH: f32 = 120.0;
const BUTTON_HEIGHT: f32 = 40.0;

pub struct OptionsUi {
  data: UiData,
  settings: Settings,
}

enum WidgetIndex {
  Background,
  Vsync,
  VsyncText,
  Fullscreen,
  FullscreenText,
  Save,
  SaveText,
  Return,
  ReturnText,
}

impl WidgetIndex {
  pub fn n(self) -> usize {
    self as usize
  }
}

impl OptionsUi {
  pub fn new(window_size: Vector2<f32>) -> OptionsUi {
    let iwindow_size = Vector2::new(window_size.x as i32, window_size.y as i32);
    let settings = Settings::load(iwindow_size, iwindow_size);
    let vsync_setting = settings.vsync_enabled();
    let fullscreen_setting = settings.is_fullscreen();
    
    let menu_width = window_size.x*0.8;
    let menu_height = window_size.y*0.8;
    
    let button_colour = Vector4::new(0.0, 0.0, 0.0, 1.0);
    let font =  "Arial".to_string();
    let positions = OptionsUi::realign_widget_positions(window_size, Vector2::new(menu_width, menu_height));
    
    
    let background = Box::new(Image::new(positions[WidgetIndex::Background.n()], Vector2::new(menu_width, menu_height))
                                     .with_colour(Vector4::new(0.0, 0.0, 0.4, 1.0)));
    
    let return_button_position = positions[WidgetIndex::Return.n()];
    let return_text_position = positions[WidgetIndex::ReturnText.n()];
    let return_text: Box<Widget> = Box::new(Text::new(return_text_position, 64.0, &font, &"Return".to_string()).center_text());
    let return_button = Box::new(Button::new(return_button_position, Vector2::new(BUTTON_WIDTH, BUTTON_HEIGHT))
                                     .with_colour(button_colour));
    
    let save_button_position = positions[WidgetIndex::Save.n()];
    let save_text_position = positions[WidgetIndex::SaveText.n()];
    let save_text: Box<Widget> = Box::new(Text::new(save_text_position, 64.0, &font, &"Save".to_string()).center_text());
    let save_button = Box::new(Button::new(save_button_position, Vector2::new(BUTTON_WIDTH, BUTTON_HEIGHT))
                                     .with_colour(button_colour));
    
    let vsync_checkbox_position = positions[WidgetIndex::Vsync.n()];
    let vsync_text_position = positions[WidgetIndex::VsyncText.n()];
    let mut vsync = Box::new(CheckBox::new(vsync_checkbox_position, Vector2::new(50.0, 50.0))
                                  .with_colour(Vector4::new(0.0, 0.4, 0.0, 1.0)));
    let vsync_text = Box::new(Text::new(vsync_text_position, 128.0, &font, &"Vsync".to_string()));
    
    let fullscreen_checkbox_position = positions[WidgetIndex::Fullscreen.n()];
    let fullscreen_text_position = positions[WidgetIndex::FullscreenText.n()];
    let mut fullscreen = Box::new(CheckBox::new(fullscreen_checkbox_position, Vector2::new(50.0, 50.0))
                                  .with_colour(Vector4::new(0.0, 0.4, 0.0, 1.0)));
    let fullscreen_text = Box::new(Text::new(fullscreen_text_position, 128.0, &font, &"Fullscreen".to_string()));
    
    if vsync_setting {
      vsync.activate();
    }
    
    if fullscreen_setting {
      fullscreen.activate();
    }
    
    OptionsUi {
      data: UiData::new()
                    .with_widget(background)
                    .with_widget(vsync)
                    .with_widget(vsync_text)
                    .with_widget(fullscreen)
                    .with_widget(fullscreen_text)
                    .with_widget(save_button)
                    .with_widget(save_text)
                    .with_widget(return_button)
                    .with_widget(return_text),
      settings,
    }
  }
  
  fn realign_widget_positions(window_size: Vector2<f32>, menu_size: Vector2<f32>) -> Vec<Vector2<f32>> {
    let background_position = window_size*0.5;
    let return_position = background_position-menu_size*0.5+Vector2::new(BUTTON_WIDTH*0.5,
                                                                         BUTTON_HEIGHT*0.5)
                                                           +Vector2::new(25.0, 25.0);
    let return_text = return_position;
    
    let save_position = background_position+
                          Vector2::new(menu_size.x*0.5, -menu_size.y*0.5) + 
                          Vector2::new(-BUTTON_WIDTH*0.5, BUTTON_HEIGHT*0.5) +
                          Vector2::new(-25.0, 25.0);
    let save_text = save_position;
    
    let vsync_text = background_position + Vector2::new(-menu_size.x*0.5, menu_size.y*0.5) + Vector2::new(50.0, -100.0);
    let vsync = vsync_text + Vector2::new(200.0, 15.0);
    
    let fullscreen_text = background_position + Vector2::new(-menu_size.x*0.5, menu_size.y*0.5) + Vector2::new(50.0, -200.0);
    let fullscreen = fullscreen_text + Vector2::new(200.0, 15.0);
    
    // Backgound pos, resume pos, resume text pos, options pos, options text pos, quit pos, quit text pos
    vec!(background_position, vsync, vsync_text, fullscreen, fullscreen_text, save_position, save_text, return_position, return_text)
  }
}

impl Ui for OptionsUi {
  fn data(&self) -> &UiData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut UiData {
    &mut self.data
  }
  
  fn check_if_needs_reenabling(&mut self) {
    
  }
  
  fn update_ui(&mut self, mouse_pos: Vector2<f32>, left_mouse: bool, escape_pressed: bool, window_size: Vector2<f32>, should_close: &mut bool, _delta_time: f32) {
    let new_positions = OptionsUi::realign_widget_positions(window_size,
                                                          self.data().widgets[WidgetIndex::Background.n()].size());
    for i in 0..new_positions.len() {
      self.mut_data().widgets[i].set_position(new_positions[i]);
    }
    
    if escape_pressed || self.data().widgets[WidgetIndex::Return.n()].pressed() {
      self.disable();
    }
    
    if self.data().widgets[WidgetIndex::Save.n()].pressed() {
      self.settings.set_vsync(self.data().widgets[WidgetIndex::Vsync.n()].activated());
      self.settings.enable_fullscreen(self.data().widgets[WidgetIndex::Fullscreen.n()].activated());
      self.settings.save();
      println!("settings svaed");
    }
  }
}


