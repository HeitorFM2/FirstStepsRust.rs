#![allow(unused_variables, dead_code, unused_mut)]

fn main(){
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        if arg == "1"{
            ex1();
        } else if arg == "2" {
            ex2();
        } else if arg == "3" {
            ex3();
        } else if arg == "4" {
            ex4();
        }
    }
}

fn ex1(){
    let width: i32 = 4;
    let height: i32 = 7;
    let depth: i32 = 10;

    {
        let area: i32 = firststeps::area_of(width, height);

        println!("Area is {}", area);
    }

    println!("Volume is {}", firststeps::volume_of(width, height, depth));
}


fn ex2() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        if arg == "sum"{
            firststeps::sum();
        } else if arg == "double" {
            firststeps::double();
        } else {
            firststeps::count();
        }
    }
}

fn ex3() {
    let mut arg = std::env::args()
    .collect::<Vec<String>>()
    .iter()
    .nth(1)
    .unwrap_or_else(|| {
        println!("Please supply an argument to this program");
        std::process::exit(1);
    }).to_owned();

    firststeps::inspect(&arg);



    firststeps::change(&mut arg);
    println!("I have many {}", arg);

    if firststeps::eat(arg){
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }

}

trait Bite {
    fn bite(self: &mut Self);
}

#[derive(Debug)]
struct Grapes {
    amount_left: i32,
}

#[derive(Debug)]
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        self.percent_left -= 0.01;
    }
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.amount_left -= 1;
    }
}

fn ex4() {
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);
}
