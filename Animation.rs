#![no_main]

use std::hash::Hash;
use std::{thread, time};
use std::collections::HashMap;
use sfml::{
    graphics::{
        Rect, RenderTarget, RenderWindow,Texture,
        Sprite, Transformable,
    },
    system::{Vector2f},
};

pub struct Animation<'a>
{
    current_frame: i32,
    texture: &'a Texture,
    sprite : Sprite<'a>,
    count_frames: i32,
    delay_between_frames_milli_sec: i32,
    width: i32,
    height: i32,
}

impl<'a> Animation<'a>
{
    pub fn new(texture: &'a Texture,mut count_frames: i32,mut delay_milli_sec: i32,mut width:i32,mut height:i32) -> Self
    {
        if delay_milli_sec < 0
        {
            delay_milli_sec = -delay_milli_sec;
        }

        if width < 0
        {
            width = -width;
        }

        if height < 0 
        {
            height = -height;
        }

        if count_frames < 0
        {
            count_frames = -count_frames;
        }

        let mut temp_sprite = Sprite::new();
        temp_sprite.set_texture(texture, true);

        Self
        {
            current_frame: 0,
            texture: texture,
            sprite: temp_sprite,
            count_frames: count_frames,
            delay_between_frames_milli_sec: delay_milli_sec,
            width: width,
            height: height,
        }
    }

    pub fn draw(&mut self,window: &mut RenderWindow, x: i32, y: i32)
    {
        let ten_millis = time::Duration::from_millis(self.delay_between_frames_milli_sec.try_into().unwrap());
        if self.current_frame == self.count_frames
        {
            self.current_frame = 0;
        }

        self.sprite.set_position(Vector2f::new(x as f32,y as f32));
        self.sprite.set_texture_rect(&Rect::new(self.width * self.current_frame, 0, self.width, self.height));
        self.current_frame += 1;
        window.draw(&self.sprite);
        thread::sleep(ten_millis);
    }
}

// pub struct AnimationManager<'a>
// {
//     names: Vec<String>,
//     animations: Vec<Animation<'a>>,
//     current_animation: String,
// }

// impl<'a> AnimationManager<'a>
// {
//     pub fn new() -> Self
//     {
//         Self
//         {
//             names: Vec::new(),
//             animations: Vec::new(),
//             current_animation:String::new(),
//         }
//     }

//     pub fn create(&mut self, name: &str, texture: &'a Texture, mut count_frames: i32, mut delay_milli_sec: i32, mut width:i32, mut height:i32)
//     {
//         let mut tmp = Animation::new(texture, count_frames, delay_milli_sec, width, height);
//         self.names.push(name.to_string());
//         self.animations.push(tmp);
//         self.current_animation = name.to_string();
//     }

//     pub fn set_animation(&mut self, name:&str)
//     {
//         self.current_animation = name.to_string();
//     }

//     pub fn draw(&mut self,mut window: &mut RenderWindow, x: i32, y: i32)
//     {
//         let mut index = 0;
//         for i in &self.names
//         {
//             if *i != self.current_animation
//             {
//                 index += 1;
//             }
//         }
//     }
// }