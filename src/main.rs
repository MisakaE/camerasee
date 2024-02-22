use cae::{exif::{parse, ExifInfo}, to_fracs_ii, to_fracu_ii, to_u32_ii};
use fltk::{
    app,
    frame::Frame,
    image::JpegImage,
    prelude::{GroupExt, ImageExt, WidgetBase, WidgetExt},
    window::Window,
};
use std::{collections::HashMap, fs::File, io::Read};
fn main() {
    let data = File::open("1.JPG").unwrap();
    let y = ExifInfo::get(data);
    println!("{}",y.make.unwrap());
    println!("{}",y.exposure_compensation.unwrap().to_float());
    /* 
    let mut data = File::open("1.JPG").unwrap();
    /* */
    let mut dat = vec![];
    data.read_to_end(&mut dat).unwrap();
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 600, 600, "D");
    let mut frame = Frame::default();
    let mut image = JpegImage::from_data(&dat).unwrap();
    image.scale(6000, 6000, true, true);
    frame.set_image(Some(image));
    frame.center_of(&wind);
    
    wind.make_resizable(true);
    wind.end();
    wind.show();
    
    app.run().unwrap();
    */
    /*
    //let mut dat = data.bytes();
    let y = parse(data);
    for i in y{
        print!("{:#02X} {} {} [",i.get().0,i.get().1,i.get().2);
        if i.get().1 == 2{
            let mut ve = i.get().3;
            ve.pop();
            let st = String::from_utf8(ve).unwrap();
            print!(" {} ",st)
        }
        else if i.get().1 == 5&&i.get().2==1{
            let fr = to_fracu_ii(&i.get().3);
            print!("{}/{}",fr.get().2,fr.get().1);
        }
        else if i.get().1 == 4&&i.get().2==1{
            let fr = to_u32_ii(&i.get().3);
            print!("{}",fr);
        }
        else if i.get().1 == 10&&i.get().2==1{
            let fr = to_fracs_ii(&i.get().3);
            if fr.get().0{
                print!("-");
            }
            print!("{}/{}",fr.get().2,fr.get().1);
        }
        else if i.get().1 == 3{
            print!("{}",to_u32_ii(&i.get().3));
        }
        else{
            for j in i.get().3{
                print!("{:02X} ",j);
            }
        }

        println!("]");
    }

    /*
    for _ in 0 .. 22{
        print!("{:02X} ",dat.next().unwrap().unwrap());
    }
    for _ in 0.. 11{
        println!("");
        for _ in 0..12{
            print!("{:02X} ",dat.next().unwrap().unwrap());
        }
    }
    */
    */
}