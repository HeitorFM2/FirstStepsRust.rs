pub fn area_of(x: i32, y: i32) -> i32 {
    x * y
}

pub fn volume_of(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}

pub fn sum(){
    let mut sum = 0;

    for i in 7..=23{
        sum += i;
    }

    println!("The sum is {}", sum);
}

pub fn double(){
    let mut count = 0;
    let mut x = 1;

    while x < 500 {
        count += 1;
        x *= 2;
    }

    println!("You can double x {} times before it exceeds 500", count);
}

pub fn count(){
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut count = 0;

    loop {
        for arg in &args {
            println!("{}: {}", count, arg);
            count += 1;
        }

        if count >= 8 {
            break;
        }
    }
}

pub fn eat(s : String) -> bool {
    if s.starts_with("b") && s.contains("a") {
        return true;
    } else {
        return false;
    }
}

pub fn inspect(s: &String){
    if s.ends_with("s") {
        println!("{} is plural", s);
    } else {
        println!("{} is singular", s);
    }
}

pub fn change(s: &mut String){
    if !s.ends_with("s") {
        s.push_str("s");
    }
}