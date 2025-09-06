
use std::io::Write;

use srng::Rand;

fn main(){
     
    let mut r = Rand::new(74829);

    let path = "norm.txt";
    let mut output = std::fs::File::create(path).expect("just make the damn file");



    for i in 0..20_000_000 {
        let x = r.normal_standard();
        let line = format!("{}\n", x);
        output.write(line.as_bytes()).expect("could not write line");
        if i % 10_000 == 0 {
            println!("{i}", i=i);
        }
    }
}
