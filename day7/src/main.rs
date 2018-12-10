use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, prelude::*, BufReader},
};

#[derive(Clone)]
struct Dag {
    edges: HashMap<char, HashSet<char>>,
}

impl Dag {
    fn new() -> Dag {
        Dag { edges: HashMap::new() }
    }

    // Adds an edge from `parent` to `child`, adding new nodes
    // to the graph if they don't exist yet.
    // TODO: make this panic if it introduces a loop
    fn add_edge(&mut self, parent: char, child: char) {
        // Add the child to the set so that we can find it later.
        self.edges.entry(child).or_insert(HashSet::new());
        let childs = self.edges.entry(parent).or_insert(HashSet::new());
        childs.insert(child);
    }

    // Find a node that doesn't have any parents.
    fn find_roots(&self) -> HashSet<char> {
        let mut possible_roots: HashSet<char> = self.edges.keys().map(|x| x.clone()).collect();
        for (_, childs) in self.edges.iter() {
            for child in childs {
                possible_roots.remove(child);
            }
        }
        possible_roots
    }

    fn remove_root(&mut self, node: &char) -> bool {
        self.edges.remove(node).is_some()
    }

    fn len(&self) -> usize {
        self.edges.len()
    }
}

struct WorkTimer {
    task: char,
    time_left: u8,
}

impl WorkTimer {
    fn new(task: char) -> WorkTimer {
        let time_left = 60 + (task as u8 - ('A' as u8) + 1);
        WorkTimer { task, time_left }
    }

    fn done(&self) -> bool {
        self.time_left == 0
    }

    fn work(&mut self) -> u8 {
        if self.time_left > 0 {
            self.time_left -= 1;
        }
        self.time_left
    }
}

struct Workers {
    tasks: Vec<WorkTimer>,
}

impl Workers {
    fn new() -> Workers {
        Workers { tasks: Vec::new() }
    }

    fn are_working(&self) -> HashSet<char> {
        self.tasks.iter().map(|x| x.task).collect()
    }

    fn len(&self) -> usize {
        self.tasks.len()
    }

    fn add_task(&mut self, task: char) {
        self.tasks.push(WorkTimer::new(task));
    }

    // Returns the set of tasks that are complete.
    // Those tasks are removed from the set of workers.
    fn work(&mut self) -> HashSet<char> {
        for x in self.tasks.iter_mut() {
            x.work();
        }
        let done: HashSet<char> = self.tasks.iter().filter(|x| x.done()).map(|x| x.task).collect();
        self.tasks.retain(|x| !x.done());
        done
    }
}

fn order_with_workers(count: usize, mut steps: Dag) -> (String, usize) {
    let mut ikea_order = String::new();

    let mut second = 0;

    let mut workers = Workers::new();

    while steps.len() > 0 {
        let done = workers.work();
        for task in done {
            steps.remove_root(&task);
            ikea_order.push(task);
        }
        let mut sorted: Vec<char>;
        {
            let ready = steps.find_roots();
            let working = workers.are_working();
            let ready_available = ready.difference(&working);
            sorted = ready_available.map(|x| x.clone()).collect();
        }
        sorted.as_mut_slice().sort_unstable();
        let mut ready_tasks = sorted.iter();
        while let Some(task) = ready_tasks.next() {
            if workers.len() >= count {
                break;
            }
            workers.add_task(*task);
        }
        println!("{}\t{:?}\t{}", second, workers.are_working(), ikea_order);
        second += 1;
    }
    println!("With {} workers, finished in {} steps in order: {}", count, second - 1, ikea_order);
    (ikea_order, second - 1)
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut steps = Dag::new();

    let mut lines = reader.lines();
    while let Some(Ok(line)) = lines.next() {
        let before: char = line.as_bytes()[5] as char;
        let after: char = line.as_bytes()[36] as char;
        println!("Step {} before {}", before, after);
        steps.add_edge(before, after);
    }

    order_with_workers(1, steps.clone());
    order_with_workers(5, steps.clone());
    Ok(())
}
