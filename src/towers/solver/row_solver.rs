use std::collections::HashSet;
use std::cmp;

// Solver methods based on the fact that each row/column must respect its view hint.

// TODO: This might not be necessary now that we have a better view solver.
// pub fn max_analysis(view: u8, row: &Vec<&HashSet<u8>>) -> (bool, Vec<(usize, u8)>) {
//     let n = row.len();
//     let mut to_remove: Vec<(usize, u8)> = Vec::new();
//
//     let mut min_so_far = 0;
//     let mut max_so_far = 0;
//     let mut potential_seen_so_far = 0;
//     let mut values_taken_for_sure: HashSet<u8> = HashSet::new();
//     for i in 0..n {
//         let min = *row[i].iter().min().unwrap();
//         let max = *row[i].iter().max().unwrap();
//         if min == max {
//             values_taken_for_sure.insert(min.clone());
//         }
//         if i == 0 || max > min_so_far {
//             potential_seen_so_far += 1;
//         } else {
//             // This cell can't possibly be seen, so this cell can't possibly hide another cell.
//             min_so_far = cmp::max(min_so_far, min);
//             max_so_far = cmp::max(max_so_far, max);
//             continue;
//         }
//
//         // To be seen after the max is chosen, the cell has to be a value greater than max
//         // and can't be one of the values taken for sure.
//         let mut potential_seen_after_if_max_chosen = 0;
//         let mut temp_min_so_far = max;
//         let mut cap_after_chosen = ((max+1)..(n as u8)).filter(|x| !values_taken_for_sure.contains(x)).count();
//
//         for j in (i+1)..n {
//             let other_min = *row[j].iter().min().unwrap();
//             let other_max = *row[j].iter().max().unwrap();
//             if other_max > temp_min_so_far {
//                 potential_seen_after_if_max_chosen += 1;
//                 potential_seen_after_if_max_chosen = cmp::min(potential_seen_after_if_max_chosen, cap_after_chosen);
//             }
//             if other_min > temp_min_so_far {
//                 temp_min_so_far = other_min;
//                 let cap_after_this = ((temp_min_so_far+1)..(n as u8)).filter(|x| !values_taken_for_sure.contains(x)).count();
//                 cap_after_chosen = potential_seen_after_if_max_chosen + cap_after_this;
//             }
//         }
//         if potential_seen_so_far + (potential_seen_after_if_max_chosen as u8) < view {
//           // Then this cell can't be its max value since the view would be impossible to attain.
//           if row[i].len() == 1 {
//               return (false, Vec::new());
//           }
//           to_remove.push((i, max));
//         }
//
//         min_so_far = cmp::max(min_so_far, min);
//         max_so_far = cmp::max(max_so_far, max);
//     }
//
//     return (true, to_remove);
// }

fn view(row: &Vec<u8>) -> u8 {
    if row.len() == 0 {
        return 0;
    }
    let mut max_so_far = row[0];
    let mut seen_so_far = 1;
    for i in 1..row.len() {
        if row[i] > max_so_far {
            max_so_far = row[i];
            seen_so_far += 1;
        }
    }
    return seen_so_far;
}

fn has_duplicate(row: &Vec<u8>) -> bool {
    for i in 0..row.len() {
        for j in (i+1)..row.len() {
            if row[i] == row[j] {
                return true;
            }
        }
    }
    return false;
}

fn all_possibilities(fields: &Vec<Vec<&u8>>, expected_view: u8) -> Vec<Vec<u8>> {
    let mut counter: Vec<usize> = fields.iter().map(|_x| 0).collect();
    let mut result: Vec<Vec<u8>> = Vec::new();
    loop {
        let possibility: Vec<u8> = fields.iter().enumerate().map(|(index, x)| *x[counter[index]]).collect();
        if !has_duplicate(&possibility) && view(&possibility) == expected_view {
            result.push(possibility);
        }
        counter[0] += 1;
        let mut index = 0;
        while counter[index] >=  fields[index].len() {
            if index == fields.len() - 1 {
                return result;
            }
            counter[index] = 0;
            counter[index + 1] += 1;
            index += 1;
        }
    }
}

fn vec_from_set(set: &HashSet<u8>) -> Vec<&u8> {
    return set.iter().collect();
}

// Tries all possibilities and removes the options that are impossible.
pub fn trial_solve(view: u8, row: &Vec<&HashSet<u8>>) -> (bool, Vec<(usize, u8)>) {
    let mut to_remove: Vec<(usize, u8)> = Vec::new();

    let mut fields = Vec::new();
    for x in row {
        fields.push(vec_from_set(x));
    }
    let possibilities = all_possibilities(&fields, view);

    for i in 0..row.len() {
        if row[i].len() == 1 {
            continue;
        }
        for value in row[i] {
            let mut found_match = false;
            // Iterate through all possibilities
            for p in &possibilities {
                if p[i] == *value {
                    found_match = true;
                    break;
                }
            }

            if !found_match {
                to_remove.push((i, *value));
            }
        }
    }
    return (true, to_remove);
}

pub fn solve(view: u8, row: &Vec<&HashSet<u8>>) -> (bool, Vec<(usize, u8)>) {
    let n = row.len();
    let mut positions_seen_for_sure: Vec<usize> = Vec::new();
    let mut positions_not_seen_for_sure: Vec<usize> = Vec::new();
    let mut values_seen_for_sure: Vec<u8> = Vec::new();
    let mut values_not_seen_for_sure: Vec<u8> = Vec::new();
    let mut to_remove: Vec<(usize, u8)> = Vec::new();

    let mut largest_so_far = 0;
    let mut smallest_so_far = 0;
    for i in 0..n {
        let min = row[i].iter().min().unwrap();
        let max = row[i].iter().max().unwrap();
        if *min >= largest_so_far {
            positions_seen_for_sure.push(i);
        }
        if i != 0 && *max <= smallest_so_far {
            positions_not_seen_for_sure.push(i);
        }
        largest_so_far = cmp::max(largest_so_far, *max);
        smallest_so_far = cmp::max(smallest_so_far, *min);
    }

    let mut earliest_pos_so_far = n;
    let mut latest_pos_so_far = n;
    for v in (0..n).rev() {
        let value = v as u8;
        let earliest = row.iter().position(|x| x.contains(&value)).unwrap();
        let latest = n - 1 - row.iter().rev().position(|x| x.contains(&value)).unwrap();
        if latest <= earliest_pos_so_far {
          values_seen_for_sure.push(value);
        }
        if earliest >= latest_pos_so_far {
          values_not_seen_for_sure.push(value);
        }
        earliest_pos_so_far = cmp::min(earliest_pos_so_far, earliest);
        latest_pos_so_far = cmp::min(latest_pos_so_far, latest);
    }

    if values_seen_for_sure.len() + values_not_seen_for_sure.len() == n {
        // No matter what we do, whether a value is seen or not is already determined.
        return (values_seen_for_sure.len() as u8 == view, Vec::new());
    }
    if positions_seen_for_sure.len() + positions_not_seen_for_sure.len() == n {
        // No matter what we do, whether a position is seen or not is already determined.
        return (positions_seen_for_sure.len() as u8 == view, Vec::new());
    }

    if ((n - positions_not_seen_for_sure.len()) as u8) == view {
        // Then all other positions should be seen!
        let mut min_so_far: u8 = 0;
        for i in 0..n {
            let min = row[i].iter().min().unwrap();
            if !positions_seen_for_sure.iter().chain(positions_not_seen_for_sure.iter()).any(|x| *x == i) {
                // Should be seen!
                // Has to be a value bigger than the largest min value so far
                for value in 0..(min_so_far + 1) {
                    to_remove.push((i, value));
                }
                // None of the previous cells can be greater than (or equal to) the max
                // value of this cell.
                let max = row[i].iter().max().unwrap();
                for j in 0..i {
                    for value in *max..(n as u8) {
                        to_remove.push((j, value));
                    }
                }
            }
            min_so_far = cmp::max(min_so_far, *min);
        }
    }

    if ((n - values_not_seen_for_sure.len()) as u8) == view {
        // Then all other values should be seen!
        let mut earliest_pos_so_far = n;
        let mut latest_pos_so_far = n;
        for value in (0..(n as u8)).rev() {
            let earliest = row.iter().position(|x| x.contains(&value)).unwrap();
            let latest = n - 1 - row.iter().rev().position(|x| x.contains(&value)).unwrap();
            if !values_seen_for_sure.iter().chain(values_not_seen_for_sure.iter()).any(|x| *x == value) {
                // Should be seen!
                // Has to be in front of any larger value
                for pos in latest_pos_so_far..n {
                    to_remove.push((pos, value));
                }
                // None of the larger values can be in front.
                for larger in (value + 1)..(n as u8) {
                    for pos in 0..(earliest + 1) {
                        to_remove.push((pos, larger));
                    }
                }
            }
            earliest_pos_so_far = cmp::min(earliest_pos_so_far, earliest);
            latest_pos_so_far = cmp::min(latest_pos_so_far, latest);
        }
    }

    // Let #v be the number possibly seen values.
    // If the value at the first possibly seen position is one of the possibly seen values, then
    // it can't be the (#v - view - 1)th possibly seen value or larger, since that would leave only
    // view - 2 values that can possibly be seen.
    // In general the value at the k-th possibly seen position, if it's one of the possibly seen
    // values, can't be the (k + #v - view - 1)th possibly seen value or larger.
    let possibly_seen_values: Vec<u8> = (0..(n as u8)).filter(|x| !values_not_seen_for_sure.iter().any(|y| x==y)).collect();
    let possibly_seen_positions = (0..n).filter(|x| !positions_not_seen_for_sure.iter().any(|y| x==y));
    let v = possibly_seen_values.len();
    for (k, pos) in possibly_seen_positions.enumerate() {
        for value in possibly_seen_values.iter().skip(1 + k + v - (view as usize)) {
            to_remove.push((pos, *value));
        }
    }



    // Values seen for sure that cannot possibly be in a position seen for sure.
    let extra_values:HashSet<u8> = values_seen_for_sure.iter().filter(|x| {
        for i in &positions_seen_for_sure {
            if row[*i].iter().any(|y| y == *x) {
                return false;
            }
        }
        return true;
    }).map(|x| *x).collect();

    // TODO: I think there is a dual to this where we start looking at the values insead of
    // the positions. Need to investigate further.
    if (positions_seen_for_sure.len() + extra_values.len()) as u8 == view {
        // The only other positions that can be seen are the ones that have one of the
        // values that is seen for sure (and in that case, it must have one of those values).
        let mut max_so_far: u8 = 0;
        let mut min_so_far: u8 = 0;
        for i in 0..n {
            let max = row[i].iter().max().unwrap();
            let min = row[i].iter().min().unwrap();
            if !positions_seen_for_sure.iter().chain(positions_not_seen_for_sure.iter()).any(|x| *x == i) {
                // Should not be seen or should have one of the values seen for sure.
                let can_be_seen_value = !row[i].is_disjoint(&extra_values);

                // Except if it's one of the extra values, this cell cannot be seen!
                // Except if it's one of the extra values, this cell cannot be a value bigger than
                // (or equal to) the largest value so far.
                for value in (max_so_far..(n as u8)).filter(|x| !extra_values.contains(x)) {
                    to_remove.push((i, value));
                }

                if !can_be_seen_value {
                    // Cannot be seen!
                    // At least one of the previous cells has to be greater than the min value of
                    // this cell.
                    // TODO: At least one of the SEEN cells has to block this one.
                    let mut candidate_found = n;
                    for j in 0..i {
                        if row[j].iter().any(|x| x > min) {
                            if candidate_found != n {
                                // More than one candidate found so we can't force anything.
                                candidate_found = n;
                                break;
                            } else {
                                candidate_found = j;
                            }
                        }
                    }
                    if candidate_found != n {
                       // We only found one cell that could have values big enough to hide cell i.
                       // So that cell must be take on one of those big enough values.
                       for value in 0..(*min + 1) {
                           to_remove.push((candidate_found, value));
                       }
                    }
                }
            }
            max_so_far = cmp::max(max_so_far, *max);
            min_so_far = cmp::max(min_so_far, *min);
        }
    }
    return (true, to_remove);
}
