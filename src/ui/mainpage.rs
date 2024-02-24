use std::sync::{mpsc::channel, Arc, Mutex};

use fltk::{
    app, enums::Color, frame::Frame, image::JpegImage, prelude::{GroupExt, ImageExt, WidgetBase, WidgetExt}, window::Window
};

//use super::control::{self, lists};
use crate::ui::control;
use crate::{get_file::FileInfo, ui::control::lists};

use super::{execute, menu::menu_body};
pub fn body(all: (Vec<FileInfo>, Vec<String>)) {
    let app = app::App::default();
    
    let mut wind = Window::new(500, 500, 500, 500, None);
    wind.set_color(Color::from_hex(0x000000));
    //let mut image_fr = Frame::default();
    let image_fr = Arc::new(Mutex::new(Frame::default()));
    let image_bo = Arc::new(Mutex::new(JpegImage::load(all.1[0].clone()).unwrap()));

    let (sen_move, rec_move) = channel::<(i32, i32, i32)>();
    //(x_mov,y_mov,size)
    let (sen_list,rec_list) = channel::<(String,i32,i32)>();
    //(path,x,y)
    let (sen_menu,rec_menu) = channel::<(bool,FileInfo)>();
    
    let image_frc = image_fr.clone();
    let image_boc = image_bo.clone();

    let mut locked_image_fr = image_frc.lock().unwrap();
    let mut locked_image_bo = image_boc.lock().unwrap();

    locked_image_fr.clone().center_of(&wind);
    locked_image_bo.scale(
        800,
        800,
        true,
        true,
    );
    
    locked_image_fr.set_image(Some(locked_image_bo.clone()));
    locked_image_fr.set_pos(wind.w()/2, wind.w()/2);
    {
        sen_menu.send((false,all.0.get(0).unwrap().clone())).unwrap();
    }
    drop(locked_image_bo);
    drop(locked_image_fr);

    
    
    wind.make_resizable(true);
    menu_body(rec_menu);
    wind.show();
    wind.end();
    
    control::direction(sen_move.clone());
    execute::image_loc(rec_move,image_fr.clone(),image_bo.clone());
    execute::image_list(rec_list, sen_move, image_bo);
    lists(wind,all.1,sen_list,all.0,sen_menu);
    app.run().unwrap();
}
