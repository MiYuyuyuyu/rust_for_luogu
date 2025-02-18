fn p5732() -> Result<(), Box<dyn std::error::Error>> {
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp)?;
    let n = s_temp.trim().parse::<usize>()?;
    let mut ans:Vec< Vec<i32> >=vec![ vec![0;n+1] ; n+1 ];
    ans[0][1]=1;
    for i in 1..=n{
        for j in 1..=i{
            ans[i][j]=ans[i-1][j]+ans[i-1][j-1];
        }
    }
    // println!("{:?}",ans);
    for i in 1..=n{
        for j in 1..=i {
            print!("{} ",ans[i][j]);
        }
        println!();
    }
    Ok(())
}