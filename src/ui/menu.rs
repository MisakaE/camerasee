use std::sync::mpsc::Receiver;

use fltk::{enums::Color,group::Flex, prelude::{DisplayExt, WidgetBase, WidgetExt}, text};

use crate::get_file::FileInfo;

use super::execute;

pub fn menu_body(
    rec_menu:Receiver<(bool,FileInfo)>
){
    let mut menu_list_y = Flex::new(0, 0, 200, 100, None).row();
    let mut menu_list = Flex::new(0, 0, 100, 200, None).column();
    menu_list_y.fixed(&menu_list,400);

    let mut tex = text::TextDisplay::default();
    menu_list.fixed(&tex, 180);

    let buf = text::TextBuffer::default();
    tex.set_buffer(buf.clone());
    tex.set_color(Color::from_hex(0x000000));
    tex.set_text_color(Color::from_hex(0xffffff));
    //thread::spawn(move||for);
    execute::image_menu(rec_menu, buf,tex);
}