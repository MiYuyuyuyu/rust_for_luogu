use std::collections::HashMap;
use std::cmp::min;
fn p2911() -> Result<(), Box<dyn std::error::Error>> {
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp)?;
    let c:Vec<&str>=s_temp.trim().split_whitespace().collect();
    let a=c[0].parse::<usize>()?;
    let b=c[1].parse::<usize>()?;
    let c=c[2].parse::<usize>()?;
    let mut mp:HashMap<usize,usize>=HashMap::new();
    let mut max_num:usize=100;
    let mut max_ans:usize=0;
    for i in 1..=a{
        for j in 1..=b{
            for k in 1..=c{
                let temp=mp.entry(i+j+k).or_insert(0);
                *temp += 1;
                if max_ans < *temp{
                    max_ans=*temp;
                    max_num=i+j+k;
                }
                else if max_ans==*temp{
                    max_num=min(i+j+k,max_num);
                }
            }
        }
    }
    // println!("{:?}",mp);
    println!("{}",max_num);
    Ok(())
}