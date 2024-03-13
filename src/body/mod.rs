pub mod main_ui;
pub mod menu_ui;
pub mod image_printer;
pub mod control;
pub enum Command {
    Move((i32,i32)),//x,y
    Resize(i32),//-+
    Switch(bool)//0:- 1:+
}