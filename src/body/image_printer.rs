use std::{error::Error, sync::mpsc::Receiver, thread};

use fltk::{app::{awake, redraw}, image::JpegImage, prelude::{ImageExt, WidgetBase, WidgetExt}, window::DoubleWindow};

use super::Command;

pub fn printer(
    rec: Receiver<Command>,
    file_list: Vec<String>,
    mut wind:DoubleWindow,
) -> Result<(), Box<dyn Error>> {
    

    let mut size = wind.h();
    
    let mut file_id: usize = 0;
    let mx_num = file_list.len();

    let mut picture = JpegImage::load(file_list[0].clone())?;
    
    picture.scale(size, size, true,true);
    
    thread::spawn(move|| {
        let mut pic_x: i32 = 0;
        let mut pic_y: i32 = 0;
        for command in rec {
            match command {
                Command::Move((x, y)) => {
                    to_move(x, y, &mut pic_x, &mut pic_y);
                }
                Command::Resize(resize) => {
                    to_resize(resize, &mut size);
                    picture.scale(size, size, true,true);
                }
                Command::Switch(list) => {
                    to_switch(list, &mut file_id, mx_num);
                },
            }
            let mut picture = picture.copy();
            wind.draw(move|f|{
                picture.draw_ext(0, 0, f.w(), f.h(), pic_x, pic_y);
            });
            awake();
            redraw();
            
        }
        
        
    });
    Ok(())
}
fn to_move(x: i32, y: i32, pic_x: &mut i32, pic_y: &mut i32) {
    *pic_x += x;
    *pic_y += y;
}
fn to_resize(resize: i32, size: &mut i32) {
    *size += resize;
}
fn to_switch(list:bool,file_id:&mut usize,mx_num:usize){
    if list{
        *file_id+=1;
        if *file_id==mx_num{
            *file_id=0;
        }
    } else {
        if *file_id==0{
            *file_id = mx_num;
        }
        *file_id-=1;
    }
}