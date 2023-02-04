mod loopy;
mod towers;
mod latin;
mod perm;
use std::io::Write;

fn main() {
    let p = latin::puzzle::Puzzle::from_tatham_string("12:12c6b1c5c1c3a8b10_11_6a5_2_1d8_11a7_5_9a12a4c9a12_7a11_2b3c9_10b7b11d8d5b2_9_6_6b8b4_12a9_11b3_5_6a10a4_7b2b9_3d1_7_4_12_3_2_11b1a7a6e2_3e6_7");
    println!("Solving puzzle: ");
    let mut s = latin::solver::Solver::new(p);
    println!("{}", s.to_string());
    let sols = s.full_solve(0, true);
    if sols.len() == 1 {
        println!("{}", sols[0].to_string());
    } else {
        println!("{}", s.to_string());
    }

    let p_type = "latin";
    let size_range = 12..13;
    let quantity = 1;
    let min_difficulty = 1;

    if p_type.eq("latin") {
        println!("Generating {} puzzles", p_type);
        for n in size_range {
            let mut file = std::fs::File::create(format!("{}/{}_{}.txt", p_type, p_type, n)).expect("create failed");
            for i in 0..quantity {
                println!("{}", i);
                let mut p = latin::maker::make_puzzle(n);
                while p.difficulty < min_difficulty {
                    println!("Generated puzzle too easy...");
                    p = latin::maker::make_puzzle(n);
                }
                file.write_all((p.difficulty.to_string() + "\n").as_bytes()).expect("write failed");
                file.write_all((p.to_tatham_string() + "\n\n").as_bytes()).expect("write failed");
            }
            println!("Data written to {}/{}_{}.txt", p_type, p_type, n);
        }
    }

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

    // let p = loopy::puzzle::Puzzle::from_string(" \
    // 2.....20
    // .22.....
    // .1..11.3
    // .1...1..
    // 2....2..
    // .1.1...0
    // .0..231.
    // ..3...2.
    // ");
    //
    // let mut s = loopy::solver::Solver::new(p);
    // println!("START!");
    // println!("{}", s.to_string());
    // println!("");
    //
    // let sol = s.full_solve(0, true);
    // if sol.len() == 1 {
    //     println!("{}", sol[0].to_string());
    // } else {
    //     println!("{}", s.to_string());
    // }
}

// ········
// ········
// ·0······
// ······2·
// ···20··0
// ····20··
// ·····2·3
// ···2····

// ..33.3.1.22..3..23.3
// 3.2.1.3..23.3.22.2..
// 3...3.2.22.....21.32
// ...2....221..2...22.
// 13...132.32....0.21.
// 32...2.2....2..322.2
// .3.2322....122..1...
// .21112222..1..33..2.
// ....221.21.2.1....3.
// ..2.21.3.....12.213.
// 22.33..2.1...212...2
// ...2...2...2.3.22...
// 232...23..2213......
// ..21.20....3.322222.
// 231....312..2111.323
// ......1..312.33....2
// 3..22.1323.....122..
// ..3222.2..212.3..2.1
// 33.2....3....2....2.
// .........23...33.213

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
