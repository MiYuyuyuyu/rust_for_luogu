fn p5728() ->Result<(),Box<dyn std::error::Error>> {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp)?;
    let n:usize=s_temp.trim().parse()?;
    let mut cnt=0;
    let mut vec_n:Vec<Vec<i32>>=vec![vec![0;3];n as usize];
    let mut sum_num:Vec<i32>=vec![0;n];
    for i in 0..n{
        s_temp.clear();
        std::io::stdin().read_line(&mut s_temp)?;
        let c:Vec<&str>=s_temp.trim().split_whitespace().collect();
        let t:Vec<i32>=c.iter().map(|x| x.parse::<i32>().expect("err")).collect();
        for j in 0..3 {
            vec_n[i][j]=t[j];
            sum_num[i]+=t[j];
        }
    }
    for i in 0..n {
        't : for j in i..n {
            if i==j {
                continue;
            }
            else {
                let mut temp:i32;
                for k in 0..3 {
                    temp=(vec_n[i][k]-vec_n[j][k]).abs();
                    if temp > 5{
                        continue 't;
                    }
                }
                if (sum_num[i]-sum_num[j]).abs() > 10 {
                    continue 't;
                }
                else {
                    cnt+=1;
                }
            }
        }
    }
    println!("{}",cnt);
    Ok(())
}