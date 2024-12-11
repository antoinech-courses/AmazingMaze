// file: maze.rs

use std::cell::RefCell;
use std::rc::Rc;

// Enumeration to represent the exploration status
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Exploration {
    Explored,
    UnExplored,
}

// Enumeration to represent the maze
#[derive(Debug)]
enum Maze {
    Branch {
        label: String,
        left: Rc<Maze>,
        right: Rc<Maze>,
        status: RefCell<Exploration>, // Enables interior mutability for exploration status
    },
    Leaf {
        label: String,
    },
}

impl Maze {
    // Helper function to create a leaf
    fn leaf(label: &str) -> Rc<Maze> {
        Rc::new(Maze::Leaf {
            label: label.to_string(),
        })
    }

    // Helper function to create a branch
    fn branch(label: &str, left: Rc<Maze>, right: Rc<Maze>) -> Rc<Maze> {
        Rc::new(Maze::Branch {
            label: label.to_string(),
            left,
            right,
            status: RefCell::new(Exploration::UnExplored), // Initial exploration status
        })
    }

    // Method to explore the maze
    fn explore(&self, trace: &mut Vec<String>) {
        match self {
            Maze::Branch { label, left, right, status } => {
                // Check the current status of the branch
                let current_status = *status.borrow();
                match current_status {
                    Exploration::UnExplored => {
                        // Update the status and explore the child branches
                        status.replace(Exploration::Explored);
                        trace.push(label.clone());
                        left.explore(trace);
                        right.explore(trace);
                    }
                    Exploration::Explored => {
                        // If the branch has already been explored, just record the label
                        trace.push(label.clone());
                    }
                }
            }
            Maze::Leaf { label } => {
                // Leaves are always added to the trace
                trace.push(label.clone());
            }
        }
    }
}

// Function to construct the maze
fn maze() -> Rc<Maze> {
    // Create the leaves
    let leaf2 = Maze::leaf("2");
    let leaf4 = Maze::leaf("4");
    let leaf5 = Maze::leaf("5");
    let leaf8 = Maze::leaf("8");

    // Create the branches
    let branch3 = Maze::branch("3", Rc::clone(&leaf4), Rc::clone(&leaf5));
    let branch1 = Maze::branch("1", Rc::clone(&leaf2), Rc::clone(&branch3));
    let branch7 = Maze::branch("7", Rc::clone(&leaf5), Rc::clone(&leaf8));
    let branch6 = Maze::branch("6", Rc::clone(&branch3), Rc::clone(&branch7));
    let branch0 = Maze::branch("0", Rc::clone(&branch1), Rc::clone(&branch6));

    branch0
}

fn main() {
    // Build the maze
    let labyrinth = maze();

    // Explore the maze
    let mut trace: Vec<String> = Vec::new();
    labyrinth.explore(&mut trace);

    // Print the trace
    println!("{:?}", trace);
}
