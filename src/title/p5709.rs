use std::io::stdin;
fn p5709() {
    //m t s 50 10 200
    // 97 8 17
    let mut s_temp:String=String::new();
    stdin().read_line(& mut s_temp).expect("in s_temp err");
    let c_temp:Vec<&str>=s_temp.trim().split_whitespace().collect();
    let m:i32=match c_temp[0].parse() {
        Ok(num)=>num,
        Err(_)=>{
            println!("in m err");
            50
        }
    };
    let t:i32=match c_temp[1].parse() {
        Ok(num)=>num,
        Err(_)=>{
            println!("in m err");
            10
        }
    };
    let s:i32=match c_temp[2].parse() {
        Ok(num)=>num,
        Err(_)=>{
            println!("in m err");
            200 
        }
    };
    if t==0 {
        println!("0");
        return;
    }
        
    let eat_num:i32=(s+t-1)/t;
    if eat_num >= m{
        println!("0");
    }
    else {
        println!("{}",m-eat_num);
    }
}