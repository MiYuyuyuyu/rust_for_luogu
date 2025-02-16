use std::vec;

fn p5729() -> Result<(), Box<dyn std::error::Error>> {
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp)?;
    let c: Vec<&str> = s_temp.trim().split_whitespace().collect();
    let all_pick: Vec<usize> = c.iter()
        .map(|i| i.parse::<usize>().expect("err"))
        .collect();

    let w = all_pick[0]; 
    let x = all_pick[1]; 
    let h = all_pick[2]; 
    let mut all_bool: Vec<Vec<Vec<bool>>> = vec![vec![vec![true; h]; x]; w];

    s_temp.clear();
    std::io::stdin().read_line(&mut s_temp)?;
    let n = s_temp.trim().parse::<usize>()?;

    for _ in 0..n {
        s_temp.clear();
        std::io::stdin().read_line(&mut s_temp)?;
        let c: Vec<&str> = s_temp.trim().split_whitespace().collect();
        let temp: Vec<usize> = c.iter().map(|i| i.parse::<usize>().expect("err")).collect();

        let x1 = temp[0]; 
        let y1 = temp[1]; 
        let z1 = temp[2]; 
        let x2 = temp[3]; 
        let y2 = temp[4]; 
        let z2 = temp[5]; 

        let xi_start = x1 - 1;
        let xi_end = x2;
        let yi_start = y1 - 1;
        let yi_end = y2;
        let zi_start = z1 - 1;
        let zi_end = z2;

        for i in xi_start..xi_end {
            for j in yi_start..yi_end {
                for k in zi_start..zi_end {
                    all_bool[i][j][k] = false;
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..w {
        for j in 0..x {
            for k in 0..h {
                if all_bool[i][j][k] {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
    Ok(())
}