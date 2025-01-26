fn p1427() {
    let mut s_temp:String=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let s:Vec<&str>=s_temp.trim().split_whitespace().collect();
    for i in (0..s.len()-1).rev(){
        print!("{} ",s[i]);
    }
}