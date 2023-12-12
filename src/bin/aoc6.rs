fn main() {
    println!("Part 1: {}", p1("inputs/day6/input"));
    println!("Part 2: {}", p2("inputs/day6/input"));
}

fn p1(filepath: &str) -> usize {
    let lines: Vec<Vec<i32>> = std::fs::read_to_string(filepath)
        .unwrap()
        .lines()
        .map(|line| {
            line.split(":")
                .last()
                .unwrap()
                .split(" ")
                .filter_map(|x| (!x.is_empty()).then_some(x))
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    lines
        .first()
        .unwrap()
        .into_iter()
        .zip(lines.last().unwrap().into_iter())
        .map(|(time, distance)| {
            (0..*time)
                .into_iter()
                .filter_map(|hold| ((time - hold) * hold > *distance).then_some(hold))
                .collect::<Vec<i32>>()
                .len()
        })
        .fold(1, |acc, x| acc * x)
}

fn p2(filepath: &str) -> u64 {
    let lines = std::fs::read_to_string(filepath)
        .unwrap()
        .lines()
        .map(|line| {
            line.split(":")
                .last()
                .unwrap()
                .replace(" ", "")
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<u64>>();
    match lines[..] {
        [time, distance] => (0..time)
            .into_iter()
            .filter_map(|hold| ((time - hold) * hold > distance).then_some(hold))
            .collect::<Vec<u64>>()
            .len() as u64,
        _ => unreachable!("Invalid input"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1("inputs/day6/test"), 288);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2("inputs/day6/test"), 71503);
    }
}
