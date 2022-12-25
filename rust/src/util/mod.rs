pub mod binary_tree;
pub mod linked_list;

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

pub fn parse_opt_values(values: &str) -> Vec<Option<i32>> {
    match values.trim_matches(TRIM_PAT) {
        "" => Vec::new(),
        other => other.split(',')
            .map(|token| match token.trim() {
                "" | "null" => None,
                other => Some(other.parse::<i32>().unwrap())
            })
            .collect()
    }            
}

pub fn parse_values(values: &str) -> Vec<i32> {
    match values.trim_matches(TRIM_PAT) {
        "" => Vec::new(),
        other => other.split(',')
            .map(|token| token.trim().parse::<i32>().unwrap())
            .collect()
    }            
}