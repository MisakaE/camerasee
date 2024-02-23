use cae::{exif::{parse, ExifInfo}, get_file::{get_all, parameters}, to_fracs_ii, to_fracu_ii, to_u32_ii, ui};
use fltk::{
    app,
    frame::Frame,
    image::JpegImage,
    prelude::{GroupExt, ImageExt, WidgetBase, WidgetExt},
    window::Window,
};
use std::{collections::HashMap, fs::{self, File}, io::Read};
fn main() {
    let para = parameters().unwrap();
    if !para.0{

    } else {
        let all = get_all(para.1);
        ui::mainpage::body(all);
    }
}