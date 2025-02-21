fn p1320() -> Result<(), Box<dyn std::error::Error>> {
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp)?;
    let c: Vec<char> = s_temp.trim().chars().collect();
    let mut num_vec: Vec<i32> = c.iter().map(|i| ((*i as u8) as i32) - ((b'0' as u8) as i32)).collect();
    let n= num_vec.len();
    let mut ans_vec:Vec<i32>=Vec::new();
    let mut cnt=0;
    let mut temp_i = num_vec[0];
    if temp_i == 1{
        ans_vec.push(0);
    }
    for _ in 1..n {
        for &i in num_vec.iter() {
            if i==temp_i {
                cnt+=1;
            }
            else{
                ans_vec.push(cnt);
                cnt=1;
                temp_i=i;
            }
        }
        let mut s_temp = String::new();
        std::io::stdin().read_line(&mut s_temp)?;
        let c: Vec<char> = s_temp.trim().chars().collect();
        num_vec.clear();
        num_vec = c.iter().map(|i| ((*i as u8) as i32) - ((b'0' as u8) as i32)).collect();
    }
    for &i in num_vec.iter() {
        if i==temp_i {
            cnt+=1;
        }
        else{
            ans_vec.push(cnt);
            cnt=1;
            temp_i=i;
        }
    }
    ans_vec.push(cnt);
    print!("{} ",n);
    ans_vec.iter().for_each(|i| print!("{} ",i));
    Ok(())
}