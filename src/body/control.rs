use std::{sync::mpsc::Sender, thread};
use fltk::app::{event_key_down, sleep};
use fltk::enums::Key;
use super::Command;

pub fn move_lis(sen:Sender<Command>){
    let speed = 20;
    sen.send(Command::Move((0,0))).unwrap();
    thread::spawn(move ||loop{
        let mut x_mov =0;
        let mut y_mov = 0;
        if event_key_down(Key::from_char('d')) {
            x_mov -= speed;
        }
        if event_key_down(Key::from_char('a')) {
            x_mov += speed;
        }
        if event_key_down(Key::from_char('w')) {
            y_mov -= speed;
        }
        if event_key_down(Key::from_char('s')) {
            y_mov += speed;
        }
        if x_mov!=0||y_mov!=0{
            sen.send(Command::Move((x_mov,y_mov))).unwrap();
        }
        sleep(0.02)
    });
}
fn list_lis(sen:Sender<Command>){
    thread::spawn(move ||loop{

    });
}
pub fn resize_lis(sen:Sender<Command>){
    thread::spawn(move ||loop{
        let mut resize = 0;
        if event_key_down(Key::from_char('e')) {
            resize += 50;
        }
        if event_key_down(Key::from_char('q')) {
            resize -= 50;
        }
        if resize!=0{
            sen.send(Command::Resize(resize)).unwrap();
        }
        sleep(0.02)
    });
}