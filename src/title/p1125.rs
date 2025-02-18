use std::collections::HashMap;

fn p1125() -> Result<(), Box<dyn std::error::Error>> {
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp)?;
    let mut mp:HashMap<char,i32>=HashMap::new();
    for i in s_temp.trim().chars(){
        let k=mp.entry(i).or_insert(0);
        *k+=1;
    }
    let (mut min_num,mut max_num):(i32,i32)=(1000000000,1);
    for (_,k) in mp{
        min_num=std::cmp::min(min_num, k);
        max_num=std::cmp::max(max_num, k);
    }
    let num=max_num-min_num;
    if zs(num) {
        println!("Lucky Word");
        println!("{}",num);
    }
    else {
        println!("No Answer");
        println!("0");
    }
    Ok(())
}

fn zs(a:i32)->bool {
    if a<2{
        return  false;
    }
    if a==2{
        return true;
    }
    for i in 2..a{
        if a/i==0{
            return false;
        }
    }
    true
}