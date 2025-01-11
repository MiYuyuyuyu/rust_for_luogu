fn q1() {
    println!("I love Luogu!");
}

fn q2(){
    //10 2 4
    let apple_num: i32=10;
    let a: i32=2;
    let nim: i32=4;
    let b: i32=apple_num-a-nim;
    println!("{} {}",a+nim,b);
}

fn q3() {
    //14 4
    let apple_num: i32=14;
    let people_class: i32=4;
    let apple_for_people=14/4;
    let apple_for_eat=apple_for_people*people_class;
    let apple_for_no_eat=apple_num-apple_for_eat;
    println!("{}\n{}\n{}",apple_for_people,apple_for_eat,apple_for_no_eat);
}

fn q4() {
    let cola:f64=500.0;
    // let people: i32=3;
    println!("{:.3}",cola/3.0);
}

fn q5() {
    let a:i32=260;
    let a_v:i32=12;
    let b:i32=220;
    let b_v:i32=20;
    let t: i32=(a+b)/(a_v+b_v);
    println!("{}",t);
}

fn q6() {
    let l:i32=6;
    let k:i32=9;
    let ans=( (l.pow(2)+k.pow(2)) as f64 ).sqrt();
    println!("{:.4}",ans);
}

fn q7(){
    let mut uim=110;
    println!("{}",uim);
    uim-=20;
    println!("{}",uim);
    println!("{}",0);
}

fn q8() {
    let pi: f64=3.141593;
    let r: i32=5;
    let c=2.0*pi*(r as f64);
    let s=pi*( r.pow(2) as f64);
    let v=(4.0/3.0) * pi *(r.pow(3) as f64);
    println!("{:.4}\n{:.4}\n{:.3}",c,s,v);
}

fn q9(){
    println!("22");
}

fn q10() {
    //8-30,10->6
    println!("9");
}


fn q11() {
    let a_v=5;
    let b_v=8;
    let ans=(100.0)/((b_v-a_v)as f64);
    println!("{:.4}",ans);
}

fn q12() {
    println!("{}\n{}",('M' as u8 -b'A')+1,(b'A'+18-1) as char);
}

fn q13(){
    println!("16");
}

fn q14(){
    //110 10
    // | 1 1 3500
    println!("50");
}

fn p2433() {
    // 这题写的我头大😶‍🌫️😇🤡♿♿♿♿♿
    let mut s_temp:String=String::new();
    std::io::stdin()
        .read_line(& mut s_temp)
        .expect("in s_temp err");
    let num_for_q:i32=match s_temp.trim().parse() {
        Ok(num)=>num,
        Err(_)=>{
            println!("哦 这还有报错的");
            0
        }
    };
    match num_for_q {
        0=>println!("哦 还真有0"),
        1=>q1(),
        2=>q2(),
        3=>q3(),
        4=>q4(),
        5=>q5(),
        6=>q6(),
        7=>q7(),
        8=>q8(),
        9=>q9(),
        10=>q10(),
        11=>q11(),
        12=>q12(),
        13=>q13(),
        14=>q14(),
        _=>{
            println!("你想想你打了什么?");
        }
    };
}