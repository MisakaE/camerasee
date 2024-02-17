use std::{
    fs::File,
    io::Read,
};
use crate::to_u32_ii;
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
    pub fn get(&self) ->(u32,u32,u32,Vec<u8>){
        (self.tag,self.valtyp,self.valnum,self.val.clone())
    }
}

pub fn parse(mut file: File) ->Vec<Directory>{
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
            if truenum > 4&&tags!=0x927c{
                vals = find_data(&data, to_u32_ii(&vals), truenum); 
            }
            if tags == 0x8769 {
                nxt = to_u32_ii(&vals);
            }
            let dy = Directory::new(tags, valtyps, valnums, vals);
            list.push(dy);
        }
        cnt = nxt as usize +12;
    }
    list
}
fn find_data(data:&Vec<u8>,ops:u32,num:u32) ->Vec<u8>{
    let mut val = Vec::new();
    let mut cnt = 0 ;
    for _ in 0..num{
        val.push(data[(ops+12+cnt) as usize]);
        cnt += 1;
    }
    val
}





fn check_byts(typ:u32)->u32{
    match typ{
        1|2|6|7 => 1,
        8 =>2,
        3|9 =>3,
        4|11 =>4,
        5|10|12 =>8,
        _ => 0,
    }

}
