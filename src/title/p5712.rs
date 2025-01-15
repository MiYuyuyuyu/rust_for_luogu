fn p5712(){
    let mut s_temp=String::new();
    std::io::stdin()
        .read_line(&mut s_temp)
        .expect("in s_temp err");
    let eat_apple_num:i32=match s_temp.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("s_temp to year err");
            0
        }
    };
    if eat_apple_num <= 1 {
        println!("Today, I ate {eat_apple_num} apple.");
    }
    else{
        println!("Today, I ate {} apples.",eat_apple_num);
    }
}