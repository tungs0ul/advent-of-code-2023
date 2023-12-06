fn main() {
    println!("Part 1: {}", p1("inputs/day2/input"));
    println!("Part 2: {}", p2("inputs/day2/input"));
}

fn helper(filepath: &str) -> Vec<(i32, i32, i32)> {
    std::fs::read_to_string(filepath)
        .unwrap()
        .lines()
        .map(|line| {
            let line = line.split(":").last().unwrap().replace(";", ",");
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for str in line.split(",") {
                match str
                    .split(" ")
                    .filter(|x| !x.is_empty())
                    .collect::<Vec<&str>>()[..]
                {
                    [x, "red"] => red = red.max(x.parse::<i32>().unwrap()),
                    [x, "green"] => green = green.max(x.parse::<i32>().unwrap()),
                    [x, "blue"] => blue = blue.max(x.parse::<i32>().unwrap()),
                    _ => unreachable!("Invalid input"),
                }
            }
            (red, green, blue)
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

fn p2(filepath: &str) -> i32 {
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
