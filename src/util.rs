pub fn to_lines(input: String) -> Vec<String> {
    input.lines().map(String::from).collect()
}

pub fn transpose<T>(input: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    assert!(input.iter().all(|sub| sub.len() == input[0].len()));
    (0..input[0].len())
        .map(|i| input.iter().map(|line| line[i]).collect())
        .collect()
}
