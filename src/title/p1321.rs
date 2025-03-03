fn main() {
    let mut s_temp = String::new();
    std::io::stdin()
        .read_line(&mut s_temp)
        .unwrap();
    s_temp=s_temp.trim().to_string();
    let mut cnt_boy = 0;
    let mut cnt_girl = 0;
    let c_temp:Vec<char> = s_temp.chars().collect();
    for (i,c) in c_temp.iter().enumerate(){
        if *c == 'b' {
            cnt_boy+=1;
        }
        if *c == 'o' && c_temp[i-1] != 'b'{
            cnt_boy+=1;
        }
        if *c == 'y' && c_temp[i-1] != 'o'{
            cnt_boy+=1;
        }


        if *c == 'g' {
            cnt_girl+=1;
        }
        if *c == 'i' && c_temp[i-1] != 'g'{
            cnt_girl+=1;
        }
        if *c == 'r' && c_temp[i-1] != 'i'{
            cnt_girl+=1;
        }
        if *c == 'l' && c_temp[i-1] != 'r'{
            cnt_girl+=1;
        }
    }
    println!("{}\n{}",cnt_boy,cnt_girl);
}