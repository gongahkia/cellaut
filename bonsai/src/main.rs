// TO IMPLEMENT
    // -- figure out the rules for bonsai tree growth
    // -- write an update_grid function
    // -- write a num_live_neighbours function
    // -- write a process_text_file function

// BONSAI TREE GROWTH RULES
    // -- use a RNG seed that is applied to every square's surrounding neighbour generation
    // -- 

use std::process::Command;
use std::io;
use std::io::Write;
use std::time::Duration;
use std::thread;

use colored::*;
use rand::*;

#[derive(Copy, Clone)]
#[derive(Debug)]
enum CellState {
    Outofbounds,
    Empty,
    Branch,
    Leaf,
}

#[derive(Copy, Clone)]
#[derive(Debug)]
struct Coordinate {
    x_coord:i32,
    y_coord:i32,
}

fn process_text_file(mut output_grid_display:Vec<Vec<CellState>>) -> Vec<Vec<CellState>> {
    // * add contents here later!
    output_grid_display
}

fn coordinate_to_cellstate(grid_display:Vec<Vec<CellState>>, c_struct:Coordinate) -> CellState {
    if c_struct.y_coord.clone() < 0 || c_struct.x_coord.clone() < 0 || c_struct.y_coord.clone() > 20 || c_struct.x_coord.clone() > 50 {
        CellState::Outofbounds
        // the overflow error is handled by the coordinate_to_cellstate() function here, not within required bounds, a check is not necessary as coordinate is not included within the count
    } else {
        grid_display[i32_to_usize(c_struct.y_coord.clone())][i32_to_usize(c_struct.x_coord.clone())]
    }
}

// resolve the return type of this function!
fn num_live_neighbours(grid_display:Vec<Vec<CellState>>, c_struct:Coordinate) -> u32 {

    // cell 5 is current coordinate cell!
    // naming convention for the neighbour coordinates            
    // [1][2][3]
    // [4][5][6]
    // [7][8][9]

    let x:i32 = c_struct.x_coord;
    let y:i32 = c_struct.y_coord;

    let c1:Coordinate = Coordinate {x_coord:x-1, y_coord:y-1};
    let c2:Coordinate = Coordinate {x_coord:x, y_coord:y-1};
    let c3:Coordinate = Coordinate {x_coord:x+1, y_coord:y-1};
    let c4:Coordinate = Coordinate {x_coord:x-1, y_coord:y};
    let c6:Coordinate = Coordinate {x_coord:x+1, y_coord:y};
    let c7:Coordinate = Coordinate {x_coord:x-1, y_coord:y+1};
    let c8:Coordinate = Coordinate {x_coord:x, y_coord:y+1};
    let c9:Coordinate = Coordinate {x_coord:x+1, y_coord:y+1};

    // max value of a u8 is 255, so the value range is 1 to 255
    let mut random_weight = rand::thread_rng();
    let random_number:u8 = random_weight.gen();

    // --- COUNTING NEIGHBOURS

    // -- *add contents here later!

    match coordinate_to_cellstate(grid_display.clone(), c1.clone()) {
        // add contents here later!
    }
    
    match coordinate_to_cellstate(grid_display.clone(), c2.clone()) {
        // add contents here later!
    }

    match coordinate_to_cellstate(grid_display.clone(), c3.clone()) {
        // add contents here later!
    }

    match coordinate_to_cellstate(grid_display.clone(), c4.clone()) {
        // add contents here later!
    }

    match coordinate_to_cellstate(grid_display.clone(), c6.clone()) {
        // add contents here later!
    }

    match coordinate_to_cellstate(grid_display.clone(), c7.clone()) {
        // add contents here later!
    }

    match coordinate_to_cellstate(grid_display.clone(), c8.clone()) {
        // add contents here later!
    }

    match coordinate_to_cellstate(grid_display.clone(), c9.clone()) {
        // add contents here later!
    }
}

fn update_grid(grid_display:Vec<Vec<CellState>>) -> Vec<Vec<CellState>> {

    let mut final_output_grid:Vec<Vec<CellState>> = Vec::new();
    for y in 0..21 {
        let mut x_output_grid:Vec<CellState> = Vec::new();
        for x in 0..51 {
            let c5:Coordinate = Coordinate {x_coord:x, y_coord:y};
            let electronhead_neighbour_count:u32 = num_live_neighbours(grid_display.clone(), c5.clone());
            match coordinate_to_cellstate(grid_display.clone(), c5.clone()) {
                CellState::Outofbounds => {
                },
                CellState::Empty => {
                    x_output_grid.push(CellState::Empty); 
                },
                CellState::ElectronHead => {
                    x_output_grid.push(CellState::ElectronTail); 
                },
                CellState::ElectronTail => {
                    x_output_grid.push(CellState::Conductor); 
                },
                CellState::Conductor => {
                    if electronhead_neighbour_count == 1 || electronhead_neighbour_count == 2 {
                        x_output_grid.push(CellState::ElectronHead);
                    } else {
                        x_output_grid.push(CellState::Conductor);
                    }
                },
            }
        }
        final_output_grid.push(x_output_grid.clone());
    }
    final_output_grid
}

// since Rust slices can only be indexed by usize
fn i32_to_usize(number_i32:i32) -> usize {
    let number_usize:usize = usize::try_from(number_i32).expect("Failed to parse large number");
    number_usize
}

// try_from() is a sick function
fn usize_to_i32(number_usize:usize) -> i32 {
    let number_i32:i32 = i32::try_from(number_usize).expect("Failed to parse large number");
    number_i32
}

fn display_grid(grid_display:Vec<Vec<CellState>>) {
    for y in 0..21 {
        for x in 0..51 {
            match grid_display[y][x] {
                CellState::Empty => {
                    print!(" ");
                    io::stdout().flush().expect("Failed to flush buffer");
                },
                CellState::Leaf => {
                    print!("{}", "X".green());
                    io::stdout().flush().expect("Failed to flush buffer");
                },
                CellState::Branch => {
                    print!("{}", "X".yellow());
                    io::stdout().flush().expect("Failed to flush buffer");
                },
                CellState::Outofbounds => (),
            }
        }
        print!("\n");
        io::stdout().flush().expect("Failed to flush buffer");
    }
}

fn main() {
    Command::new("clear").status().expect("Failed to run command");
    let mut grid_display:Vec<Vec<CellState>> = vec![vec![CellState::Empty; 51]; 21];
    grid_display = process_text_file(grid_display);

    loop {
        // println!("ass");
        Command::new("clear").status().expect("Failed to run command");
        println!("{}\n", "To grow a bonsai tree.".cyan());
        display_grid(grid_display.clone());
        grid_display = update_grid(grid_display.clone());
        thread::sleep(Duration::from_millis(300));
    }
}
