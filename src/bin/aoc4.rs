fn main() {
    println!("Part 1: {}", p1("inputs/day4/input"));
    println!("Part 2: {}", p2("inputs/day4/input"));
}

fn helper(filepath: &str) -> Vec<u32> {
    std::fs::read_to_string(filepath)
        .unwrap()
        .lines()
        .map(|line| {
            let game = line
                .split(":")
                .last()
                .unwrap()
                .trim()
                .split(" | ")
                .collect::<Vec<&str>>();
            let winning = game
                .first()
                .unwrap()
                .split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let having = game
                .last()
                .unwrap()
                .split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            having
                .into_iter()
                .filter_map(|x| match winning.contains(&x) {
                    true => Some(1),
                    false => None,
                })
                .collect::<Vec<i32>>()
                .len() as u32
        })
        .collect()
}

fn p1(filepath: &str) -> u32 {
    helper(filepath)
        .into_iter()
        .map(|x| match x {
            0 => 0,
            x => 2_u32.pow(x - 1),
        })
        .sum()
}

fn p2(filepath: &str) -> u32 {
    let mut x: Vec<u32> = vec![];
    helper(filepath)
        .into_iter()
        .enumerate()
        .for_each(|(idx, num)| {
            if x.len() < idx + 1 {
                x.push(0);
            }
            x[idx] += 1;
            for _ in 0..x[idx] {
                for i in 0..num {
                    if x.len() < idx + i as usize + 2 {
                        x.push(0);
                    }
                    x[idx + i as usize + 1] += 1;
                }
            }
        });
    x.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1("inputs/day4/test"), 13);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2("inputs/day4/test"), 30);
    }
}
