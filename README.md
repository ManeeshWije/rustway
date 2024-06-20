# Rustway

### Conway's Game of Life in the terminal

### Usage

This program parses a file that contains the starting coordinates of the alive cells.

The file format is as follows:

```
x,y
x,y
x,y
x,y
```

> NOTE: -x = left, +x = right, -y = up, +y = down

Once you have this file created, you can simply run `cargo run -- starting_coords`
