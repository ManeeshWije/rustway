use std::{
    collections::HashMap,
    env,
    fs::read_to_string,
    io::{self, Error, ErrorKind, Write},
    thread,
    time::{self, Duration},
};

const SLEEP_FOR: Duration = time::Duration::from_millis(100);
const GREEN_SQUARE: &str = "\x1b[48;2;0;255;0m  \x1b[0m";
// minX, minY
// maxX, maxY
const DIMENSIONS: [(i32, i32); 2] = [(0, 0), (1000, 1000)];

fn parse_file(filename: &str) -> Result<Vec<(i32, i32)>, Error> {
    let mut data: Vec<String> = read_to_string(filename)?
        .lines()
        .map(String::from)
        .collect();

    data.retain(|s| s != "");

    let mut coords: Vec<(i32, i32)> = Vec::new();
    for coord in data.iter() {
        let parts: Vec<&str> = coord.split(",").collect();
        if parts.len() != 2 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid coordinate format",
            ));
        }

        println!("{:?}", parts);

        let x = parts[0]
            .trim()
            .parse::<i32>()
            .expect("ERROR: Could not parse x coordinate as int");
        let y = parts[1]
            .trim()
            .parse::<i32>()
            .expect("ERROR: Could not parse x coordinate as int");

        coords.push((x, y));
    }
    Ok(coords)
}

fn in_bounds(x: i32, y: i32) -> bool {
    if x >= DIMENSIONS[0].0 && x < DIMENSIONS[1].0 && y >= DIMENSIONS[0].1 && y < DIMENSIONS[1].1 {
        return true;
    }
    return false;
}

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
    let mut neighbor_count: HashMap<(i32, i32), i32> = HashMap::new();

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
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("ERROR: invalid command line arguments. Usage: cargo run -- <coords_file>");
    }

    let mut starting_coords: Vec<(i32, i32)> =
        parse_file(&args[1]).expect("ERROR: invalid file format, please see example file");

    loop {
        thread::sleep(SLEEP_FOR);
        println!("{esc}c", esc = 27 as char);
        for pos in starting_coords.iter() {
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
        starting_coords = compute_next(&mut starting_coords);
        io::stdout().flush().unwrap();
    }
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
