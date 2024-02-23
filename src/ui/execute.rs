use std::{
    sync::{mpsc::Receiver, Arc, Mutex},
    thread,
};

use fltk::{
    app::{awake, redraw},
    frame::Frame,
    image::JpegImage,
    prelude::{ImageExt, WidgetExt},
};

pub fn image_loc(
    rec_move: Receiver<(i32, i32, i32)>,
    image_fr: Arc<Mutex<Frame>>,
    image_bo: Arc<Mutex<JpegImage>>,
) {
    thread::spawn(move || {
        let image_frc = image_fr.clone();
        let image_boc = image_bo.clone();
        
        let mut x = 0;
        let mut y = 0;
        let mut size = 500;
        for i in rec_move {
            let mut locked_image_fr = image_frc.lock().unwrap();
            if i.2 != 0 {
                let mut locked_image_bo = image_boc.lock().unwrap();
                if i.2 == 1 {
                    size += 50;
                } else if i.2 == -1 {
                    size -= 50;
                } else if i.2 == 2 {
                    size = 500;
                    x = 0;
                    y = 0;
                }
                locked_image_bo.scale(size, size, true, true);
                locked_image_fr.set_image(Some(locked_image_bo.clone()));
                drop(locked_image_bo);
                
                
            }
            x += i.0;
            y += i.1;
            locked_image_fr.set_pos(x, y);
            drop(locked_image_fr);
            redraw();
            awake();
        }
    });
}
pub fn image_list(
    rec_list: Receiver<String>,
    image_fr: Arc<Mutex<Frame>>,
    image_bo: Arc<Mutex<JpegImage>>,
) {
    thread::spawn(move || for i in rec_list {});
}
