fn main() {
    println!("Part 1: {}", p1("inputs/day3/input"));
    println!("Part 2: {}", p2("inputs/day3/input"));
}

fn helper(filepath: &str) -> Vec<((i32, i32), Vec<i32>)> {
    let mut symbols: Vec<((i32, i32), Vec<i32>)> = vec![];
    let mut nums: Vec<(i32, Vec<(i32, i32)>)> = vec![];

    std::fs::read_to_string(filepath)
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            let mut current = "".to_string();
            let mut position: Vec<(i32, i32)> = vec![];
            for (x, chr) in line.chars().enumerate() {
                if chr.is_digit(10) {
                    position.push((x as i32, y as i32));
                    current.push(chr);
                    if x == line.len() - 1 {
                        nums.push((current.parse::<i32>().unwrap(), position.clone()));
                        current = "".to_string();
                        position = vec![];
                    }
                    continue;
                }
                if chr != '.' {
                    symbols.push(((x as i32, y as i32), vec![]));
                }
                if current.is_empty() {
                    continue;
                }
                nums.push((current.parse::<i32>().unwrap(), position.clone()));
                current = "".to_string();
                position = vec![];
            }
        });

    let is_neighbor =
        |(x1, y1): (i32, i32), (x2, y2): (i32, i32)| (x1 - x2).abs() <= 1 && (y1 - y2).abs() <= 1;

    for ((x, y), numbers) in &mut symbols {
        for (num, positions) in &nums {
            if positions.iter().any(|p| is_neighbor(*p, (*x, *y))) {
                numbers.push(*num);
            }
        }
    }
    symbols
}

fn p1(filepath: &str) -> i32 {
    helper(filepath)
        .into_iter()
        .map(|(_, nums)| nums.iter().sum::<i32>())
        .sum()
}

fn p2(filepath: &str) -> i32 {
    helper(filepath)
        .into_iter()
        .filter_map(|(_, nums)| {
            if nums.len() == 2 {
                Some(nums.first().unwrap() * nums.last().unwrap())
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1("inputs/day3/test"), 4361);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2("inputs/day3/test"), 467835);
    }
}
