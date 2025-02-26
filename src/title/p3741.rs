fn main() {
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let n = s_temp.trim().parse::<usize>().expect("err");
    s_temp.clear();
    std::io::stdin().read_line(&mut s_temp).expect("err");

    let s_trimmed = s_temp.trim();
    let mut c_vec: Vec<char> = s_trimmed.chars().collect();
    let mut cnt = 0;
    let mut bool_cnt = false;

    // 第一遍遍历，统计 "VK" 的数量
    for i in 0..c_vec.len() - 1 {
        if c_vec[i] == 'V' && c_vec[i + 1] == 'K' {
            cnt += 1;
            c_vec[i] = 'X'; // 标记已处理过的 "VK"
            c_vec[i + 1] = 'X';
        }
    }

    // 第二遍遍历，检查是否有可以转换为 "VK" 的情况
    for i in 0..c_vec.len() - 1 {
        if c_vec[i] == 'V' && c_vec[i + 1] == 'V' {
            bool_cnt = true;
            break;
        } else if c_vec[i] == 'K' && c_vec[i+1]=='K' {
            bool_cnt = true;
            break;
        }
    }

    if bool_cnt {
        cnt += 1;
    }

    println!("{}", cnt);
}