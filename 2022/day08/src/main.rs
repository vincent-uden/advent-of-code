use std::{
    fs,
    io::{self, BufRead},
};

// Right, Top, Left, Bottom
// tree_pos: ( row, column )
fn visibility(forest: &Vec<Vec<u32>>, tree_pos: (usize, usize)) -> (bool, bool, bool, bool) {
    let (row, column) = tree_pos;
    let tree_height = forest[row][column];

    let mut visbility = (true, true, true, true);

    // From tree -> Down to bottom
    for i in row + 1..forest.len() {
        if forest[i][column] >= tree_height {
            visbility.3 = false;
        }
    }

    // From tree -> Up to top
    for i in 0..row {
        if forest[i][column] >= tree_height {
            visbility.1 = false;
        }
    }

    // From tree -> To the right
    for i in column + 1..forest[row].len() {
        if forest[row][i] >= tree_height {
            visbility.0 = false;
        }
    }

    // From tree -> To the left
    for i in 0..column {
        if forest[row][i] >= tree_height {
            visbility.2 = false;
        }
    }

    return visbility;
}

fn any_visible(visibility: (bool, bool, bool, bool)) -> bool {
    visibility.0 || visibility.1 || visibility.2 || visibility.3
}

fn scenic_score(forest: &Vec<Vec<u32>>, tree_pos: (usize, usize)) -> u32 {
    let (row, column) = tree_pos;
    let tree_height = forest[row][column];
    let rows = forest.len();
    let columns = forest[0].len();

    let mut directions = (0, 0, 0, 0);

    for i in 1..columns-column {
        directions.0 += 1;
        if forest[row][column+i] >= tree_height {
            break;
        }
    }

    for i in 1..column+1 {
        directions.2 += 1;
        if forest[row][column-i] >= tree_height {
            break;
        }
    }

    for i in 1..rows-row {
        directions.3 += 1;
        if forest[row+i][column] >= tree_height {
            break;
        }
    }

    for i in 1..row+1 {
        directions.1 += 1;
        if forest[row-i][column] >= tree_height {
            break;
        }
    }

    println!("{:?} {:?}", tree_pos, directions);

    return directions.0 * directions.1 * directions.2 * directions.3;
}

fn part_1() -> u32 {
    let file = fs::File::open("./input.txt").unwrap();

    let mut trees: Vec<Vec<u32>> = vec![];

    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            let mut inner_line: Vec<u32> = vec![];

            for c in l.chars() {
                inner_line.push(c.to_digit(10).unwrap());
            }

            trees.push(inner_line);
        }
    }

    let rows = trees.len();
    let columns = trees[0].len();

    let mut visible_trees = 0;

    for i in 0..rows {
        for j in 0..columns {
            let vis = visibility(&trees, (i, j));
            println!("{}, {}, {:?}", i, j, vis);
            if any_visible(vis) {
                visible_trees += 1;
            }
        }
    }

    return visible_trees;
}

fn part_2() -> u32 {
    let file = fs::File::open("./input.txt").unwrap();

    let mut trees: Vec<Vec<u32>> = vec![];

    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            let mut inner_line: Vec<u32> = vec![];

            for c in l.chars() {
                inner_line.push(c.to_digit(10).unwrap());
            }

            trees.push(inner_line);
        }
    }

    let rows = trees.len();
    let columns = trees[0].len();

    let mut biggest_scenic = 0;

    for i in 0..rows {
        for j in 0..columns {
            let score = scenic_score(&trees, (i,j));
            if score > biggest_scenic {
                biggest_scenic = score;
            }
        }
    }

    return biggest_scenic;
}

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}
