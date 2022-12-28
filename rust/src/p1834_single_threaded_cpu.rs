use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

#[derive(Eq)]
struct WaitingTask {
    ready_time: i32,
    task: Task
}

impl WaitingTask {
    fn new(ready_time: i32, processing_time: i32, idx: i32) -> WaitingTask {
        WaitingTask { ready_time, task: Task { processing_time, idx } }
    }
}

impl PartialEq for WaitingTask {
    fn eq(&self, other: &Self) -> bool {
        self.ready_time == other.ready_time
    }
}

impl Ord for WaitingTask {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.ready_time.cmp(&self.ready_time)
    }
}

impl PartialOrd for WaitingTask {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct Task {
    processing_time: i32,
    idx: i32
}

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let mut wait_queue = tasks.iter().enumerate()
            .map(|(idx, t)| WaitingTask::new(t[0], t[1], idx as i32))
            .collect::<BinaryHeap<WaitingTask>>();
        let mut processing_queue = BinaryHeap::new();
        let mut result = Vec::with_capacity(tasks.len());
        let mut time = 0;

        while !wait_queue.is_empty() || !processing_queue.is_empty() {
            while let Some(task) = wait_queue.peek() {
                if task.ready_time > time {
                    break;
                }
                processing_queue.push(Reverse(wait_queue.pop().unwrap().task))
            }

            if let Some(Reverse(task)) = processing_queue.pop() {
                result.push(task.idx);
                time += task.processing_time;
            }
            else if let Some(next_task) = wait_queue.peek() {
                if time < next_task.ready_time {
                    time = next_task.ready_time;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::util::parse_pairs;
    use super::*;

    #[test_case(parse_pairs("[[1,2],[2,4],[3,2],[4,1]]"), vec![0,2,3,1])]
    #[test_case(parse_pairs("[[7,10],[7,12],[7,5],[7,4],[7,2]]"), vec![4,3,2,0,1])]
    #[test_case(parse_pairs("[[7,2],[7,2],[7,2]]"), vec![0,1,2])]
    #[test_case(parse_pairs("[[5,2],[7,2],[9,4],[6,3],[5,10],[1,1]]"), vec![5, 0, 1, 3, 2, 4])]
    fn solve(tasks: Vec<Vec<i32>>, expected: Vec<i32>) {
        assert_eq!(expected, Solution::get_order(tasks));
    }
}