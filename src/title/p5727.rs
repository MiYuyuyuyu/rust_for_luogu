fn p5727() {
    let mut s_temp:String=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let mut n:i32=s_temp.trim().parse().expect("err");
    let mut vec_num:Vec<i32>=Vec::new();
    loop{
        if n==1 {
            vec_num.push(n);
            break;
        }
        else if n%2==0 {
            vec_num.push(n);
            n/=2;
        }
        else{
            vec_num.push(n);
            n=n*3+1;
        }
    }
    for i in (0..vec_num.len()).rev(){
        print!("{} ",vec_num[i]);
    }
}