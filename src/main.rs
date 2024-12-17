use std::sync::{Arc, Mutex};
use std::thread;

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
        left: Arc<Maze>,
        right: Arc<Maze>,
        status: Mutex<Exploration>, // Thread-safe interior mutability
    },
    Leaf {
        label: String,
    },
}

impl Maze {
    // Helper function to create a leaf
    fn leaf(label: &str) -> Arc<Maze> {
        Arc::new(Maze::Leaf {
            label: label.to_string(),
        })
    }

    // Helper function to create a branch
    fn branch(label: &str, left: Arc<Maze>, right: Arc<Maze>) -> Arc<Maze> {
        Arc::new(Maze::Branch {
            label: label.to_string(),
            left,
            right,
            status: Mutex::new(Exploration::UnExplored),
        })
    }

    // Exploration function
    fn explore(&self, work: Arc<Mutex<Vec<Arc<Maze>>>>, trace: &mut Vec<String>) {
        match self {
            Maze::Branch { label, left, right, status } => {
                let mut current_status = status.lock().unwrap();
                match *current_status {
                    Exploration::UnExplored => {
                        // Mark as partially explored and push children to the work stack
                        *current_status = Exploration::PartiallyExplored;
                        trace.push(label.clone());
                        let mut w = work.lock().unwrap();
                        w.push(Arc::clone(right)); // Push right child first
                        w.push(Arc::clone(left));  // Push left child second
                    }
                    Exploration::PartiallyExplored => {
                        // Mark as fully explored
                        *current_status = Exploration::Explored;
                        trace.push(label.clone());
                    }
                    Exploration::Explored => {
                        // Do nothing, already explored
                    }
                }
            }
            Maze::Leaf { label } => {
                trace.push(label.clone());
            }
        }
    }
}

// Function to construct the maze
fn maze() -> Arc<Maze> {
    // Create the leaves
    let leaf2 = Maze::leaf("2");
    let leaf4 = Maze::leaf("4");
    let leaf5 = Maze::leaf("5");
    let leaf8 = Maze::leaf("8");

    // Create the branches
    let branch3 = Maze::branch("3", Arc::clone(&leaf4), Arc::clone(&leaf5));
    let branch1 = Maze::branch("1", Arc::clone(&leaf2), Arc::clone(&branch3));
    let branch7 = Maze::branch("7", Arc::clone(&leaf5), Arc::clone(&leaf8));
    let branch6 = Maze::branch("6", Arc::clone(&branch3), Arc::clone(&branch7));
    let branch0 = Maze::branch("0", Arc::clone(&branch1), Arc::clone(&branch6));

    branch0
}

fn main() {
    let maze = maze();

    // Shared work stack protected by a Mutex
    let work = Arc::new(Mutex::new(vec![Arc::clone(&maze)]));

    // Shared trace output vector (per thread)
    let num_threads = 4;
    let mut handles = vec![];

    for i in 0..num_threads {
        let work = Arc::clone(&work);
        let thread_handle = thread::spawn(move || {
            let mut local_trace = Vec::new();

            while let Some(node) = {
                let mut w = work.lock().unwrap();
                if !w.is_empty() {
                    Some(w.pop().unwrap())
                } else {
                    None
                }
            } {
                node.explore(Arc::clone(&work), &mut local_trace);
                thread::sleep(std::time::Duration::from_millis(100));
                // thread::yield_now();
            }
            println!("Worker {} explored nodes: {:?}", i, local_trace);
        });
        handles.push(thread_handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Exploration complete.");
}
