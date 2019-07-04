use maat_graphics::DrawCall;

use std::cmp;

use crate::cgmath::{Vector2, Vector4};

#[derive(Debug, Clone)]
pub struct Node {
  location: Vector2<f32>,
  l_child: Option<Box<Node>>,
  r_child: Option<Box<Node>>,
  min_width: Option<f32>,
  max_width: Option<f32>,
  min_height: Option<f32>,
  max_height: Option<f32>,
}

impl Node {
  pub fn get_depth_index(&self, position: Vector2<f32>, depth: u32, mut index: (i32, i32)) -> (i32, i32) {
    let k = 2;
    let axis = depth%k;
    if axis == 0 {
      if position.x < self.location.x {
        if let Some(child) = &self.l_child {
          index.0 += depth as i32+1;
          child.get_depth_index(position, depth+1, index)
        } else {
          index
        }
      } else {
        if let Some(child) = &self.r_child {
          index.1 += depth as i32+1;
          child.get_depth_index(position, depth+1, index)
        } else {
          index
        }
      }
    } else {
      if position.y < self.location.y {
        if let Some(child) = &self.l_child {
          index.0 += depth as i32+1;
          child.get_depth_index(position, depth+1, index)
        } else {
          index
        }
      } else {
        if let Some(child) = &self.r_child {
          index.1 += depth as i32+1;
          child.get_depth_index(position, depth+1, index)
        } else {
          index
        }
      }
    }
  }
  
  pub fn get_boundries(&self) -> Vector4<f32> {
    let mut boundry = Vector4::new(0.0, 0.0, 1.0, 1.0);
    
    if let Some(min_width) = &self.min_width {
      boundry.x = *min_width;
    }
    if let Some(max_width) = &self.max_width {
      boundry.z = *max_width;
    }
    
    if let Some(child) = &self.l_child {
      if let Some(min_height) = child.min_height {
        boundry.y = min_height;
      }
    } else {
      boundry.y = boundry.x;
    }
    
    if let Some(child) = &self.r_child {
      if let Some(max_height) = child.max_height {
        boundry.w = max_height;
      }
    } else {
      boundry.w = boundry.z;
    }
    
    boundry
  }
  
  pub fn create_kdtree(mut positions: Vec<Vector2<f32>>, goal_number: usize, depth: u32, max_depth: u32) -> Option<Box<Node>> {
    if positions.len() < goal_number || depth > max_depth {
      return None;
    }
    
    let k = 2;
    let axis = depth%k;
    
    let mut min_width = None;
    let mut max_width = None;
    let mut min_height = None;
    let mut max_height = None;
    if axis == 0 {
      positions.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(cmp::Ordering::Equal));
      min_width = Some(positions[0].x);
      max_width = Some(positions[positions.len()-1].x);
    } else { 
      positions.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap_or(cmp::Ordering::Equal));
      min_height = Some(positions[0].y);
      max_height = Some(positions[positions.len()-1].y);
    }
    
    let median = (positions.len() as f32 * 0.5).floor();
    
    let (first_half, second_half) = positions.split_at(median as usize);
    
    Some(Box::new(Node {
      location: second_half[0].clone(),
      l_child: Node::create_kdtree(first_half.to_vec(), goal_number, depth+1, max_depth),
      r_child: Node::create_kdtree(second_half.to_vec(), goal_number, depth+1, max_depth),
      min_width,
      max_width,
      min_height,
      max_height,
    }))
  }
  
  pub fn draw_kdtree(mut nodes: Option<Box<Node>>, depth: i32, draw_calls: &mut Vec<DrawCall>, parent_bound: f32, width: f32, height: f32) {
    if nodes.is_none() || depth == 3 {
      return;
    }
    
    let temp_node = nodes.unwrap();
    
    let k = 2;
    let axis = depth % k;
    
    let mut new_parent_bound = 0.0;

    match axis {
      0 => {
        let mut upper_height_constraint = height;
        let mut lower_height_constraint = 0.0;
        
        if parent_bound > temp_node.location.y {
          upper_height_constraint = parent_bound; 
        } else {
          lower_height_constraint = parent_bound;
        }
        
        let length = upper_height_constraint - lower_height_constraint;
        
        let x = temp_node.location.x;
        let y = upper_height_constraint - length*0.5;
        
        draw_calls.push(DrawCall::draw_coloured(Vector2::new(x, y),
                                                Vector2::new(10.0, length),
                                                Vector4::new(1.0, 0.0, 0.0, 1.0),
                                                0.0));
        
        new_parent_bound = temp_node.location.x;
      },
      1 => {
        let mut upper_width_constraint = width;
        let mut lower_width_constraint = 0.0;
        
        if parent_bound > temp_node.location.x {
          upper_width_constraint = parent_bound; 
        } else {
          lower_width_constraint = parent_bound;
        }
        
        
        let length = upper_width_constraint - lower_width_constraint;
        
        let x = upper_width_constraint - length*0.5;
        let y = temp_node.location.y;
        
        draw_calls.push(DrawCall::draw_coloured(Vector2::new(x, y),
                                                Vector2::new(length, 10.0),
                                                Vector4::new(0.0, 0.0, 1.0, 1.0),
                                                0.0));
        
        new_parent_bound = temp_node.location.y;
      },
      _ => {
        ()
      },
    }
    
    
    Node::draw_kdtree(temp_node.l_child.clone(), depth+1, draw_calls, new_parent_bound, width, height);
    Node::draw_kdtree(temp_node.r_child, depth+1, draw_calls, new_parent_bound, width, height);
  }
}
