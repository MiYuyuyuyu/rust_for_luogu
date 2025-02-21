fn p1319() -> Result<(), Box<dyn std::error::Error>> {
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp)?;
    let c: Vec<&str> = s_temp.trim().split_whitespace().collect();
    let n = c[0].parse::<usize>()?;
    let v:Vec<i32>=c[1..].iter().map(|i| i.parse().expect("err")).collect();
    let mut ans:Vec<i32>=vec![ 0;(n).pow(2)];
    let mut cnt =0 ; 
    let mut b:bool=false;
    for i in v {
        for _ in 0..i{
            if b {
                ans[cnt]+=1;
            }
            cnt+=1;
        }
        b = ! b;
    }
    // println!("{:?}",ans);
    cnt=0;
    for _ in 0..n{
        for _ in 0..n{
            print!("{}",ans[cnt]);
            cnt+=1;
        }
        println!();
    }
    Ok(())
}