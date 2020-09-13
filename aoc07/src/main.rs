use serde_scan::scan;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

type Task = char;
type Result<T> = std::result::Result<T, std::boxed::Box<dyn std::error::Error>>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct InProgressTask {
    task: Task,
    remaining_time: usize,
}

impl InProgressTask {
    fn new(task: Task) -> Self {
        InProgressTask {
            task,
            remaining_time: (task as u8 - b'A' + 61) as usize,
        }
    }
    fn is_complete(&self) -> bool {
        self.remaining_time == 0
    }

    fn log_work(&mut self) {
        self.remaining_time -= 1;
    }
}

#[derive(Debug)]
struct Constraint {
    dependency: Task,
    dependent: Task,
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input")?;
    // TODO: don't build the actual vector here, build graph from an iterator
    let constraints = input
        .lines()
        .map(parse_constraint)
        .collect::<Result<Vec<Constraint>>>()?;
    let task_to_dependents = dependency_map_from_constraints(&constraints);
    println!("{:?}", part1(&task_to_dependents));
    println!("{:?}", part2(&task_to_dependents));
    Ok(())
}

fn part1(task_to_dependents: &HashMap<Task, Vec<Task>>) -> String {
    topo_sort(
        task_to_dependents,
        make_task_to_num_dependencies_map(&task_to_dependents),
    )
    .iter()
    .collect()
}

fn part2(task_to_dependents: &HashMap<Task, Vec<Task>>) -> usize {
    let mut task_to_num_dependencies = make_task_to_num_dependencies_map(&task_to_dependents);
    let mut num_idle_workers = 5;
    let mut work_in_progress = Vec::with_capacity(5);
    let mut time = 0;
    let mut task_queue: VecDeque<Task> = task_to_num_dependencies
        .iter()
        .filter(|(_task, &num_dependencies)| num_dependencies == 0)
        .map(|(&task, _num_dependencies)| task)
        .collect();

    while !task_queue.is_empty() || !work_in_progress.is_empty() {
        println!("{}", time);
        // assign work
        let num_tasks_to_assign = std::cmp::min(num_idle_workers, task_queue.len());
        for _ in 0..num_tasks_to_assign {
            let task = task_queue.pop_front().unwrap();
            work_in_progress.push(InProgressTask::new(task));
        }
        num_idle_workers -= num_tasks_to_assign;

        // do work
        for work in work_in_progress.iter_mut() {
            work.log_work();
        }
        let completed_tasks: HashSet<InProgressTask> = work_in_progress
            .iter()
            .filter(|task| task.is_complete())
            .cloned()
            .collect();

        // update state of task queue and in progress tasks
        for completed_task in &completed_tasks {
            for dependent in &task_to_dependents[&completed_task.task] {
                *task_to_num_dependencies.get_mut(dependent).unwrap() -= 1;
                // task_to_num_dependencies[dependent] -= 1;
                if task_to_num_dependencies[dependent] == 0 {
                    task_queue.push_back(*dependent);
                }
            }
        }
        work_in_progress.retain(|task| !completed_tasks.contains(task));
        // work_in_progress.drain_filter(|task| completed_tasks.contains(task));
        num_idle_workers += completed_tasks.len();
        time += 1;
    }

    time
}

fn make_task_to_num_dependencies_map(
    task_to_dependents: &HashMap<Task, Vec<Task>>,
) -> HashMap<Task, usize> {
    let mut task_to_num_dependencies: HashMap<Task, usize> =
        HashMap::with_capacity(task_to_dependents.len());
    for (&dependency, dependents) in task_to_dependents {
        // ensure the dependency is in the map
        task_to_num_dependencies.entry(dependency).or_insert(0);
        for dependent in dependents {
            *task_to_num_dependencies.entry(*dependent).or_insert(0) += 1;
        }
    }
    task_to_num_dependencies
}

fn topo_sort(
    task_to_dependents: &HashMap<Task, Vec<Task>>,
    mut task_to_num_dependencies: HashMap<Task, usize>,
) -> Vec<Task> {
    let mut queue = BinaryHeap::new();
    let mut ordering = Vec::with_capacity(task_to_dependents.len());
    let roots: Vec<Task> = task_to_num_dependencies
        .iter()
        .filter(|(_task, &num_dependencies)| num_dependencies == 0)
        .map(|(&task, _num_dependencies)| task)
        .collect();
    for root in roots {
        queue.push(Reverse(root));
    }

    while let Some(Reverse(task)) = queue.pop() {
        ordering.push(task);
        for dependent in task_to_dependents.get(&task).unwrap() {
            let num_dependencies = task_to_num_dependencies.get_mut(dependent).unwrap();
            *num_dependencies -= 1;
            if *num_dependencies == 0 {
                queue.push(Reverse(*dependent))
            }
        }
    }
    ordering
}

fn dependency_map_from_constraints(constraints: &[Constraint]) -> HashMap<Task, Vec<Task>> {
    let mut task_to_dependents = HashMap::with_capacity(26);
    for constraint in constraints {
        task_to_dependents
            .entry(constraint.dependency)
            .or_insert_with(Vec::new)
            .push(constraint.dependent);

        // also make sure the dependent appears in the map
        task_to_dependents
            .entry(constraint.dependent)
            .or_insert_with(Vec::new);
    }
    task_to_dependents
}

fn parse_constraint(s: &str) -> Result<Constraint> {
    let (dependency, dependent): (Task, Task) =
        scan!("Step {} must be finished before step {} can begin." <- s)?;
    Ok(Constraint {
        dependency,
        dependent,
    })
}
