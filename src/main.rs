mod loopy;
mod towers;
mod latin;
mod perm;
use std::io::Write;

fn main() {
    let p = loopy::puzzle::Puzzle::from_string(" \
        ...3.3.
        .3.....
        2...223
        2.2..1.
        2....2.
        2.11.21
        .2012.3
    ");
    println!("{}", p.to_string());

    // let mut file = std::fs::File::create("towers_9.txt").expect("create failed");
    // for i in 0..100 {
    //     println!("{}", i);
    //     let p = towers::maker::make_puzzle(9);
    //     file.write_all("Difficulty: ".as_bytes()).expect("write failed");
    //     file.write_all(p.difficulty.to_string().as_bytes()).expect("write failed");
    //     file.write_all("\n".as_bytes()).expect("write failed");
    //     file.write_all(p.to_string().as_bytes()).expect("write failed");
    //     file.write_all("\n-----\n".as_bytes()).expect("write failed");
    // }
    // println!("data written to file");

    let mut s = loopy::solver::Solver::new(p);
    println!("START!");
    println!("{}", s.to_string());
    println!("");

    s.full_solve(0, true);
    println!("{}", s.to_string())
}
