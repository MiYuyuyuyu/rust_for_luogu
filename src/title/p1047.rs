fn p1047() {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let s_temp:Vec<&str>=s_temp.trim().split_whitespace().collect();
    let num_tree:usize=s_temp[0].parse().expect("err");
    let mut tree:Vec<bool>=vec![true;num_tree+1];
    let n:i32=s_temp[1].parse().expect("err");
    for _ in 0..n{
        let mut s_temp=String::new();
        std::io::stdin().read_line(&mut s_temp).expect("err");
        let s_temp:Vec<&str>=s_temp.trim().split_whitespace().collect();
        let a:usize=s_temp[0].parse().expect("err");
        let b:usize=s_temp[1].parse().expect("err");
        for i in a..b+1{
            if tree[i]{
                tree[i]=false;
            }
        }
    }
    let mut ans=0;
    for i in tree {
        if i {
            ans+=1;
        }
    }
    println!("{}",ans);
}
