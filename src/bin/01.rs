advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;

    for line in input.lines() {
        let num_iter: Vec<&str> = line
            .match_indices(|r#char| char::is_numeric(r#char))
            .map(|(_, n)| n)
            .collect();
        let first = num_iter.first().unwrap();
        let last = num_iter.last().unwrap();
        let num = (first.to_string() + last).parse::<u32>().unwrap();

        total += num;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        todo!();
        // assert_eq!(result, None);
    }
}
