pub struct Solution;

enum Move { Cell(usize), Next }
impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        fn get_jump(cell_idx: usize, board: &[Vec<i32>]) -> Option<usize> {
            let n = board.len();
            let row_idx = n - 1 - (cell_idx - 1) / n;
            let cell_idx = match (n - row_idx) % 2 {
                1 => (cell_idx - 1) % n,
                _ => n - 1 - (cell_idx - 1) % n,
            };
            match board[row_idx][cell_idx] {
                cell_val if cell_val==-1 => None,
                cell_val => Some(cell_val as usize),
            }
        }
        let n = board.len();
        let nn = n*n;
        let mut count = 0;
        let mut visited = vec![false; n*n + 1];
        visited[1] = true;
        let mut queue = std::collections::VecDeque::from([Move::Cell(1), Move::Next]);
        'queue_loop: while let Some(mv) = queue.pop_front() {
            match mv {
                Move::Next => {
                    if queue.is_empty() {
                        return -1;
                    }
                    queue.push_back(Move::Next);
                    count += 1;
                },
                Move::Cell(i) => {
                    if i + 6 >= nn {
                        break 'queue_loop;
                    }
                    let mut max_non_jump_advance = None;
                    for step in 1..=6 {
                        let next = i + step;
                        if !visited[next] {
                            visited[next] = true;
                            if let Some(jump) = get_jump(next, &board) {
                                if jump == nn {
                                    break 'queue_loop;
                                }
                                queue.push_back(Move::Cell(jump));
                            }
                            else {
                                max_non_jump_advance = Some(next);
                            }
                        }
                    }
                    if let Some(next) = max_non_jump_advance {
                        queue.push_back(Move::Cell(next));
                    }
                }
            }
        }

        count + 1
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;
    use crate::util::parse_matrix;

    #[test_case(parse_matrix("[[-1,-1,-1,46,47,-1,-1,-1],[51,-1,-1,63,-1,31,21,-1],[-1,-1,26,-1,-1,38,-1,-1],[-1,-1,11,-1,14,23,56,57],[11,-1,-1,-1,49,36,-1,48],[-1,-1,-1,33,56,-1,57,21],[-1,-1,-1,-1,-1,-1,2,-1],[-1,-1,-1,8,3,-1,6,56]]"), 4)]
    #[test_case(parse_matrix("[[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,35,-1,-1,13,-1],[-1,-1,-1,-1,-1,-1],[-1,15,-1,-1,-1,-1]]"), 4)]
    #[test_case(parse_matrix("[[-1,7,-1],[-1,6,9],[-1,-1,2]]"), 1)]
    #[test_case(parse_matrix("[[-1,-1,-1],[-1,9,8],[-1,8,9]]"), 1)]
    #[test_case(parse_matrix("[[-1,1,1,1],[-1,7,1,1],[16,1,1,1],[-1,1,9,1]]"), 3)]
    fn solve(board: Vec<Vec<i32>>, expected: i32) {
        assert_eq!(expected, Solution::snakes_and_ladders(board));
    }

}