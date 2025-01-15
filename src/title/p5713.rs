fn p5713(){
    let mut s_temp=String::new();
    std::io::stdin()
        .read_line(&mut s_temp)
        .expect("in s_temp err");
    let question_num:i32=match s_temp.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("s_temp to year err");
            0
        }
    };
    let local=5*question_num;
    let luogu=11+3*question_num;
    match local {
        _ if local < luogu =>{
            println!("Local");
        }
        _ =>{
            println!("Luogu");
        }
    };
}