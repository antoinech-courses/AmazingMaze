// Import
use std::cell::RefCell;

// Enumeration to represent the exploration state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Exploration {
    Explored,
    UnExplored,
}

// Enumeration to represent the maze
#[derive(Debug)]
enum Maze<'a> {
    Branch {
        label: String,
        left: &'a Maze<'a>,
        right: &'a Maze<'a>,
        status: RefCell<Exploration>, // Allows internal mutability
    },
    Leaf {
        label: String,
    },
}

// Generic type to have handle life cycle of the maze
impl<'a> Maze<'a> {
    // Helper function to create a leaf
    fn leaf(label: &str) -> Maze<'a> {
        Maze::Leaf {
            label: label.to_string(),
        }
    }

    // Helper function to create a branch
    fn branch(label: &str, left: &'a Maze<'a>, right: &'a Maze<'a>) -> Maze<'a> {
        Maze::Branch {
            label: label.to_string(),
            left,
            right,
            status: RefCell::new(Exploration::UnExplored), // Initialize exploration status
        }
    }

    // Method to explore the maze
    fn explore(&self, trace: &mut Vec<String>) {
        match self {
            Maze::Branch { label, left, right, status } => {
                // Check the status of the branch
                let current_status = *status.borrow();
                match current_status {
                    Exploration::UnExplored => {
                        // Update the status and explore the children
                        status.replace(Exploration::Explored);
                        trace.push(label.clone());
                        left.explore(trace);
                        right.explore(trace);
                    }
                    Exploration::Explored => {
                        // If the status is already explored, just record the label
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

fn main() {
    // Create leaves
    let leaf2 = Maze::leaf("2");
    let leaf4 = Maze::leaf("4");
    let leaf5 = Maze::leaf("5");
    let leaf8 = Maze::leaf("8");

    // Create branches
    let branch3 = Maze::branch("3", &leaf4, &leaf5);
    let branch1 = Maze::branch("1", &leaf2, &branch3);
    let branch7 = Maze::branch("7", &leaf5, &leaf8);
    let branch6 = Maze::branch("6", &branch3, &branch7);
    let branch0 = Maze::branch("0", &branch1, &branch6);

    // Explore the maze
    let mut trace: Vec<String> = Vec::new();
    branch0.explore(&mut trace);

    // Print the trace
    println!("{:?}", trace);
}