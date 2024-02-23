use std::{
    sync::{mpsc::channel, Arc, Mutex},
    thread,
};

use fltk::{
    app,
    frame::Frame,
    image::JpegImage,
    prelude::{GroupExt, ImageExt, WidgetBase, WidgetExt},
    window::Window,
};

//use super::control::{self, lists};
use crate::ui::control;
use crate::{get_file::FileInfo, ui::control::lists};

use super::execute;
pub fn body(all: (Vec<FileInfo>, Vec<String>)) {
    let app = app::App::default();
    let mut wind = Window::new(500, 500, 500, 500, None);
    //let mut image_fr = Frame::default();
    let image_fr = Arc::new(Mutex::new(Frame::default()));
    let image_bo = Arc::new(Mutex::new(JpegImage::load(all.1[0].clone()).unwrap()));

    let (sen_move, rec_move) = channel::<(i32, i32, i32)>();
    //(x_mov,y_mov,size)
    let (sen_list,rec_list) = channel::<String>();

    let image_frc = image_fr.clone();
    let image_boc = image_bo.clone();

    let mut locked_image_fr = image_frc.lock().unwrap();
    let mut locked_image_bo = image_boc.lock().unwrap();

    locked_image_fr.clone().center_of(&wind);
    locked_image_bo.scale(
        all.0[1].exif.exif_image_width.unwrap() as i32,
        all.0[1].exif.exif_image_height.unwrap() as i32,
        true,
        true,
    );
    locked_image_fr.set_image(Some(locked_image_bo.clone()));
    drop(locked_image_bo);
    drop(locked_image_fr);

    control::direction(sen_move);
    execute::image_loc(rec_move,image_fr,image_bo);
    wind.make_resizable(true);
    wind.show();
    wind.end();
    lists(wind,all.1,sen_list);
    app.run().unwrap();
}
