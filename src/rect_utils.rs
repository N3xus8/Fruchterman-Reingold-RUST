use raylib::prelude::*;
use std::fmt::Error;



pub fn center_rect(rect: Rectangle, relative_width: f32, relative_height: f32) -> Result<Rectangle, Error> {
    return Ok(Rectangle::new(rect.x + rect.width*(1.0-relative_width)/2.0, 
                             rect.y  + rect.height*(1.0-relative_height)/2.0, 
                             rect.width * relative_width, 
                             rect.height * relative_height)
            );
}