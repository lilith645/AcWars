use maat_graphics::DrawCall;
use maat_graphics::Settings;

use crate::modules::ui::{Ui, UiData};
use maat_gui::widgets::{Widget, Image, Text, Button, CheckBox, DropdownBox};

use crate::cgmath::{Vector2, Vector4};

#[derive(Clone)]
pub struct OptionsUi {
  data: UiData,
  settings: Settings,
  resolution_indexs: Vec<usize>,
}

//const BACKGROUND: usize = 0;
const VSYNC: usize =  1;
//const VSYNC_TEXT: usize = 2;
const FULLSCREEN: usize = 3;
//const FULLSCREEN_TEXT: usize = 4;
const MSAA: usize =  5;
//const MSAA_TEXT: usize =  6;
const RESOLUTION: usize = 7;
//const RESOLUTION_TEXT: usize =  8;
const SAVE: usize =  9;
//const SAVE_TEXT: usize =  10;
const RETURN: usize =  11;
//const RETURN_TEXT: usize =  12;

impl OptionsUi {
  pub fn new(window_size: Vector2<f32>) -> OptionsUi {
    let _iwindow_size = Vector2::new(window_size.x as i32, window_size.y as i32);
    let settings = Settings::load();
    let vsync_setting = settings.vsync_enabled();
    let fullscreen_setting = settings.is_fullscreen();
    let msaa = settings.get_texture_msaa();
    
    let background_colour = Vector4::new(0.2, 0.2, 0.3, 1.0);
    let button_colour = Vector4::new(0.8, 0.8, 0.8, 1.0);
    let checked_box = Vector4::new(0.2, 0.2, 0.2, 1.0);
    let font =  "Arial".to_string();
    
    let background = OptionsUi::create_background(window_size, background_colour);
    let (return_button, return_text) = OptionsUi::create_return_button(window_size, &font, button_colour);
    let (save_button, save_text) = OptionsUi::create_save_button(window_size, &font, button_colour);
    let (mut vsync, vsync_text) = OptionsUi::create_vsync_button(window_size, &font, checked_box, button_colour);
    let (mut fullscreen, fullscreen_text) = OptionsUi::create_fullscreen_button(window_size, &font, checked_box, button_colour);
    let (msaa, msaa_text) = OptionsUi::create_msaa_dropdownbox(window_size, &font, button_colour, msaa);
    let (resolution, resolution_text, resolution_indexs) = OptionsUi::create_resolution_dropdownbox(window_size, &font, button_colour, &settings);
    
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
      resolution_indexs,
    }
  }
  
  fn create_background(window_size: Vector2<f32>, colour: Vector4<f32>) -> Box<Widget> {
    let pos = OptionsUi::background_position(window_size);
    let size = OptionsUi::background_size(window_size);
    
    let background = Box::new(Image::new(pos, size)
                                     .with_primary_colour(colour));
    
    background
  }
  
  fn create_return_button(window_size: Vector2<f32>, font: &String, colour: Vector4<f32>) -> (Box<Widget>, Box<Widget>) {
    let pos = OptionsUi::return_button_position(window_size);
    let text_pos = OptionsUi::return_text_position(window_size);
    
    let text = OptionsUi::small_text_size(window_size);
    
    let return_text: Box<Widget> = Box::new(Text::new(text_pos, text, font,
                                            &"Return".to_string()).center_text());
    let return_button = Box::new(Button::new(pos, OptionsUi::button_size(window_size))
                                         .with_primary_colour(colour));
    
    (return_button, return_text)
  }
  
  fn create_save_button(window_size: Vector2<f32>, font: &String, colour: Vector4<f32>) -> (Box<Widget>, Box<Widget>) {
    let pos = OptionsUi::save_button_position(window_size);
    let text_pos = OptionsUi::save_text_position(window_size);
    
    let button_size = OptionsUi::button_size(window_size);
    let text = OptionsUi::small_text_size(window_size);
    let save_text: Box<Widget> = Box::new(Text::new(text_pos, text, font, &"Save".to_string()).center_text());
    let save_button = Box::new(Button::new(pos, button_size)
                                     .with_primary_colour(colour));
    
    (save_button, save_text)
  }
  
  fn create_vsync_button(window_size: Vector2<f32>, font: &String, primary_colour: Vector4<f32>, secondary_colour: Vector4<f32>) -> (Box<Widget>, Box<Widget>) {
    let pos = OptionsUi::vsync_button_position(window_size);
    let text_pos = OptionsUi::vsync_text_position(window_size);
    
    let checkbox_size = OptionsUi::checkbox_size(window_size);
    let text = OptionsUi::large_text_size(window_size);
    let vsync = Box::new(CheckBox::new(pos, checkbox_size)
                                       .with_primary_colour(primary_colour)
                                       .with_secondary_colour(secondary_colour));
    let vsync_text = Box::new(Text::new(text_pos, text, &font, &"Vsync".to_string()));
    
    (vsync, vsync_text)
  }
  
  fn create_fullscreen_button(window_size: Vector2<f32>, font: &String, primary_colour: Vector4<f32>, secondary_colour: Vector4<f32>) -> (Box<Widget>, Box<Widget>) {
    let pos = OptionsUi::vsync_button_position(window_size);
    let text_pos = OptionsUi::vsync_text_position(window_size);
    
    let checkbox_size = OptionsUi::checkbox_size(window_size);
    let text = OptionsUi::large_text_size(window_size);
    let fullscreen = Box::new(CheckBox::new(pos, checkbox_size)
                                  .with_primary_colour(primary_colour)
                                  .with_secondary_colour(secondary_colour));
    let fullscreen_text = Box::new(Text::new(text_pos, text, &font, &"Fullscreen".to_string()));
    
    (fullscreen, fullscreen_text)
  }
  
  fn create_msaa_dropdownbox(window_size: Vector2<f32>, font: &String, primary_colour: Vector4<f32>, msaa: u32) -> (Box<Widget>, Box<Widget>) {
   
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
    
    let options = OptionsUi::msaa_options();
    let dropdownbox_size = OptionsUi::dropdownbox_size(window_size);
    
    let mut msaa = DropdownBox::new(OptionsUi::msaa_dropdown_position(window_size), dropdownbox_size, font.to_string());
    
    for option in options {
      msaa = msaa.add_option(option.to_string());
    }
    msaa = msaa.set_option(msaa_index).with_primary_colour(primary_colour);
    let text = OptionsUi::large_text_size(window_size);
    let text_pos = OptionsUi::msaa_dropdown_text_position(window_size);
    let msaa_text = Box::new(Text::new(text_pos, text, &font, &"Msaa".to_string()));
    
    (Box::new(msaa), msaa_text)
  }
  
  fn create_resolution_dropdownbox(window_size: Vector2<f32>, font: &String, primary_colour: Vector4<f32>, settings: &Settings) -> (Box<Widget>, Box<Widget>, Vec<usize>) {
    let pos = OptionsUi::resolution_dropdown_position(window_size);
    let text_pos = OptionsUi::resolution_dropdown_text_position(window_size);
    
    let size = OptionsUi::resolution_dropdownbox_size(window_size);
    let text = OptionsUi::large_text_size(window_size);
    let resolution_text = Box::new(Text::new(pos, text, &font, &"Resolution".to_string()));
    let mut resolution = DropdownBox::new(text_pos, size, "Arial".to_string());
    
    let (resolution_options, resolution_indexs, selected_index) = OptionsUi::resolution_options(settings, window_size);
    
    for option in resolution_options {
      resolution = resolution.add_option(option.to_string());
      
    }
    
    resolution = resolution.set_option(selected_index+1).with_primary_colour(primary_colour);
    
    (Box::new(resolution), resolution_text, resolution_indexs)
  }
  
  fn background_position(window_size: Vector2<f32>) -> Vector2<f32> {
    window_size*0.5
  }
  
  fn background_size(window_size: Vector2<f32>) -> Vector2<f32> {
    window_size*0.8
  }
  
  fn resolution_dropdownbox_size(window_size: Vector2<f32>) -> Vector2<f32> {
    Vector2::new(window_size.y/3.0, window_size.y/21.6)
  }
  
  fn dropdownbox_size(window_size: Vector2<f32>) -> Vector2<f32> {
    Vector2::new(window_size.y/21.6, window_size.y/21.6)
  }
  
  fn checkbox_size(window_size: Vector2<f32>) -> Vector2<f32> {
    Vector2::new(window_size.y/21.6, window_size.y/21.6)
  }
  
  fn button_size(window_size: Vector2<f32>) -> Vector2<f32> {
    Vector2::new(window_size.x/16.0, window_size.y/16.0)
  }
  
  fn small_text_size(window_size: Vector2<f32>) -> f32 {
    window_size.y/16.875
  }
  
  fn large_text_size(window_size: Vector2<f32>) -> f32 {
    window_size.y/8.4375
  }
  
  fn return_button_position(window_size: Vector2<f32>) -> Vector2<f32> {
    let background_position = OptionsUi::background_position(window_size);
    let background_size = OptionsUi::background_size(window_size);
    
    let return_position = background_position-background_size*0.5+OptionsUi::return_offset(window_size);
    return_position
  }
  
  fn return_text_position(window_size: Vector2<f32>) -> Vector2<f32> {
    OptionsUi::return_button_position(window_size)
  }
  
  fn return_offset(window_size: Vector2<f32>) -> Vector2<f32> {
    let button_size = OptionsUi::button_size(window_size);
    Vector2::new(button_size.x*0.5, button_size.y*0.5)+Vector2::new(25.0, 25.0)
  }
  
  fn save_button_position(window_size: Vector2<f32>) -> Vector2<f32> {
    let background_position = OptionsUi::background_position(window_size);
    let background_size = OptionsUi::background_size(window_size);
    let save_position = background_position+Vector2::new(background_size.x*0.5, -background_size.y*0.5) + OptionsUi::save_offset(window_size);
    
    save_position
  }
  
  fn save_text_position(window_size: Vector2<f32>) -> Vector2<f32> {
    OptionsUi::save_button_position(window_size)
  }
  
  fn save_offset(window_size: Vector2<f32>) -> Vector2<f32> {
    let button_size = OptionsUi::button_size(window_size);
    Vector2::new(-button_size.x*0.5, button_size.y*0.5) + Vector2::new(-25.0, 25.0)
  }
  
  fn vsync_text_position(window_size: Vector2<f32>) -> Vector2<f32> {
    let background_position = OptionsUi::background_position(window_size);
    let background_size = OptionsUi::background_size(window_size);
    let vsync_text = background_position + Vector2::new(-background_size.x*0.5, background_size.y*0.5) + Vector2::new(50.0, -100.0);
    
    vsync_text
  }
  
  fn vsync_button_position(window_size: Vector2<f32>) -> Vector2<f32> {
     OptionsUi::vsync_text_position(window_size) + Vector2::new(200.0, 15.0)
  }
  
  fn fullscreen_text_position(window_size: Vector2<f32>) -> Vector2<f32> {
    let background_position = OptionsUi::background_position(window_size);
    let background_size = OptionsUi::background_size(window_size);
    let fullscreen_text = background_position + Vector2::new(-background_size.x*0.5, background_size.y*0.5) + Vector2::new(50.0, -200.0);
    
    fullscreen_text
  }
  
  fn fullscreen_button_position(window_size: Vector2<f32>) -> Vector2<f32> {
    OptionsUi::fullscreen_text_position(window_size) + Vector2::new(200.0, 15.0)
  }
  
  fn msaa_dropdown_position(window_size: Vector2<f32>) -> Vector2<f32> {
    OptionsUi::msaa_dropdown_text_position(window_size) + Vector2::new(200.0, 15.0)
  }
  
  fn msaa_dropdown_text_position(window_size: Vector2<f32>) -> Vector2<f32> {
    let background_position = OptionsUi::background_position(window_size);
    let background_size = OptionsUi::background_size(window_size);
    let msaa_text = background_position + Vector2::new(-background_size.x*0.5, background_size.y*0.5) + Vector2::new(50.0, -300.0);
    
    msaa_text
  }
  
  fn msaa_options() -> Vec<String> {
    vec!("x1".to_string(), "x2".to_string(), "x4".to_string(), "x8".to_string(), "x16".to_string(),)
  }
  
  fn resolution_dropdown_position(window_size: Vector2<f32>) -> Vector2<f32> {
    OptionsUi::resolution_dropdown_text_position(window_size) + Vector2::new(275.0, 15.0)
  }
  
  fn resolution_dropdown_text_position(window_size: Vector2<f32>) -> Vector2<f32> {
    let background_position = OptionsUi::background_position(window_size);
    let background_size = OptionsUi::background_size(window_size);
    let resolution_text = background_position + Vector2::new(-background_size.x*0.25, background_size.y*0.5) + Vector2::new(50.0, -100.0);
    resolution_text
  }
  
  fn resolution_options(settings: &Settings, window_size: Vector2<f32>) -> (Vec<String>, Vec<usize>, u32) {
    let max_resolution = settings.max_monitor_resolution();
    
    let mut resolutions = Vec::new();
    
    let mut index_references = Vec::new();
    let mut selected_index = 0;
    
    let mut i = 0;
    for (width, height, ratio) in Settings::resolutions().iter() {
      if *width <= max_resolution[0] && *height <= max_resolution[1] {
        resolutions.push(width.to_string().to_owned() + &"x" + &height.to_string() + " " + ratio);
        index_references.push(i);
        if *width == window_size.x as u32 && *height == window_size.y as u32 {
          selected_index = index_references.len() as u32-1;
        }
      }
      i+=1;
    }
    
    (resolutions, index_references, selected_index)
  }
  
  fn realign_widget_positions(window_size: Vector2<f32>) -> Vec<Vector2<f32>> {
    let background_position = OptionsUi::background_position(window_size);
    let return_position = OptionsUi::return_button_position(window_size);
    let return_text_position = OptionsUi::return_text_position(window_size);
    let save_position = OptionsUi::save_button_position(window_size);
    let save_text_position = OptionsUi::save_text_position(window_size);
    let vsync_position = OptionsUi::vsync_button_position(window_size);
    let vsync_text_position = OptionsUi::vsync_text_position(window_size);
    let msaa_position = OptionsUi::msaa_dropdown_position(window_size);
    let msaa_text_position = OptionsUi::msaa_dropdown_text_position(window_size);
    let fullscreen_position = OptionsUi::fullscreen_button_position(window_size);
    let fullscreen_text_position = OptionsUi::fullscreen_text_position(window_size);
    let resolution_position = OptionsUi::resolution_dropdown_position(window_size);
    let resolution_text_position = OptionsUi::resolution_dropdown_text_position(window_size);
    
    // Backgound pos, resume pos, resume text pos, options pos, options text pos, quit pos, quit text pos
    vec!(background_position, 
         vsync_position, vsync_text_position, 
         fullscreen_position, fullscreen_text_position, 
         msaa_position, msaa_text_position, 
         resolution_position, resolution_text_position, 
         save_position, save_text_position, 
         return_position, return_text_position)
  }
  
  fn realign_widget_sizes(window_size: Vector2<f32>) -> Vec<Vector2<f32>> {
    let large_text_size = OptionsUi::large_text_size(window_size);
    let small_text_size = OptionsUi::small_text_size(window_size);
    
    let background_size = OptionsUi::background_size(window_size);
    let return_size = OptionsUi::button_size(window_size);
    let return_text_size = Vector2::new(small_text_size, small_text_size);
    let save_size = OptionsUi::button_size(window_size);
    let save_text_size = Vector2::new(small_text_size, small_text_size);
    let vsync_size = OptionsUi::checkbox_size(window_size);
    let vsync_text_size = Vector2::new(large_text_size, large_text_size);
    let msaa_size = OptionsUi::dropdownbox_size(window_size);
    let msaa_text_size = Vector2::new(large_text_size, large_text_size);
    let fullscreen_size = OptionsUi::checkbox_size(window_size);
    let fullscreen_text_size = Vector2::new(large_text_size, large_text_size);
    let resolution_size = OptionsUi::resolution_dropdownbox_size(window_size);
    let resolution_text_size = Vector2::new(large_text_size, large_text_size);
    
    // Backgound pos, resume pos, resume text pos, options pos, options text pos, quit pos, quit text pos
    vec!(background_size, 
         vsync_size, vsync_text_size, 
         fullscreen_size, fullscreen_text_size, 
         msaa_size, msaa_text_size, 
         resolution_size, resolution_text_size, 
         save_size, save_text_size, 
         return_size, return_text_size)
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
  
  fn update_ui(&mut self, _mouse_pos: Vector2<f32>, _left_mouse: bool, escape_pressed: bool, window_size: Vector2<f32>, _should_close: &mut bool, should_resize: &mut Option<(Vector2<f32>, bool)>, _should_next_scene: &mut bool, _delta_time: f32) {
    let new_positions = OptionsUi::realign_widget_positions(window_size);
    let new_sizes = OptionsUi::realign_widget_sizes(window_size);
    for i in 0..new_positions.len() {
      self.mut_data().widgets[i].set_position(new_positions[i]);
      self.mut_data().widgets[i].set_size(new_sizes[i]);
    }
    
    if escape_pressed || self.data().widgets[RETURN].pressed() {
      self.disable();
    }
    
    if self.data().widgets[SAVE].pressed() {
      self.settings.set_vsync(self.data().widgets[VSYNC].activated());
      self.settings.enable_fullscreen(self.data().widgets[FULLSCREEN].activated());
      let msaa = {
        match self.data().widgets[MSAA].text().as_ref() {
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
        let index = self.data().widgets[RESOLUTION].external_option_value();
        
        let i = self.resolution_indexs[index as usize];
        let resolutions = Settings::resolutions();
        
        Vector2::new(resolutions[i].0 as f32, resolutions[i].1 as f32)
      };
      
      let is_fullscreen = self.data().widgets[FULLSCREEN].activated();
      *should_resize = Some((resolution, is_fullscreen));
      
      self.settings.set_resolution(Vector2::new(resolution.x as i32, resolution.y as i32));
      self.settings.set_texture_msaa(msaa);
      self.settings.save();
      println!("settings svaed");
    }
  }
  
  fn custom_draw(&self, _draw_calls: &mut Vec<DrawCall>) {
    
  }
}


