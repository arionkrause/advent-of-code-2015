use std::collections::HashMap;

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
//    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

fn decode_input(input: &str) -> HashMap<usize, Vec<(usize, usize)>> {
    let re = regex::Regex::new(r"^(?P<from>\w+) to (?P<to>\w+) = (?P<distance>\d+)$").unwrap();
    let mut locations = HashMap::new();
    let mut locations_indices = HashMap::new();

    for line in input.lines() {
        let captures = re.captures(&line).unwrap();
        let from = captures.name("from").unwrap().as_str().to_string();
        let to = captures.name("to").unwrap().as_str().to_string();
        let distance: usize = captures.name("distance").unwrap().as_str().parse().unwrap();

        let index_from = if let Some(index) = locations_indices.get(&from) {
            *index
        } else {
            locations_indices.insert(from, locations_indices.len());
            locations.insert(locations_indices.len() - 1,Vec::new());
            locations_indices.len() - 1
        };

        let index_to = if let Some(index) = locations_indices.get(&to) {
            *index
        } else {
            locations_indices.insert(to, locations_indices.len());
            locations.insert(locations_indices.len() - 1,Vec::new());
            locations_indices.len() - 1
        };

        locations.entry(index_from).and_modify(|distances| distances.push((index_to, distance)));
        locations.entry(index_to).and_modify(|distances| distances.push((index_from, distance)));
    }

    locations
}

mod part_1 {
    use crate::day_9::decode_input;
    use std::collections::VecDeque;
    use std::collections::HashMap;

    pub fn solve(input: &str) -> usize {
        let locations = decode_input(&input);
        shortest_route_total_distance(&locations)
    }

    fn shortest_route_total_distance(locations: &HashMap<usize, Vec<(usize, usize)>>) -> usize {
        let mut routes_distances = Vec::new();
        let mut queue: VecDeque<(usize, Vec<usize>, usize)> = locations.iter().map(|(&key, _)| (key, vec![key], 0)).collect();

        loop {
            if queue.is_empty() {
                break;
            }

            let (location, visited_locations, distance) = queue.pop_front().unwrap();

            if visited_locations.len() == locations.len() {
                routes_distances.push(distance);
                continue;
            }

            for (destination, destination_distance) in locations.get(&location).unwrap() {
                if visited_locations.contains(destination) {
                    continue;
                }

                let mut new_visited_locations = visited_locations.clone();
                new_visited_locations.push(location);
                queue.push_back((*destination, new_visited_locations, distance + destination_distance));
            }
        }

        *routes_distances.iter().min().unwrap()
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
