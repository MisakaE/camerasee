use std::fs::{self, File};
use std::env;
use std::io::Error;

use regex::Regex;

use crate::exif::ExifInfo;
use std::path::Path;
#[derive(Clone)]
pub struct FileInfo{
    pub path:String,
    pub exif:ExifInfo,
    pub len:u64,
}
pub fn get_all(current_path:String) -> (Vec<FileInfo>,Vec<String>){
    let mut list :Vec<FileInfo>= Vec::new();
    let mut list_path:Vec<String>=Vec::new();
    for entry in fs::read_dir(current_path).expect("get dir err!"){
        let entry = entry.unwrap();
        //[\.][A-Z]*[a-z]*$
        let re = Regex::new(r"[\.][A-Z]*[a-z]*$").unwrap();
        let path_str = entry.path().display().to_string();
        let pat = re.find(&path_str);
        if pat.is_none(){
            continue;
        }
        match pat.unwrap().as_str() {
            ".JPG"|".jpg"|".JPEG"|".jpeg" => (),
            _ => continue
        }

        let file = File::open(entry.path()).unwrap();
        let lens = fs::metadata(entry.path()).unwrap().len();
        let exif_info = ExifInfo::get(file);
        list_path.push(entry.path().display().to_string());
        list.push(FileInfo { path: entry.path().display().to_string(), exif: exif_info, len: lens });
    }
    (list,list_path)
}
pub fn parameters() -> Result<(bool,String),Error>{
    let para:Vec<String> = env::args().collect();
    if para.len()<=1{
        return Err(Error::new(std::io::ErrorKind::NotFound,"Error"));
    }
    if para.len() == 2{
        if Path::new(para[1].as_str()).is_file(){
            return Ok((false,para[1].clone()));
        }
    }
    if para.len() == 3{
        if para[1] == "-d"{
            if Path::new(para[2].as_str()).is_dir(){
                return Ok((true,para[2].clone()));
            }
        }
    }
    Err(Error::new(std::io::ErrorKind::NotFound,"Error"))
}