use rusty_baseball;

fn main() {
    let bbs = rusty_baseball::BaseballState {
        x: 0.0,  // ft
        y: 55.0, // ft
        z: 5.0,  // ft
    };

    println!("{}", bbs);
}
