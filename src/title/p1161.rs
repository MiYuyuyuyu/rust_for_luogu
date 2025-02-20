fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp)?;
    let n=s_temp.trim().parse::<usize>()?;
    let mut mp:Vec<bool>=vec![false;1000000];
    for _ in 0..n {
        s_temp.clear();
        std::io::stdin().read_line(&mut s_temp)?;
        let c:Vec<&str>=s_temp.trim().split_whitespace().collect();
        let a = c[0].parse::<f64>()?;
        let t = c[1].parse::<f64>()?;
        for i in 1..=(t as i32) {
            let temp=(a*(i as f64)) as usize;
            mp[temp] = !mp[temp];
        }
    }
    let mut k=0;
    mp.iter().for_each(|&i| {
        k+=1;
        if i {
            println!("{}",k-1);
        }
    });
    Ok(())
}