pub struct Solution1;

// BFS solution
impl Solution1 {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        enum Action { Explore(usize, usize, i32), Dive }

        let g = flights.iter().fold(vec![vec![]; n as usize], |mut g, el| { g[el[0] as usize].push(&el[1..]); g });

        let (src, dst) = (src as usize, dst as usize);
        let mut depth = 0;
        let mut min_cost = None;
        let mut queue = std::collections::VecDeque::from(vec![Action::Explore(src, src, 0), Action::Dive]);
        let mut travel_cost = vec![None; n as usize];
        while let Some(action) = queue.pop_front() {
            match action {
                Action::Dive => {
                    if depth > k {
                        break;
                    }
                    depth += 1;
                    queue.push_back(Action::Dive);
                },
                Action::Explore(node, parent, cost) => {
                    travel_cost[node] = travel_cost[node]
                        .map(|prev_cost| cost.min(prev_cost))
                        .or(Some(cost));
                    if node==dst {
                        min_cost = min_cost.map(|curr| cost.min(curr)).or(Some(cost));
                    }
                    else if depth <= k {
                        queue.extend(g[node].iter()
                            .filter(|el| {
                                let next = el[0] as usize;
                                next != parent && travel_cost[next].map_or(true, |prev_cost| prev_cost >= cost + el[1])
                            })
                            .map(|el| Action::Explore(el[0] as usize, node, cost + el[1])));
                    }
                }
            }
        }
        min_cost.map_or(-1, |cost| cost)
    }
}

// Bellman Ford Solution
// See https://leetcode.com/explore/featured/card/graph/622/single-source-shortest-path-algorithm/3864/
pub struct Solution2;
impl Solution2 {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let (src, dst) = (src as usize, dst as usize);
        let mut travel_costs = vec![None; n as usize];
        travel_costs[src] = Some(0);

        for _ in 0..=k {
            let mut next_costs = travel_costs.clone();

            for flight in flights.iter() {
                if let Some(start_cost) = travel_costs[flight[0] as usize] {
                    let flight_dst = flight[1] as usize;
                    next_costs[flight_dst] = next_costs[flight_dst]
                        .map(|cost| cost.min(start_cost + flight[2]))
                        .or(Some(start_cost + flight[2]));
                }
            }
            travel_costs = next_costs;
        }

        travel_costs[dst].unwrap_or(-1)
    }
}

// Dijkstra solution
// See https://leetcode.com/explore/featured/card/graph/622/single-source-shortest-path-algorithm/3862/
struct Solution3;
impl Solution3 {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        use std::collections::BinaryHeap;
        use std::cmp::Ordering;

        #[derive(Eq)]
        struct Route { cost: i32, dst: usize, stops: i32 }
        impl PartialEq for Route { fn eq(&self, other: &Self) -> bool { self.cost == other.cost } }
        impl Ord for Route { fn cmp(&self, other: &Self) -> Ordering { other.cost.cmp(&self.cost) } }
        impl PartialOrd for Route { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) } }

        let (src, dst) = (src as usize, dst as usize);
        let g = flights.iter().fold(vec![vec![]; n as usize], |mut g, el| { g[el[0] as usize].push(&el[1..]); g });
        let mut priority_queue = BinaryHeap::from(vec![Route { cost:0, dst: src, stops: 0 }]);
        let mut stops = vec![None; n as usize];
        while let Some(route) = priority_queue.pop() {
            if route.dst == dst {
                return route.cost;
            }
            if stops[route.dst].map_or(false, |curr_stops| curr_stops <= route.stops) {
                continue;
            }
            stops[route.dst] = Some(route.stops);
            if route.stops <= k {
                for next in g[route.dst].iter() {
                    let next_route = Route{ cost: route.cost + next[1], dst: next[0] as usize, stops: route.stops + 1 };
                    priority_queue.push(next_route);
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;
    use crate::util::parse_matrix;

    #[test_case(3, parse_matrix("[[0,1,100],[1,2,100],[0,2,500]]"), 0, 2, 1, 200)]
    #[test_case(3, parse_matrix("[[0,1,100],[1,2,100],[0,2,500]]"), 0, 2, 0, 500)]
    #[test_case(4, parse_matrix("[[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]]"), 0, 3, 1, 700)]
    #[test_case(11, parse_matrix("[[0,3,3],[3,4,3],[4,1,3],[0,5,1],[5,1,100],[0,6,2],[6,1,100],[0,7,1],[7,8,1],[8,9,1],[9,1,1],[1,10,1],[10,2,1],[1,2,100]]"), 0, 2, 4, 11)]
    fn solve1(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32, expected: i32) {
        assert_eq!(expected, Solution1::find_cheapest_price(n, flights, src, dst, k));
    }

    #[test_case(3, parse_matrix("[[0,1,100],[1,2,100],[0,2,500]]"), 0, 2, 1, 200)]
    #[test_case(3, parse_matrix("[[0,1,100],[1,2,100],[0,2,500]]"), 0, 2, 0, 500)]
    #[test_case(4, parse_matrix("[[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]]"), 0, 3, 1, 700)]
    #[test_case(11, parse_matrix("[[0,3,3],[3,4,3],[4,1,3],[0,5,1],[5,1,100],[0,6,2],[6,1,100],[0,7,1],[7,8,1],[8,9,1],[9,1,1],[1,10,1],[10,2,1],[1,2,100]]"), 0, 2, 4, 11)]
    fn solve2(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32, expected: i32) {
        assert_eq!(expected, Solution2::find_cheapest_price(n, flights, src, dst, k));
    }
    #[test_case(3, parse_matrix("[[0,1,100],[1,2,100],[0,2,500]]"), 0, 2, 1, 200)]
    #[test_case(3, parse_matrix("[[0,1,100],[1,2,100],[0,2,500]]"), 0, 2, 0, 500)]
    #[test_case(4, parse_matrix("[[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]]"), 0, 3, 1, 700)]
    #[test_case(11, parse_matrix("[[0,3,3],[3,4,3],[4,1,3],[0,5,1],[5,1,100],[0,6,2],[6,1,100],[0,7,1],[7,8,1],[8,9,1],[9,1,1],[1,10,1],[10,2,1],[1,2,100]]"), 0, 2, 4, 11)]
    fn solve3(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32, expected: i32) {
        assert_eq!(expected, Solution3::find_cheapest_price(n, flights, src, dst, k));
    }
}