use std::{
    cmp,
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

use fltk::{
    app::{awake, redraw},
    frame::Frame,
    image::JpegImage,
    prelude::{ImageExt, WidgetExt},
    text,
};

use crate::get_file::FileInfo;

pub fn image_loc(
    rec_move: Receiver<(i32, i32, i32)>,
    image_fr: Arc<Mutex<Frame>>,
    image_bo: Arc<Mutex<JpegImage>>,
) {
    thread::spawn(move || {
        let image_frc = image_fr.clone();
        let image_boc = image_bo.clone();

        let locked_image_fr = image_frc.lock().unwrap();
        let mut x = locked_image_fr.x();
        let mut y = locked_image_fr.y();
        drop(locked_image_fr);
        let mut size = 500;
        for i in rec_move {
            let mut locked_image_fr = image_frc.lock().unwrap();
            if i.2 != 0 {
                let mut locked_image_bo = image_boc.lock().unwrap();
                if i.2 == 1 {
                    if size <= 6000 {
                        size = cmp::max((size as f64 * 1.25) as i32, size + 500);
                    }
                } else if i.2 == -1 {
                    if size >= 50 {
                        size = (size as f64 * 0.8) as i32;
                    }
                } else if i.2 == 2 {
                    size = 800;
                    x = -i.0 / 2;
                    y = -i.1 / 2;
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
    rec_list: Receiver<(String, i32, i32)>,
    sen_move: Sender<(i32, i32, i32)>,
    image_bo: Arc<Mutex<JpegImage>>,
) {
    thread::spawn(move || {
        for i in rec_list {
            let image_boc = image_bo.clone();
            let mut locked_image_bo = image_boc.lock().unwrap();
            locked_image_bo.clone_from(
                &JpegImage::load(i.0).unwrap()
            );
            sen_move.send((i.1, i.2, 2)).expect("sen_move err");
        }
    });
}
pub fn image_menu(
    rec_menu: Receiver<(bool, FileInfo)>,
    mut buf: text::TextBuffer,
    mut tex: text::TextDisplay,
) {
    thread::spawn(move || {
        let mut cnt = 0;
        for i in rec_menu {
            if i.0 {
                cnt += 1;
                if cnt == 3 {
                    tex.show();
                    cnt = 0;
                }
            }
            if cnt == 0 {
                let te = format!(
                    "日期：{} ({})\n焦距：{} mm\n光圈：F {}\n快门：{} s\nISO：{}",
                    i.1.exif.date_time.unwrap(),
                    i.1.exif.offset_time.unwrap(),
                    i.1.exif.focal_length.unwrap().to_float(),
                    i.1.exif.f_number.unwrap().to_float(),
                    i.1.exif.exposure_time.unwrap().to_string(),
                    i.1.exif.iso.unwrap()
                );
                buf.set_text(&te);
            }
            if cnt == 1 {
                let te = format!(
                    "相机型号：{}\n镜头型号：{}\n文件大小：{:.3} MB\n文件路径：{}",
                    i.1.exif.make.unwrap(),
                    i.1.exif.lens_model.unwrap(),
                    (i.1.len as f64 )/ 1024.0/1024.0,
                    i.1.path
                );
                buf.set_text(&te);
            }
            if cnt == 2 {
                tex.hide();
            }
        }
    });
}
