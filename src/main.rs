use std::sync::{Arc, Mutex};
use std::thread;

// Enumeration to represent the exploration status
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Exploration {
    Explored,
    PartiallyExplored,
    UnExplored,
}

// Enumeration to represent the maze
#[derive(Debug)]
enum Maze {
    Branch {
        label: String,
        left: Arc<Maze>,
        right: Arc<Maze>,
        status: Mutex<Exploration>, // Interior mutability with Mutex
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

    // Concurrent exploration function
    fn explore(&self, work: &Arc<Mutex<Vec<Arc<Maze>>>>, trace: &mut Vec<String>) {
        match self {
            Maze::Branch { label, left, right, status } => {
                let mut current_status = status.lock().unwrap();
                match *current_status {
                    Exploration::UnExplored => {
                        *current_status = Exploration::PartiallyExplored;
                        trace.push(label.clone());
                        work.lock().unwrap().push(Arc::clone(left));
                        work.lock().unwrap().push(Arc::clone(self));
                    }
                    Exploration::PartiallyExplored => {
                        *current_status = Exploration::Explored;
                        trace.push(label.clone());
                        work.lock().unwrap().push(Arc::clone(right));
                    }
                    Exploration::Explored => {
                        trace.push(label.clone());
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
    let leaf2 = Maze::leaf("2");
    let leaf4 = Maze::leaf("4");
    let leaf5 = Maze::leaf("5");
    let leaf8 = Maze::leaf("8");

    let branch3 = Maze::branch("3", Arc::clone(&leaf4), Arc::clone(&leaf5));
    let branch1 = Maze::branch("1", Arc::clone(&leaf2), Arc::clone(&branch3));
    let branch7 = Maze::branch("7", Arc::clone(&leaf5), Arc::clone(&leaf8));
    let branch6 = Maze::branch("6", Arc::clone(&branch3), Arc::clone(&branch7));
    let branch0 = Maze::branch("0", Arc::clone(&branch1), Arc::clone(&branch6));

    branch0
}

fn main() {
    // Build the maze
    let maze = maze();

    // Shared work stack and trace
    let work = Arc::new(Mutex::new(vec![Arc::clone(&maze)]));
    let trace = Arc::new(Mutex::new(Vec::new()));

    // Number of threads
    let num_threads = 4;
    let mut handles = vec![];

    for i in 0..num_threads {
        let work_clone = Arc::clone(&work);
        let trace_clone = Arc::clone(&trace);

        // Spawn a thread
        let handle = thread::spawn(move || {
            while let Some(node) = work_clone.lock().unwrap().pop() {
                node.explore(&work_clone, &mut trace_clone.lock().unwrap());
                println!("Thread {} processed a node", i);
                thread::yield_now(); // Allow other threads to run
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final trace
    let trace_result = trace.lock().unwrap();
    println!("Final Trace: {:?}", *trace_result);
}
