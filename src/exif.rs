use crate::{to_fracs_ii, to_fracu_ii, to_u32_ii, Frac};
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
#[derive(Clone)]
pub struct ExifInfo {
    pub make: Option<String>,                //相机信息 0x10F 0x110
    pub date_time: Option<String>,           //拍摄日期 0x9003
    pub exposure_compensation: Option<Frac>, //曝光补偿 0x9204
    pub exposure_time: Option<Frac>,         //快门速度 0x829A
    pub f_number: Option<Frac>,              //光圈大小 0x829D
    pub offset_time: Option<String>,         //零时区时差 0x9011
    pub iso:Option<u32>,                    //0x8827
    pub focal_length: Option<Frac>,          //焦距 0x920A
    pub metering_mode: Option<u32>,          //测光模式 0x9207
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
    pub exif_image_width: Option<u32>,  //0xA002
    pub exif_image_height: Option<u32>, //0xA003
    pub color_space: Option<u32>,       //色彩空间 0xA001
    pub lens_model: Option<String>,     //镜头模型0xA434
}
impl ExifInfo {
    pub fn get(file: File) ->ExifInfo{
        let map = parse(file);
        /* make 0x11F 0x110 */
        let make_1 = map.get(&0x10F);
        let make_2 = map.get(&0x110);
        let make = if make_1.is_none()|| make_2.is_none(){
            None
        } else {
            let mut vecc = make_1.unwrap().val.clone();
            vecc.pop();
            let make_1 = String::from_utf8(vecc).unwrap();
            let mut vecc = make_2.unwrap().val.clone();
            vecc.pop();
            let make_2 = String::from_utf8(vecc).unwrap();
            Some(format!("{} {}",make_1,make_2))
        };

        /* date_time 0x9003 */
        let date_time = map.get(&0x9003);
        let date_time = if date_time.is_none(){
            None
        } else {
            let mut vecc = date_time.unwrap().val.clone();
            vecc.pop();
            Some(String::from_utf8(vecc).unwrap())
        };

        /* offset_time 0x9011 */
        let offset_time = map.get(&0x9011);
        let offset_time = if offset_time.is_none(){
            None
        } else {
            let mut vecc = offset_time.unwrap().val.clone();
            vecc.pop();
            Some(String::from_utf8(vecc).unwrap())
        };

        let lens_model = map.get(&0xA434);
        let lens_model = if lens_model.is_none(){
            None
        } else {
            let mut vecc = lens_model.unwrap().val.clone();
            vecc.pop();
            Some(String::from_utf8(vecc).unwrap())
        };



        let exposure_compensation = if map.get(&0x9204).is_none(){
            None
        } else {
            Some(to_fracs_ii(&map.get(&0x9204).unwrap().val))
        };

        let exposure_time = if map.get(&0x829A).is_none(){
            None
        } else {
            Some(to_fracu_ii(&map.get(&0x829A).unwrap().val))
        };

        let f_number = if map.get(&0x829D).is_none(){
            None
        } else {
            Some(to_fracu_ii(&map.get(&0x829D).unwrap().val))
        };

        let iso = if map.get(&0x8827).is_none(){
            None
        } else {
            Some(to_u32_ii(&map.get(&0x8827).unwrap().val))
        };

        let focal_length = if map.get(&0x920A).is_none(){
            None
        } else {
            Some(to_fracu_ii(&map.get(&0x920A).unwrap().val))
        };

        let metering_mode = if map.get(&0x9207).is_none(){
            None
        } else {
            Some(to_u32_ii(&map.get(&0x9207).unwrap().val))
        };

        let exif_image_width = if map.get(&0xA002).is_none(){
            None
        } else {
            Some(to_u32_ii(&map.get(&0xA002).unwrap().val))
        };

        let exif_image_height = if map.get(&0xA003).is_none(){
            None
        } else {
            Some(to_u32_ii(&map.get(&0xA003).unwrap().val))
        };

        let color_space = if map.get(&0xA001).is_none(){
            None
        } else {
            Some(to_u32_ii(&map.get(&0xA001).unwrap().val))
        };
        

        ExifInfo {
            make,
            date_time,
            exposure_compensation,
            exposure_time,
            f_number,
            offset_time,
            iso,
            focal_length,
            metering_mode,
            exif_image_width,
            exif_image_height,
            color_space,
            lens_model,
        }
    }
}
pub fn parse(mut file: File) -> HashMap<u32, Directory> {
    let mut map: HashMap<u32, Directory> = HashMap::new();
    let mut data = vec![];
    file.read_to_end(&mut data).unwrap();
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
