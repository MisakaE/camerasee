use crate::{to_u32_ii, Frac};
use std::{fs::File, io::Read};
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
    iso: u32,
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
pub fn parse(mut file: File) -> Vec<Directory> {
    let mut list: Vec<Directory> = Vec::new();
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
            list.push(dy);
        }
        cnt = nxt as usize + 12;
    }
    list
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
