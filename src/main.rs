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
    3.3..
    2.0.2
    2.22.
    22.3.
    .....
    ");

    let mut s = loopy::solver::Solver::new(p);
    println!("START!");
    println!("{}", s.to_string());
    println!("");

    let sol = s.full_solve(0, true);
    println!("{}", s.to_string());
}

// ···3······
// ···1····23
// ·········3
// ·3··0·····
// ··········
// ····2·1··0
// ·····2·1··
// ·11··3·0··
// ······1··2
// ······0···

// ······
// ·····1
// ······
// ····0·
// ····1·
// ···0··


// 0······
// ·······
// ·····0·
// ·······
// ··2·0·1
// ····0·3
// ·······


// ·····2·2
// ·······3
// ········
// ·······0
// ·2······
// ···3···2
// 00···3··
// ·1·2··0·


// ·······0
// ········
// ········
// ········
// ·3·0·0··
// ····2··1
// ·3···2··
// 3····2··

// ··21·2·
// ·······
// ·······
// ····0··
// 0·····0
// ····3·1
// ···1·1·

// 3······
// ·······
// ···3···
// ·······
// 2····10
// ···3···
// ····01·

// ······
// ······
// ···0··
// ······
// ···0··
// 2·2···

// ···0···
// ··0··2·
// ·······
// ···3·01
// ·······
// ···33··
// ·222··3

// ··1····
// ·······
// ······1
// ·······
// 123···3
// 21·0·23
// 2······

// ······0
// 2······
// ······0
// ·······
// ·2··11·
// ···0·1·
// ···0···
