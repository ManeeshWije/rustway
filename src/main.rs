use std::{
    io::{self, Write},
    thread,
    time::{self, Duration},
};

const SLEEP_FOR: Duration = time::Duration::from_millis(100);
const GREEN_SQUARE: &str = "\x1b[48;2;0;255;0m  \x1b[0m";
// minY, minX
// maxY, maxX
const DIMENSIONS: [(i32, i32); 2] = [(0, 0), (400, 400)];

fn get_nbors(y: i32, x: i32) -> Vec<(i32, i32)> {
    let mut neighbors = Vec::with_capacity(8); // There are exactly 8 possible neighbors

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for &(dy, dx) in &directions {
        let ny = y.wrapping_add(dy);
        let nx = x.wrapping_add(dx);

        neighbors.push((ny, nx));
    }

    neighbors
}

// Any live cell with fewer than two live neighbours dies, as if by underpopulation.
// Any live cell with two or three live neighbours lives on to the next generation.
// Any live cell with more than three live neighbours dies, as if by overpopulation.
// Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
fn compute_next(curr_alive: &mut Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut next = Vec::new();

    let alive_set: std::collections::HashSet<_> = curr_alive.iter().copied().collect();

    let mut neighbor_count: std::collections::HashMap<(i32, i32), i32> =
        std::collections::HashMap::new();

    for &(y, x) in curr_alive.iter() {
        let neighbors = get_nbors(y, x);

        for neighbor in neighbors {
            if alive_set.contains(&neighbor) {
                *neighbor_count.entry(neighbor).or_insert(0) += 1;
            }
        }
    }

    for (&cell, &count) in &neighbor_count {
        if count == 2 || count == 3 {
            // Cell survives to the next generation
            next.push(cell);
        }
        // Any dead cell with exactly three live neighbors becomes alive
        for _ in get_nbors(cell.1, cell.0) {
            if !alive_set.contains(&cell) && neighbor_count.get(&cell) == Some(&3) {
                next.push(cell);
            }
        }
    }

    next
}

fn main() {
    let mut curr_alive: Vec<(i32, i32)> = vec![(21, 21), (22, 21), (20, 21), (21, 20), (20, 22)];

    println!("{esc}c", esc = 27 as char);
    loop {
        thread::sleep(SLEEP_FOR);
        println!("{esc}c", esc = 27 as char);
        for pos in curr_alive.iter() {
            let y = pos.0;
            let x = pos.1;
            if in_bounds(y, x) {
                print!(
                    "{esc}[{y};{x}H{green}",
                    esc = 27 as char,
                    y = y,
                    x = x,
                    green = GREEN_SQUARE
                );
            }
        }
        curr_alive = compute_next(&mut curr_alive);
        io::stdout().flush().unwrap();
    }
}

fn in_bounds(y: i32, x: i32) -> bool {
    if y >= DIMENSIONS[0].0 && y < DIMENSIONS[1].0 && x >= DIMENSIONS[0].1 && x < DIMENSIONS[1].1 {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coords_in_bounds() {
        let some_squares: Vec<(i32, i32)> = vec![(21, 21), (22, 21), (20, 21), (21, 20), (20, 22)];
        for pos in some_squares.iter() {
            assert!(in_bounds(pos.1, pos.0))
        }
    }

    #[test]
    fn test_coords_not_in_bounds() {
        let some_squares: Vec<(i32, i32)> = vec![(0, 400)];
        for pos in some_squares.iter() {
            assert!(!in_bounds(pos.1, pos.0))
        }
    }

    #[test]
    fn test_nbors() {
        let some_squares: Vec<(i32, i32)> = vec![(21, 21)];
        let mut expected_squares: Vec<(i32, i32)> = vec![
            (21, 20),
            (21, 22),
            (20, 20),
            (20, 21),
            (20, 22),
            (22, 22),
            (22, 21),
            (22, 20),
        ];
        let mut calculcated_nbors: Vec<(i32, i32)> = Vec::with_capacity(8);
        expected_squares.sort();

        for _ in some_squares.iter() {
            let nbors = get_nbors(21, 21);
            calculcated_nbors.extend(nbors)
        }
        calculcated_nbors.sort();
        assert_eq!(calculcated_nbors, expected_squares);
    }
}
