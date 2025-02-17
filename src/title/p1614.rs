use std::cmp::min;
fn p1614() -> Result<(), Box<dyn std::error::Error>> {
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp)?;
    let c:Vec<&str>=s_temp.trim().split_whitespace().collect();
    let a=c[0].parse::<usize>()?;
    let b=c[1].parse::<usize>()?;
    let mut ans:Vec<i32>=vec![0;a];
    for i in 0..a{
        s_temp.clear();
        std::io::stdin().read_line(&mut s_temp)?;
        let temp = s_temp.trim().parse()?;
        ans[i]=temp;
    }
    let mut min_num=0;
    for i in 0..b {
        min_num+=ans[i];
    }
    let mut min_ans=min_num;
    for i in b..a{
        min_num+=ans[i];
        min_num-=ans[i-b];
        min_ans=min(min_ans,min_num);
    }
    println!("{}",min_ans);
    Ok(())
}