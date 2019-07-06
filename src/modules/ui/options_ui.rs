use maat_graphics::DrawCall;
use maat_graphics::Settings;

use crate::modules::ui::{Ui, UiData};
use maat_gui::widgets::{Widget, Image, Text, Button, CheckBox, DropdownBox};

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
  Msaa,
  MsaaText,
  Resolution,
  ResolutionText,
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
    let msaa = settings.get_texture_msaa();
    
    let menu_width = window_size.x*0.8;
    let menu_height = window_size.y*0.8;
    
    let background_colour = Vector4::new(0.2, 0.2, 0.3, 1.0);
    let button_colour = Vector4::new(0.8, 0.8, 0.8, 1.0);
    let checked_box = Vector4::new(0.2, 0.2, 0.2, 1.0);
    let font =  "Arial".to_string();
    let positions = OptionsUi::realign_widget_positions(window_size, Vector2::new(menu_width, menu_height));
    
    
    let background = Box::new(Image::new(positions[WidgetIndex::Background.n()], Vector2::new(menu_width, menu_height))
                                     .with_primary_colour(background_colour));
    
    let return_button_position = positions[WidgetIndex::Return.n()];
    let return_text_position = positions[WidgetIndex::ReturnText.n()];
    let return_text: Box<Widget> = Box::new(Text::new(return_text_position, 64.0, &font, &"Return".to_string()).center_text());
    let return_button = Box::new(Button::new(return_button_position, Vector2::new(BUTTON_WIDTH, BUTTON_HEIGHT))
                                     .with_primary_colour(button_colour));
    
    let save_button_position = positions[WidgetIndex::Save.n()];
    let save_text_position = positions[WidgetIndex::SaveText.n()];
    let save_text: Box<Widget> = Box::new(Text::new(save_text_position, 64.0, &font, &"Save".to_string()).center_text());
    let save_button = Box::new(Button::new(save_button_position, Vector2::new(BUTTON_WIDTH, BUTTON_HEIGHT))
                                     .with_primary_colour(button_colour));
    
    let vsync_checkbox_position = positions[WidgetIndex::Vsync.n()];
    let vsync_text_position = positions[WidgetIndex::VsyncText.n()];
    let mut vsync = Box::new(CheckBox::new(vsync_checkbox_position, Vector2::new(50.0, 50.0))
                                       .with_secondary_colour(button_colour)
                                       .with_primary_colour(checked_box));
    let vsync_text = Box::new(Text::new(vsync_text_position, 128.0, &font, &"Vsync".to_string()));
    
    let fullscreen_checkbox_position = positions[WidgetIndex::Fullscreen.n()];
    let fullscreen_text_position = positions[WidgetIndex::FullscreenText.n()];
    let mut fullscreen = Box::new(CheckBox::new(fullscreen_checkbox_position, Vector2::new(50.0, 50.0))
                                  .with_primary_colour(checked_box)
                                  .with_secondary_colour(button_colour));
    let fullscreen_text = Box::new(Text::new(fullscreen_text_position, 128.0, &font, &"Fullscreen".to_string()));
    
    let msaa_dropdown_position = positions[WidgetIndex::Msaa.n()];
    let msaa_text_position = positions[WidgetIndex::MsaaText.n()];
    
    let msaa_index = match msaa {
      2 => {
        2
      },
      4 => {
        3
      },
      8 => {
        4
      },
      16 => {
        5
      },
      _ => {
        1
      },
    };
    
    let mut msaa = Box::new(DropdownBox::new(msaa_dropdown_position, Vector2::new(50.0, 50.0), "Arial".to_string())
                                  /*.with_colour(Vector4::new(0.0, 0.4, 0.0, 1.0))*/
                                  .add_option("x1".to_string())
                                  .add_option("x2".to_string())
                                  .add_option("x4".to_string())
                                  .add_option("x8".to_string())
                                  .add_option("x16".to_string())
                                  .set_option(msaa_index)
                                  .with_primary_colour(button_colour));
    
    let msaa_text = Box::new(Text::new(msaa_text_position, 128.0, &font, &"Msaa".to_string()));
    
    let resolution_dropdown_position = positions[WidgetIndex::Resolution.n()];
    let resolution_position = positions[WidgetIndex::ResolutionText.n()];
    
    let resolution_text = Box::new(Text::new(resolution_position, 128.0, &font, &"Resolution".to_string()));
    let mut resolution = Box::new(DropdownBox::new(resolution_dropdown_position, Vector2::new(150.0, 50.0), "Arial".to_string())
                                  .add_option("800x600".to_string())
                                  .add_option("1024x1200".to_string())
                                  .add_option("1280x1080".to_string())
                                  .add_option("1920x1080".to_string())
                                  .add_option("2560x1080".to_string())
                                  .set_option(msaa_index)
                                  .with_primary_colour(button_colour));
    
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
                    .with_widget(msaa)
                    .with_widget(msaa_text)
                    .with_widget(resolution)
                    .with_widget(resolution_text)
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
    
    let msaa_text = background_position + Vector2::new(-menu_size.x*0.5, menu_size.y*0.5) + Vector2::new(50.0, -300.0);
    let msaa = msaa_text + Vector2::new(200.0, 15.0);
    
    let resolution_text = background_position + Vector2::new(-menu_size.x*0.25, menu_size.y*0.5) + Vector2::new(50.0, -100.0);
    let resolution = resolution_text + Vector2::new(275.0, 15.0);
    
    // Backgound pos, resume pos, resume text pos, options pos, options text pos, quit pos, quit text pos
    vec!(background_position, vsync, vsync_text, fullscreen, fullscreen_text, msaa, msaa_text, resolution, resolution_text, save_position, save_text, return_position, return_text)
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
  
  fn update_ui(&mut self, mouse_pos: Vector2<f32>, left_mouse: bool, escape_pressed: bool, window_size: Vector2<f32>, should_close: &mut bool, should_resize: &mut Option<Vector2<f32>>, _delta_time: f32) {
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
      let msaa = {
        match self.data().widgets[WidgetIndex::Msaa.n()].text().as_ref() {
          "x2" => {
            2
          },
          "x4" => {
            4
          },
          "x8" => {
            8
          },
          "x16" => {
            16
          },
          _ => {
            1
          },
        }
      };
      let resolution: Vector2<f32> = {
        match self.data().widgets[WidgetIndex::Resolution.n()].text().as_ref() {
          "800x600" => {
            Vector2::new(800.0, 600.0)
          },
          "1024x1200" => {
            Vector2::new(1024.0, 1200.0)
          },          
          "1920x1080" => {
            Vector2::new(1920.0, 1080.0)
          },
          "2560x1080" => {
            Vector2::new(2560.0, 1080.0)
          },
          "1280x1080" | _ => {
            Vector2::new(1280.0, 1080.0)
          },
        }
      };
      *should_resize = Some(resolution);
      self.settings.set_resolution(Vector2::new(resolution.x as i32, resolution.y as i32));
      self.settings.set_texture_msaa(msaa);
      self.settings.save();
      println!("settings svaed");
    }
  }
}


