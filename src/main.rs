mod loopy;
mod towers;
mod latin;
mod perm;
use std::io::Write;

fn main() {
    // for n in 2..10 {
    //     let mut file = std::fs::File::create(format!("loopy/loopy_{}.txt", n)).expect("create failed");
    //     for i in 0..100 {
    //         println!("{}", i);
    //         let p = loopy::maker::make_puzzle(n);
    //         file.write_all("Difficulty: ".as_bytes()).expect("write failed");
    //         file.write_all(p.difficulty.to_string().as_bytes()).expect("write failed");
    //         file.write_all("\n".as_bytes()).expect("write failed");
    //         file.write_all(p.to_string().as_bytes()).expect("write failed");
    //         file.write_all("\n-----\n".as_bytes()).expect("write failed");
    //     }
    //     println!("Data written to loopy/loopy_{}.txt", n);
    // }

    let p = loopy::puzzle::Puzzle::from_string(" \
        .......
        1...1..
        .......
        .....0.
        ..3...1
        .......
        .....31
    ");

    let mut s = loopy::solver::Solver::new(p);
    println!("START!");
    println!("{}", s.to_string());
    println!("");

    let sol = s.full_solve(0, true);
    println!("{}", s.to_string());
}
