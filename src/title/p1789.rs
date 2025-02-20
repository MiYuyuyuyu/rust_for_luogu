fn p1789() -> Result<(), Box<dyn std::error::Error>> {
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp)?;
    let c: Vec<&str> = s_temp.trim().split_whitespace().collect();
    // n m k
    let n = c[0].parse::<usize>()?;
    let m = c[1].parse::<i32>()?;
    let k = c[2].parse::<i32>()?;
    let mut vec_bool: Vec<Vec<bool>> = vec![vec![false; n]; n];

    for _ in 0..m {
        s_temp.clear();
        std::io::stdin().read_line(&mut s_temp)?;
        let c: Vec<&str> = s_temp.trim().split_whitespace().collect();
        let x = (c[0].parse::<usize>()?)-1;
        let y = (c[1].parse::<usize>()?)-1;

        // 处理 x-2 到 x+2 的范围
        for i in (x as isize - 2)..=(x as isize + 2) {
            if i >= 0 && i < n as isize {
                if !vec_bool[i as usize][y] {
                    vec_bool[i as usize][y] = true;
                }
            }
        }

        // 处理 y-2 到 y+2 的范围
        for j in (y as isize - 2)..=(y as isize + 2) {
            if j >= 0 && j < n as isize {
                if !vec_bool[x][j as usize] {
                    vec_bool[x][j as usize] = true;
                }
            }
        }

        // 处理对角线上的点
        if x > 0 && y > 0 && !vec_bool[x - 1][y - 1] {
            vec_bool[x - 1][y - 1] = true;
        }
        if x + 1 < n && y > 0 && !vec_bool[x + 1][y - 1] {
            vec_bool[x + 1][y - 1] = true;
        }
        if x > 0 && y + 1 < n && !vec_bool[x - 1][y + 1] {
            vec_bool[x - 1][y + 1] = true;
        }
        if x + 1 < n && y + 1 < n && !vec_bool[x + 1][y + 1] {
            vec_bool[x + 1][y + 1] = true;
        }
    }

    for _ in 0..k {
        s_temp.clear();
        std::io::stdin().read_line(&mut s_temp)?;
        let c: Vec<&str> = s_temp.trim().split_whitespace().collect();
        let x = (c[0].parse::<usize>()?)-1;
        let y = (c[1].parse::<usize>()?)-1;

        // 处理 x-2 到 x+2 和 y-2 到 y+2 的范围
        for i in (x as isize - 2)..=(x as isize + 2) {
            if i >= 0 && i < n as isize {
                for j in (y as isize - 2)..=(y as isize + 2) {
                    if j >= 0 && j < n as isize {
                        if !vec_bool[i as usize][j as usize] {
                            vec_bool[i as usize][j as usize] = true;
                        }
                    }
                }
            }
        }
    }
    // println!("{:#?}",vec_bool);
    let mut cnt=0;
    for i in vec_bool{
        for j in i {
            if !j {
                cnt+=1;
            }
        }
    }
    println!("{}",cnt);
    Ok(())
}