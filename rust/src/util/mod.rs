pub mod binary_tree;
pub mod linked_list;

#[cfg(test)]
const TRIM_PAT: &[char] = &[' ', '[', ']'];

#[cfg(test)]
pub fn parse_matrix(matrix: &str) -> Vec<Vec<i32>> {
    let rows = matrix
        .trim_matches(TRIM_PAT)
        .split("],[");
    let mut result = Vec::<Vec<i32>>::new();
    for row in rows {
        result.push(row.trim_matches(TRIM_PAT).split(',').map(|el| el.parse().unwrap()).collect());
    }
    result
}

#[cfg(test)]
pub fn parse_pairs(edges: &str) -> Vec<Vec<i32>> {
    edges
        .trim_matches(TRIM_PAT)
        .split("],[")
        .map(|row| row
            .trim_matches(TRIM_PAT)
            .split_once(',')
            .map(|(a, b)| vec![a.parse().unwrap(), b.parse().unwrap()]).unwrap())
        .collect()
}