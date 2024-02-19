use crate::{to_u32_ii, Frac};
use std::{collections::HashMap, fs::File, io::Read};
//#[derive(Debug)]
pub struct Directory {
    tag: u32,
    valtyp: u32,
    valnum: u32,
    val: Vec<u8>,
}
impl Directory {
    pub fn new(tags: u32, valtyps: u32, valnums: u32, vals: Vec<u8>) -> Self {
        Directory {
            tag: tags,
            valtyp: valtyps,
            valnum: valnums,
            val: vals,
        }
    }
    pub fn get(&self) -> (u32, u32, u32, Vec<u8>) {
        (self.tag, self.valtyp, self.valnum, self.val.clone())
    }
}
pub struct ExifInfo {
    make: String,                //相机信息 0x10F 0x110
    date_time: String,           //拍摄日期 0x9003
    exposure_compensation: Frac, //曝光补偿 0x9204
    exposure_time: Frac,         //快门速度 0x829A
    f_number: Frac,              //光圈大小 0x829D
    offset_time: String,         //零时区时差 0x9011
    iso: u32,                      //0x8827
    focal_length: Frac,          //焦距 0x920A
    metering_mode: u32,          //测光模式 0x9207
                        /*
                            1 = Monochrome area
                            2 = One-chip color area
                            3 = Two-chip color area
                            4 = Three-chip color area
                            5 = Color sequential area
                            6 = Monochrome linear
                            7 = Trilinear
                            8 = Color sequential linear
                        */
    exif_image_width:u32, //0xA002
    exif_image_height:u32, //0xA003
    color_space:u32,        //色彩空间 0xA001
    lens_model:String, //镜头模型0xA434
}
impl ExifInfo {
    pub fn get(file:File){
        let map = parse(file);
        let make_1 = map.get(&0x10F).unwrap();
        let make_2 = map.get(&0x110).unwrap();
        let date_time = map.get(&0x9003).unwrap();
        let exposure_compensation = map.get(&0x9204).unwrap();
        let exposure_time = map.get(&0x829A).unwrap();
        let f_number = map.get(&0x829D).unwrap();
        let offset_time = map.get(&0x9011).unwrap();
        let iso = map.get(&0x8827).unwrap();
        let focal_length = map.get(&0x920A).unwrap();
        let metering_mode = map.get(&0x9207).unwrap();
        let exif_image_width = map.get(&0xA002).unwrap();
        let exif_image_height = map.get(&0xA003).unwrap();
        let color_space = map.get(&0xA001).unwrap();
        let lens_model = map.get(&0xA434).unwrap();

        let mut vecc = vec![];
        vecc = make_1.val.clone();
        vecc.pop();
        let make_1 = String::from_utf8(vecc).unwrap();
        print!("{}",make_1);

    }
}
pub fn parse(mut file: File) -> HashMap<u32,Directory> {
    let mut map :HashMap<u32,Directory>= HashMap::new();
    let mut data = vec![];
    file.read_to_end(&mut data).unwrap();
    //let mut s = sta_it.skip(22);
    //let
    let mut cnt = 0;
    cnt += 12;
    cnt += 8;
    let mut n = 0;
    let mut nxt = 20;

    while nxt != 0 {
        for _ in 0..1 {
            let mut v = Vec::new();
            v.push(data[cnt]);
            cnt += 1;
            v.push(data[cnt]);
            cnt += 1;
            n = to_u32_ii(&v);
        }
        nxt = 0;
        for _ in 0..n {
            let mut v = Vec::new();
            v.push(data[cnt]);
            cnt += 1;
            v.push(data[cnt]);
            cnt += 1;
            let tags = to_u32_ii(&v);

            v.clear();
            v.push(data[cnt]);
            cnt += 1;
            v.push(data[cnt]);
            cnt += 1;
            let valtyps = to_u32_ii(&v);
            v.clear();

            for _ in 0..4 {
                v.push(data[cnt]);
                cnt += 1;
            }
            let valnums = to_u32_ii(&v);
            let truenum = valnums * check_byts(valtyps);
            let mut vals = Vec::new();
            for _ in 0..4 {
                vals.push(data[cnt]);
                cnt += 1;
            }
            if truenum > 4 && tags != 0x927c {
                vals = find_data(&data, to_u32_ii(&vals), truenum);
            }
            if tags == 0x8769 {
                nxt = to_u32_ii(&vals);
            }
            let dy = Directory::new(tags, valtyps, valnums, vals);
            //list.push(dy);
            map.insert(dy.tag, dy);
        }
        cnt = nxt as usize + 12;
    }
    map
}
fn find_data(data: &Vec<u8>, ops: u32, num: u32) -> Vec<u8> {
    let mut val = Vec::new();
    let mut cnt = 0;
    for _ in 0..num {
        val.push(data[(ops + 12 + cnt) as usize]);
        cnt += 1;
    }
    val
}

fn check_byts(typ: u32) -> u32 {
    match typ {
        1 | 2 | 6 | 7 => 1,
        8 => 2,
        3 | 9 => 3,
        4 | 11 => 4,
        5 | 10 | 12 => 8,
        _ => 0,
    }
}
