fn main() {
    let mut s_temp=String::new();
    std::io::stdin()
        .read_line(&mut s_temp)
        .expect("err");
    let str_1=s_temp.trim().to_lowercase();
    s_temp.clear();
    std::io::stdin()
        .read_line(&mut s_temp)
        .expect("err");
    s_temp=s_temp.trim().to_ascii_lowercase();
    let str_2:Vec<&str>=s_temp.split_whitespace().collect();
    let mut bo:bool=false;
    let mut cnt=0;
    let mut first_where=0;
    let mut j=0;
    str_2.iter().for_each(|&i|{
        if i == str_1{
            if !bo{
                bo=!bo;
                first_where=j;
            }
            cnt+=1;
        }
        j+=1;
    });
    if bo{
        println!("{} {}",cnt,first_where);
    }
    else{
        println!("-1");
    }
}