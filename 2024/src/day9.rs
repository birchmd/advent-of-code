use aoc_core::{digit_value, Solution};

pub struct Day9;

impl Solution<'_> for Day9 {
    type Input = Vec<u8>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(data: &'_ str) -> Self::Input {
        data.trim()
            .bytes()
            .map(|b| digit_value(b).unwrap())
            .collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let mut layout = make_layout(&input);
        make_contiguous(&mut layout);
        layout
            .into_iter()
            .enumerate()
            .filter_map(|(i, p)| Some(i * p?))
            .sum::<usize>()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let mut layout = make_layout(&input);
        let file_metadata = build_file_metadata(&input);
        make_contiguous2(&mut layout, &file_metadata);
        layout
            .into_iter()
            .enumerate()
            .filter_map(|(i, p)| Some(i * p?))
            .sum::<usize>()
    }
}

fn make_layout(disk_map: &[u8]) -> Vec<Option<usize>> {
    let mut layout = Vec::new();
    let mut file_id = 0;
    for (index, size) in disk_map.iter().enumerate() {
        let size = (*size) as usize;
        if index % 2 == 0 {
            for _ in 0..size {
                layout.push(Some(file_id))
            }
            file_id += 1;
        } else {
            for _ in 0..size {
                layout.push(None)
            }
        }
    }
    layout
}

fn make_contiguous(layout: &mut [Option<usize>]) {
    let n = layout.len();
    let mut i = 0;
    let mut j = n - 1;
    while layout[i].is_some() {
        i += 1;
    }
    while layout[j].is_none() {
        j -= 1;
    }
    while i < j {
        layout[i] = layout[j];
        layout[j] = None;
        while layout[i].is_some() {
            i += 1;
        }
        while layout[j].is_none() {
            j -= 1;
        }
    }
}

fn build_file_metadata(disk_map: &[u8]) -> Vec<(usize, u8)> {
    let mut result = Vec::new();
    let mut cumulative_sum = 0;
    for (index, size) in disk_map.iter().enumerate() {
        if index % 2 == 0 {
            result.push((cumulative_sum, *size));
        }
        cumulative_sum += (*size) as usize
    }
    result
}

fn make_contiguous2(layout: &mut [Option<usize>], file_metadata: &[(usize, u8)]) {
    let mut file_id = file_metadata.len();
    while file_id > 0 {
        file_id -= 1;
        let (start_index, size) = file_metadata[file_id];
        let Some(free_index) = find_free_space(layout, size as usize) else {
            continue;
        };
        if start_index < free_index {
            continue;
        }
        let mut i = free_index;
        let mut j = start_index;
        for _ in 0..size {
            layout[i] = Some(file_id);
            layout[j] = None;
            i += 1;
            j += 1;
        }
    }
}

fn find_free_space(layout: &[Option<usize>], required_size: usize) -> Option<usize> {
    let n = layout.len();
    let mut free_index = 0;
    loop {
        while layout[free_index].is_some() {
            free_index += 1;
            if free_index == n {
                return None;
            }
        }
        let mut free_size = 0;
        while layout[free_index + free_size].is_none() {
            if free_index + free_size + 1 == n {
                break;
            }
            free_size += 1;
        }
        if free_size == 0 {
            return None;
        } else if required_size <= free_size {
            return Some(free_index);
        }
        free_index += free_size;
        if free_index >= n {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day9_example.txt");

    #[test]
    fn test_part1() {
        let input = Day9::parse_input(EXAMPLE_INPUT);
        let output = Day9::part_1(input);
        assert_eq!(output, 1928);
    }

    #[test]
    fn test_part2() {
        let input = Day9::parse_input(EXAMPLE_INPUT);
        let output = Day9::part_2(input);
        assert_eq!(output, 2858);
    }
}
