fn main() {
    println!("Part 1: {}", p1("inputs/day2/input"));
    println!("Part 2: {}", p2("inputs/day2/input"));
}

fn helper(filepath: &str) -> Vec<(u32, u32, u32)> {
    std::fs::read_to_string(filepath)
        .unwrap()
        .lines()
        .map(|line| {
            line.split(":")
                .last()
                .unwrap()
                .replace(";", ",")
                .split(",")
                .fold((0, 0, 0), |(red, green, blue), str| {
                    match str
                        .split(" ")
                        .filter(|x| !x.is_empty())
                        .collect::<Vec<&str>>()[..]
                    {
                        [x, "red"] => (red.max(x.parse::<u32>().unwrap()), green, blue),
                        [x, "green"] => (red, green.max(x.parse::<u32>().unwrap()), blue),
                        [x, "blue"] => (red, green, blue.max(x.parse::<u32>().unwrap())),
                        _ => unreachable!("Invalid input"),
                    }
                })
        })
        .collect()
}

fn p1(filepath: &str) -> u32 {
    helper(filepath)
        .into_iter()
        .enumerate()
        .filter_map(|(idx, (red, green, blue))| {
            if red > 12 || green > 13 || blue > 14 {
                None
            } else {
                Some(idx as u32 + 1)
            }
        })
        .sum()
}

fn p2(filepath: &str) -> u32 {
    helper(filepath)
        .into_iter()
        .map(|(red, green, blue)| red * green * blue)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1("inputs/day2/test"), 8);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2("inputs/day2/test"), 2286);
    }
}
