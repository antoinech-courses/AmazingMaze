// file: maze.rs

use std::cell::RefCell;
use std::rc::Rc;

// Enumeration to represent the exploration status
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Exploration {
    Explored,           // Fully explored
    PartiallyExplored,  // Left child explored, right child to be explored
    UnExplored,         // Not yet explored
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

    // Concurrent version of the exploration function
    fn explore(&self, node: Rc<Maze>, work: &mut Vec<Rc<Maze>>, trace: &mut Vec<String>) {
        match self {
            Maze::Branch { label, left, right, status } => {
                let current_status = *status.borrow();
                match current_status {
                    Exploration::UnExplored => {
                        // Push the current branch onto the work stack
                        status.replace(Exploration::PartiallyExplored); // Update the status
                        trace.push(label.clone());
                        work.push(node.clone()); // Push the current node to the work stack
                        left.explore(Rc::clone(left), work, trace); // Explore the left child
                    }
                    Exploration::PartiallyExplored => {
                        // The left child has already been explored, now explore the right
                        status.replace(Exploration::Explored); // Mark this branch as fully explored
                        trace.push(label.clone());
                        right.explore(Rc::clone(right), work, trace); // Explore the right child
                    }
                    Exploration::Explored => {
                        // If the branch is already fully explored, just add it to the trace
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
    let maze = maze();

    // Initialize the work stack with the root node
    let mut work: Vec<Rc<Maze>> = vec![Rc::clone(&maze)];
    let mut trace: Vec<String> = vec![];

    // Simulate concurrent exploration
    while !work.is_empty() {
        let node = work.pop().expect("work stack should not be empty");
        node.explore(Rc::clone(&node), &mut work, &mut trace);
        println!("Trace so far: {:?}", trace);
    }
}
