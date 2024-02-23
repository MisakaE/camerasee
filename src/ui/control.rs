use std::{sync::mpsc::Sender, thread};

use fltk::{
    app::{event, event_key_down, sleep},
    enums::{Event, Key},
    prelude::WidgetBase,
    window::DoubleWindow,
};
pub fn direction(sen_move: Sender<(i32, i32, i32)>) {
    thread::spawn(move || loop {
        if event() == Event::KeyDown {
            let mut x_mov = 0;
            let mut y_mov = 0;
            let mut size = 0;
            if event_key_down(Key::from_char('a')) {
                x_mov -= 15;
            }
            if event_key_down(Key::from_char('d')) {
                x_mov += 15;
            }
            if event_key_down(Key::from_char('s')) {
                y_mov += 15;
            }
            if event_key_down(Key::from_char('w')) {
                y_mov -= 15;
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
        }
        sleep(0.05);
    });
}
pub fn lists(mut wind: DoubleWindow,list:Vec<String>,sen_list:Sender<String>) {
    let mut cnt = 1;

    wind.handle(move |_, event| match event {
        Event::KeyDown => {
            
            if event_key_down(Key::from_char('c')) {
                cnt+=1;
                if cnt == list.len(){
                    cnt = 0;
                }
                sen_list.send(list[cnt].clone()).expect("sen_list err");
            }
            if event_key_down(Key::from_char('z')) {
                cnt-=1;
                if cnt == 0{
                    cnt = list.len()-1;
                }
                sen_list.send(list[cnt].clone()).expect("sen_list err");
            }
            if event_key_down(Key::from_char('x')) {

            }
            true
        }
        _ => false,
    });
}
