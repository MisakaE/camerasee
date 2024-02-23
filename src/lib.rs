pub struct Frac{
    sign:bool,
    num:u32,
    deon:u32,
}
impl Frac {
    pub fn new(signs:bool,nums:u32,denos:u32)->Frac{
        Frac { sign: signs, num: nums, deon: denos }
    }
    pub fn get(&self) -> (bool,u32,u32){
        (self.sign,self.deon,self.num)
    }
    pub fn to_float(&self) -> f32{
        let mut sign = 1;
        if self.sign{
            sign*=-1;
        }
        (self.num as f32)/(self.deon as f32)*(sign as f32)
    }
    pub fn to_string(&self)-> String{
        if self.sign{
            format!("-{}/{}",self.num,self.deon)
        } else {
            format!("{}/{}",self.num,self.deon)
        }
        
    }
}
pub fn to_fracs_ii(hex: &Vec<u8>)->Frac{
    let mut sign = false;
    let num = vec![hex[0],hex[1],hex[2],hex[3]];
    let deno = vec![hex[4],hex[5],hex[6],hex[7]];
    let mut num = to_i32_ii(&num);
    let mut deno = to_i32_ii(&deno);
    if num < 0{
        num *= -1;
        sign=!sign;
    }
    if deno < 0{
        deno *= -1;
        sign=!sign;
    }
    Frac::new(sign, num as u32, deno as u32)
}
pub fn to_fracu_ii(hex:&Vec<u8>) -> Frac{
    let num = vec![hex[0],hex[1],hex[2],hex[3]];
    let deno = vec![hex[4],hex[5],hex[6],hex[7]];
    Frac::new(false, to_u32_ii(&num), to_u32_ii(&deno))
}
pub fn to_u32_ii(hex: &Vec<u8>) -> u32 {
    let mut num:u32 = 0;
    let mut cnt:u64 = 1;
    //println!("TE");
    for i in hex {
        num += (*i as u32) * (cnt as u32);
        cnt *= 256;
        //println!("TE");
    }
    num
}
pub fn to_i32_ii(hex: &Vec<u8>) -> i32{
    let bcod:u64 = 1<<32;
    if (hex[3]|128)==1{
        return ((bcod-to_u32_ii(&hex) as u64)as i32)*-1;
    }
    to_u32_ii(&hex) as i32

}
pub mod exif;
pub mod ui;