mod towers;
mod latin;
mod perm;

fn main() {
    // let square = latin::square::Square::from_string(" \
    //    1 2 3
    //    2 3 1
    //    3 1 2");
    //
    // println!("{}", square.to_string());


    println!("main start");
    let iter = latin::square::SquareIter::new(6);
    for test in iter {
       println!("{}\n", test.to_string());
    }


    // 4*4 unreasonable
    // let p = towers::puzzle::Puzzle::from_string(" \
    //     ? ? ? ?
    //   3 ? ? ? ? ?
    //   ? ? ? ? ? 2
    //   3 ? ? ? ? ?
    //   ? ? ? ? ? ?
    //     1 ? 2 ?   ");

    // 4*4 unreasonable
    // let p = towers::puzzle::Puzzle::from_string(" \
    //     ? ? 3 ?
    //   ? ? ? ? ? ?
    //   2 ? ? ? ? ?
    //   2 ? ? ? ? ?
    //   ? ? ? ? ? 3
    //     ? ? ? 3   ");

    // 5*5 hard
    // let mut p = towers::puzzle::from_string(" \
    //     3 ? ? 4 ?
    //   ? ? ? ? ? ? ?
    //   ? ? ? ? ? ? ?
    //   ? ? ? ? ? ? ?
    //   2 ? 3 ? ? ? ?
    //   3 ? ? ? ? ? ?
    //     ? 4 ? ? 1  ");

    // 5*5 unreasonable
    // let p = towers::puzzle::Puzzle::from_string(" \
    //     3 ? 4 ? ?
    //   ? ? ? ? ? ? 1
    //   3 ? ? ? ? ? ?
    //   ? ? ? ? ? ? ?
    //   ? 1 ? ? ? ? ?
    //   ? ? ? ? ? ? ?
    //     ? ? ? 2 ?  ");

    // 5*5 unreasonable - Depth 4 required to solve!
    // let p = towers::puzzle::Puzzle::from_string(" \
    //     ? ? 3 ? ?
    //   ? ? ? ? ? ? ?
    //   ? ? ? ? ? ? 2
    //   ? ? ? ? ? ? 2
    //   ? ? ? ? ? ? 2
    //   3 ? ? 1 ? ? ?
    //     ? 4 ? ? ?  ");

    // 6*6 easy
    // let mut p = towers::puzzle::from_string(" \
    //     4 2 2 3 5 1
    //   4 ? ? ? ? ? ? 1
    //   2 ? ? 3 ? ? ? 3
    //   3 ? ? ? ? ? ? 3
    //   2 ? ? ? ? ? ? 3
    //   1 ? ? 1 ? ? ? 3
    //   2 ? ? ? ? ? ? 2
    //     2 4 3 2 1 2  ");

    // 6*6 hard - Still needs to brute force views!
    // let p = towers::puzzle::Puzzle::from_string(" \
    //     ? ? ? ? 2 ?
    //   5 ? ? ? ? ? ? ?
    //   4 3 ? ? ? ? ? 3
    //   ? ? ? 4 ? ? ? 2
    //   ? ? ? ? ? ? 3 ?
    //   ? ? ? ? ? ? ? 2
    //   ? ? ? ? ? ? ? ?
    //     ? 3 ? 3 3 ?  ");

    // 6*6 hard
    // let mut p = towers::puzzle::from_string(" \
    //     ? ? 4 3 ? ?
    //   ? ? 2 ? ? ? ? ?
    //   ? ? ? ? ? 2 ? 2
    //   ? ? ? ? ? ? ? ?
    //   4 ? ? ? ? ? ? ?
    //   2 ? ? ? 4 ? ? 1
    //   ? ? ? ? ? ? ? ?
    //     ? ? ? 3 2 ?  ");

    // 6*6 extreme
    // let mut p = towers::puzzle::from_string(" \
    //     ? 2 ? 3 ? ?
    //   ? ? ? ? ? ? ? ?
    //   5 ? ? ? ? ? ? 2
    //   3 ? ? ? ? ? ? 3
    //   ? ? ? ? ? ? ? ?
    //   ? ? ? ? ? ? ? 4
    //   ? ? ? ? ? ? ? ?
    //     ? 3 1 2 2 3  ");

    // 6*6 extreme
    // let p = towers::puzzle::Puzzle::from_string(" \
    //     ? ? 4 3 3 ?
    //   2 ? ? ? ? ? ? ?
    //   ? ? ? ? ? ? ? ?
    //   3 ? ? ? ? 4 ? ?
    //   ? ? ? ? ? ? ? 1
    //   3 ? ? ? ? ? ? 3
    //   ? ? ? ? ? ? ? ?
    //     4 4 ? ? ? 3  ");

    // 6*6 extreme
    // let p = towers::puzzle::Puzzle::from_string(" \
    //     ? ? 3 ? 3 ?
    //   ? ? ? ? ? ? ? ?
    //   1 ? ? ? ? ? 2 4
    //   ? ? ? ? ? ? ? ?
    //   2 ? ? ? ? ? ? 3
    //   4 ? ? ? ? 1 ? 2
    //   ? ? ? 2 ? ? ? ?
    //     4 3 ? ? 2 ?  ");

    // 6*6 unreasonable
    // let p = towers::puzzle::Puzzle::from_string(" \
    //     3 ? 3 ? 3 ?
    //   ? ? ? ? ? ? ? 3
    //   ? ? ? ? ? ? ? 3
    //   ? ? ? ? ? ? ? ?
    //   ? ? ? ? ? 2 ? 5
    //   4 ? ? 1 ? ? ? 1
    //   ? ? ? ? ? ? ? ?
    //     ? 2 ? 3 ? ?  ");

    // 7*7 unreasonable.
    // let p = towers::puzzle::Puzzle::from_string(" \
    //     ? ? 3 4 ? ? 2
    //   ? ? ? ? ? ? ? ? 3
    //   3 ? ? ? ? 5 ? ? ?
    //   3 ? ? ? 1 ? ? ? 2
    //   4 ? ? ? ? ? ? ? ?
    //   ? ? ? ? 3 ? ? 4 ?
    //   3 ? ? ? ? ? ? ? ?
    //   2 ? ? 2 ? ? ? ? 2
    //     ? ? ? ? 1 3 ?  ");

    // 8*8 unreasonable.
    // let p = towers::puzzle::Puzzle::from_string(" \
    //     4 ? ? 5 2 4 3 ?
    //   ? 1 ? ? 2 ? 3 ? ? ?
    //   3 ? ? ? ? ? ? ? ? ?
    //   3 ? ? 4 ? ? ? ? ? 2
    //   2 ? ? ? ? ? ? ? ? 3
    //   3 ? 1 ? ? ? ? ? ? 2
    //   ? ? 2 ? ? 1 ? ? ? 4
    //   2 ? ? ? ? ? ? ? ? 3
    //   4 ? ? ? ? ? ? ? ? 3
    //     ? 4 2 ? 1 4 ? 4 ");

    // 9*9 unreasonable - Can solve! But I think there are multiple answers
    // There are multiple answers! I might have not correctly copied this puzzle...
    // let p = towers::puzzle::Puzzle::from_string(" \
    //     ? ? ? 4 2 ? 3 ? 4
    //   4 4 2 ? ? ? ? ? ? ? ?
    //   4 3 ? ? ? ? ? ? 5 ? ?
    //   ? ? 5 4 ? ? ? ? ? ? 4
    //   ? ? ? ? ? ? ? ? ? ? 6
    //   ? ? ? ? 2 ? 9 6 ? ? 4
    //   3 ? ? ? ? ? ? ? ? ? ?
    //   2 ? ? ? ? 3 ? ? 7 ? ?
    //   ? ? ? ? ? ? ? 7 ? ? 4
    //   ? ? ? ? ? ? 6 ? ? ? ?
    //     3 5 ? ? 3 3 3 ? 2  ");

    let p = towers::maker::make_puzzle(3);

    let mut s = towers::solver::Solver::new(p);
    println!("START!");
    println!("{}", s.to_detailed_string());
    println!("");

    let solutions = s.full_solve(0);
    towers::maker::make_puzzle(3);
    println!("{}", s.to_detailed_string());
    println!("Sample Solutions: ");
    for sol in solutions {
        println!("{}", sol.to_detailed_string());
    }
}
