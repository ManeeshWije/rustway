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

fn get_nbors(x: i32, y: i32) -> Vec<(i32, i32)> {
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

    for &(dx, dy) in &directions {
        let nx = x.wrapping_add(dx);
        let ny = y.wrapping_add(dy);

        neighbors.push((nx, ny));
    }

    neighbors
}

// Any live cell with fewer than two live neighbours dies, as if by underpopulation.
// Any live cell with two or three live neighbours lives on to the next generation.
// Any live cell with more than three live neighbours dies, as if by overpopulation.
// Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
fn compute_next(curr_alive: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut next = Vec::new();
    let mut neighbor_count: std::collections::HashMap<(i32, i32), i32> = std::collections::HashMap::new();

    // Count neighbors for all live cells
    for &(x, y) in curr_alive.iter() {
        let neighbors = get_nbors(x, y);
        for neighbor in neighbors {
            *neighbor_count.entry(neighbor).or_insert(0) += 1;
        }
    }

    // Determine next generation based on rules
    for (cell, &count) in &neighbor_count {
        if count == 3 || (count == 2 && curr_alive.contains(&cell)) {
            // Cell survives or becomes alive in next generation
            next.push(*cell);
        }
    }

    next
}

fn main() {
    let mut curr_alive: Vec<(i32, i32)> = vec![(21, 21), (22, 21), (23, 21), (23, 22), (22, 23)];

    loop {
        thread::sleep(SLEEP_FOR);
        println!("{esc}c", esc = 27 as char);
        for pos in curr_alive.iter() {
            let x = pos.0;
            let y = pos.1;
            if in_bounds(x, y) {
                print!(
                    "{esc}[{y};{x}H{green}",
                    esc = 27 as char,
                    x = x,
                    y = y,
                    green = GREEN_SQUARE
                );
            }
        }
        curr_alive = compute_next(&mut curr_alive);
        io::stdout().flush().unwrap();
    }
}

fn in_bounds(x: i32, y: i32) -> bool {
    if x >= DIMENSIONS[0].0 && x < DIMENSIONS[1].0 && y >= DIMENSIONS[0].1 && y < DIMENSIONS[1].1 {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coords_in_bounds() {
        let some_squares: Vec<(i32, i32)> = vec![(21, 21), (22, 21), (23, 21), (23, 22), (22, 23)];
        for pos in some_squares.iter() {
            assert!(in_bounds(pos.0, pos.1))
        }
    }

    #[test]
    fn test_coords_not_in_bounds() {
        let some_squares: Vec<(i32, i32)> = vec![(0, 400)];
        for pos in some_squares.iter() {
            assert!(!in_bounds(pos.0, pos.1))
        }
    }

    #[test]
    fn test_nbors() {
        let some_squares: Vec<(i32, i32)> = vec![(21, 21)];
        let mut expected_squares: Vec<(i32, i32)> = vec![
            (21, 20), // up
            (21, 22), // down
            (20, 21), // left
            (22, 21), // right
            (20, 20), // top-left
            (22, 20), // top-right
            (20, 22), // bottom-left
            (22, 22), // bottom-right
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
