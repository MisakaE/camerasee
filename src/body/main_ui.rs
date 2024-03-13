use std::sync::mpsc;

use fltk::app;
use fltk::enums::Color;
use fltk::prelude::GroupExt;
use fltk::window::Window;
use fltk::prelude::WidgetBase;
use fltk::prelude::WidgetExt;

use crate::get_file::get_all;

use super::control;
use super::image_printer;

pub fn ui(){
    let app = app::App::default();
    let mut wind = Window::new(500, 500, 500, 500, None);
    wind.set_color(Color::from_hex(0x000000));
    
    wind.make_resizable(true);
    wind.end();
    wind.show();
    let (sen,rec) = mpsc::channel();
    let file_list = get_all("./".to_string()).1;
    control::move_lis(sen.clone());
    control::resize_lis(sen);
    image_printer::printer(rec, file_list,wind).unwrap();
    

    app.run().unwrap();
}