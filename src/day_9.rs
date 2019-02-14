use std::collections::HashMap;

#[derive(Debug)]
struct Path {
    left: usize,
    right: usize,
    distance: usize,
}

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
//    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

fn decode_input(input: &str) -> Vec<Path> {
    let re = regex::Regex::new(r"^(?P<from>\w+) to (?P<to>\w+) = (?P<distance>\d+)$").unwrap();
    let mut locations = HashMap::new();
    let mut paths = Vec::new();

    for line in input.lines() {
        let captures = re.captures(&line).unwrap();
        let left = captures.name("from").unwrap().as_str().to_string();
        let right = captures.name("to").unwrap().as_str().to_string();
        let distance = captures.name("distance").unwrap().as_str().parse().unwrap();

        let index_left = if let Some(index) = locations.get(&left) {
            *index
        } else {
            locations.insert(left, locations.len());
            locations.len() - 1
        };

        let index_right = if let Some(index) = locations.get(&right) {
            *index
        } else {
            locations.insert(right, locations.len());
            locations.len() - 1
        };

        paths.push(Path {
            left: index_left,
            right: index_right,
            distance,
        });
    }

    paths
}

mod part_1 {
    use crate::day_9::decode_input;
    use crate::day_9::Path;
    use std::collections::VecDeque;

    pub fn solve(input: &str) -> usize {
        let paths = decode_input(&input);
        shortest_route_total_distance(&paths)
    }

    fn shortest_route_total_distance(paths: &Vec<Path>) -> usize {
        let mut shortest_route_total_distance = None;
        let mut locations = Vec::new();

        for path in paths {
            if !locations.contains(&path.left) {
                locations.push(path.left);
            }

            if !locations.contains(&path.right) {
                locations.push(path.right);
            }
        }

        let mut queue: VecDeque<(Vec<usize>, usize)> = locations.iter().map(|&location| (vec![location], 0)).collect();

        loop {
            if queue.is_empty() {
                break;
            }

            let (route, distance) = queue.pop_front().unwrap();

            if route.len() == locations.len()
                    && (shortest_route_total_distance.is_none()
                    || distance < shortest_route_total_distance.unwrap()) {
                shortest_route_total_distance = Some(distance);
            }

            for path in paths {
                if path.left == *route.last().unwrap()
                        && !route.contains(&path.right) {
                    let mut new_route = route.clone();
                    new_route.push(path.right);
                    queue.push_back((new_route, distance + path.distance));
                }

                if path.right == *route.last().unwrap()
                        && !route.contains(&path.left) {
                    let mut new_route = route.clone();
                    new_route.push(path.left);
                    queue.push_back((new_route, distance + path.distance));
                }
            }
        }

        shortest_route_total_distance.unwrap()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

        assert_eq!(solve(&input), 605);
    }
}

//mod part_2 {
//    pub fn solve(input: &str) -> usize {
//        0
//    }
//
//    #[cfg(test)]
//    #[test]
//    fn test_1() {
////        assert_eq!(solve(&""), );
//    }
//}
