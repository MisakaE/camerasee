use std::{sync::mpsc::Sender, thread};

use fltk::{
    app::{event_key_down, sleep},
    enums::{Event, Key},
    prelude::{WidgetBase, WidgetExt},
    window::DoubleWindow,
};

use crate::get_file::FileInfo;
pub fn direction(sen_move: Sender<(i32, i32, i32)>) {
    let speed = 20;
    thread::spawn(move || loop {
        //if event() !=  Event::KeyDown{
        let mut x_mov = 0;
        let mut y_mov = 0;
        let mut size = 0;

        if event_key_down(Key::from_char('a')) {
            x_mov -= speed;
        }
        if event_key_down(Key::from_char('d')) {
            x_mov += speed;
        }
        if event_key_down(Key::from_char('w')) {
            y_mov -= speed;
        }
        if event_key_down(Key::from_char('s')) {
            y_mov += speed;
        }
        if event_key_down(Key::from_char('q')) {
            size -= 1;
        }
        if event_key_down(Key::from_char('e')) {
            size += 1;
        }
        if x_mov != 0 || y_mov != 0 || size != 0 {
            sen_move
                .send((x_mov, y_mov, size))
                .expect("sen_move send error!");
        }
        //}
        sleep(0.05);
    });
}
pub fn lists(
    mut wind: DoubleWindow,
    list: Vec<String>,
    sen_list: Sender<(String, i32, i32)>,
    file_info_list: Vec<FileInfo>,
    sen_menu: Sender<(bool, FileInfo)>,
) {
    let mut cnt = 1;
    let wins = wind.clone();
    wind.handle(move |_, event| match event {
        Event::KeyDown => {
            if event_key_down(Key::from_char('c')) {
                cnt += 1;
                if cnt == list.len() {
                    cnt = 0;
                }
                sen_list
                    .send((list[cnt].clone(), wins.w(), wins.h()))
                    .expect("sen_list err");
                sen_menu
                    .send((false, file_info_list.get(cnt).unwrap().clone()))
                    .expect("sen_menu err");
            }
            if event_key_down(Key::from_char('z')) {
                if cnt == 0{
                    cnt = list.len();
                }
                cnt -= 1;
                sen_list
                    .send((list[cnt].clone(), wins.w(), wins.h()))
                    .expect("sen_list err");
                sen_menu
                    .send((false, file_info_list.get(cnt).unwrap().clone()))
                    .expect("sen_menu err");
            }
            if event_key_down(Key::from_char('x')) {
                sen_menu.send((true,file_info_list.get(cnt).unwrap().clone())).expect("sen_menu err");
            }
            true
        }
        _ => false,
    });
}
