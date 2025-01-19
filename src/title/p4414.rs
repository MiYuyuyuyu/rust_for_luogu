use std::collections::HashMap;

fn p4414() {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err in s_temp");
    let abc:Vec<&str>=s_temp.trim().split_whitespace().collect();
    let mut num_abc:Vec<i32>=Vec::new();
    for i in abc{
        num_abc.push(i.parse().expect("err"));
    }
    num_abc.sort();
    s_temp.clear();
    std::io::stdin().read_line(&mut s_temp).expect("err in s_temp");
    let abc:Vec<char>=s_temp.trim().chars().collect();
    let mut hash_abc:HashMap<char,i32>=HashMap::new();
    hash_abc.insert('A', num_abc[0]);
    hash_abc.insert('B', num_abc[1]);
    hash_abc.insert('C', num_abc[2]);
    for i in &abc{
        print!("{} ",hash_abc[i]);
    }
}