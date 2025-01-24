fn p1217() {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let s_temp:Vec<&str>=s_temp.trim().split_whitespace().collect();
    let a: usize=s_temp[0].parse().expect("err");
    let b:usize=s_temp[1].parse().expect("err");
    let mut zs_bool:Vec<bool>=vec![true;b as usize +1];
    for i in 2..b+1{
        if zs_bool[i]{
            let mut temp=i;
            loop{
                temp+=i;
                if temp > b {
                    break;
                }
                zs_bool[temp]=false;
            }
            if a<=i && i<=b{
                let s1=i.to_string();
                let s2:String=s1.chars().rev().collect();
                if s1==s2 {
                    println!("{}",i);
                }
            }
        }
    }
}