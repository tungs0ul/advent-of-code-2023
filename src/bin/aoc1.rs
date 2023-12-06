fn main() {
    println!("Part 1: {}", p1("inputs/day1/input"));
    println!("Part 2: {}", p2("inputs/day1/input"));
}

fn p1(filepath: &str) -> u32 {
    std::fs::read_to_string(filepath)
        .unwrap()
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter_map(|c| match c.is_digit(10) {
                    true => Some(c.to_digit(10).unwrap()),
                    false => None,
                })
                .collect::<Vec<u32>>();
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum()
}

fn p2(filepath: &str) -> u32 {
    std::fs::read_to_string(filepath)
        .unwrap()
        .lines()
        .map(|line| {
            let digits = [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ];
            let digits_reversed = [
                "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
            ];

            let find_digit = |line: Vec<char>, rev: bool| -> u32 {
                let digits = if rev { digits_reversed } else { digits };
                let mut current = "".to_string();
                for c in line {
                    if c.is_digit(10) {
                        return c.to_digit(10).unwrap();
                    }
                    current.push(c);
                    match digits.iter().position(|digit| digit == &current.as_str()) {
                        Some(pos) => {
                            return pos as u32 + 1;
                        }
                        None => {}
                    };
                    if digits.iter().any(|digit| digit.starts_with(&current)) {
                        continue;
                    }
                    current.remove(0);
                }

                unreachable!("No digit found")
            };

            let first = find_digit(line.chars().collect(), false);
            let last = find_digit(line.chars().rev().collect(), true);
            first * 10 + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1("inputs/day1/test"), 142);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2("inputs/day1/test2"), 281);
    }
}
