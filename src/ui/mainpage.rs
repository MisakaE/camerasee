use std::{sync::{mpsc::channel, Arc, Mutex}, thread};

use fltk::{app, frame::Frame, image::JpegImage, prelude::{GroupExt, ImageExt, WidgetBase, WidgetExt}, window::Window};

//use super::control::{self, lists};
use crate::ui::control::lists;
use crate::ui::control;
pub fn body(){
    let app = app::App::default();
    let mut wind = Window::new(500, 500, 500, 500, None);
    //let mut image_fr = Frame::default();
    let image_fr = Arc::new(Mutex::new(Frame::default()));
    let image_bo = Arc::new(Mutex::new(JpegImage::load("1.JPG").unwrap()));

    
    let (sen_move,rec_move) = channel::<(i32,i32,i32)>();
    //(x_mov,y_mov,size)

    let image_frc = image_fr.clone();
    let image_boc = image_bo.clone();

    let mut locked_image_fr = image_frc.lock().unwrap();
    let mut locked_image_bo = image_boc.lock().unwrap();

    locked_image_fr.clone().center_of(&wind);
    locked_image_bo.scale(6000, 6000, true, true);
    locked_image_fr.set_image(Some(locked_image_bo.clone()));
    
    control::direction(sen_move);
    
    wind.make_resizable(true);
    wind.show();
    wind.end();
    lists(wind);
    app.run().unwrap();
}