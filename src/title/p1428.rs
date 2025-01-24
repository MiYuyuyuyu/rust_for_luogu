fn p1428() {
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let n:i32=s_temp.trim().parse().expect("err");
    s_temp.clear();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let c:Vec<&str>=s_temp.trim().split_whitespace().collect();
    let a:Vec<i32>=c.iter().map(|i| i.parse().expect("err")).collect();
    for i in 0..n as usize {
        let mut cnt = 0;
        for j in 0..i {
            if a[i]>a[j]{
                cnt+=1;
            }
        }
        print!("{} ",cnt);
    }
}