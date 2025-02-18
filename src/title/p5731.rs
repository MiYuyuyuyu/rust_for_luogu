fn p5731() -> Result<(), Box<dyn std::error::Error>> {
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp)?;
    let n = s_temp.trim().parse::<usize>()?;
    let mut ans: Vec<Vec<String>> = vec![vec![String::new(); n]; n];
    let n_i32 = n as i32;
    let xy: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 0], vec![0, -1], vec![-1, 0]]; // -> v <- ^
    let mut xy_num: usize = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for i in 1..=n.pow(2) {
        let temp = format!("{:3}", i);
        ans[x as usize][y as usize] = temp;
        // 更新坐标
        let mut next_x = x + xy[xy_num][0];
        let mut next_y = y + xy[xy_num][1];

        // 检查是否超出边界或已填充
        if next_x >= n_i32 || next_x < 0 || next_y >= n_i32 || next_y < 0 || !ans[next_x as usize][next_y as usize].is_empty() {
            xy_num = (xy_num + 1) % 4;
            next_x = x + xy[xy_num][0];
            next_y = y + xy[xy_num][1];
        }

        // 更新坐标
        x = next_x;
        y = next_y;
        
    }

    // println!("{:?}", ans);
    for i in ans{
        i.iter().for_each(|i| print!("{}",i));
        println!();
    }
    Ok(())
}