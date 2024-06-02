use std::{
    io::{self, Write},
    thread,
    time::{self, Duration},
};

const SLEEP_FOR: Duration = time::Duration::from_millis(100);
const GREEN_SQUARE: &str = "\x1b[48;2;0;255;0m  \x1b[0m";
// minY, minX
// maxY, maxX
const DIMENSIONS: [(usize, usize); 2] = [(0, 0), (400, 400)];

fn compute_next(curr_alive: &mut Vec<(usize, usize)>) -> &mut Vec<(usize, usize)> {
    print!("{:?}", curr_alive);
    return curr_alive;
}

fn main() {
    let mut curr_alive: Vec<(usize, usize)> =
        vec![(21, 21), (22, 21), (20, 21), (21, 20), (20, 22)];

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
        compute_next(&mut curr_alive);
        io::stdout().flush().unwrap();
    }
}

fn in_bounds(y: usize, x: usize) -> bool {
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
        let some_squares: Vec<(usize, usize)> =
            vec![(21, 21), (22, 21), (20, 21), (21, 20), (20, 22)];
        for pos in some_squares.iter() {
            assert!(in_bounds(pos.1, pos.0))
        }
    }

    #[test]
    fn test_coords_not_in_bounds() {
        let some_squares: Vec<(usize, usize)> = vec![(0, 400)];
        for pos in some_squares.iter() {
            assert!(!in_bounds(pos.1, pos.0))
        }
    }
}
