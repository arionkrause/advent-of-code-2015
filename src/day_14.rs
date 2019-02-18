pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input, 2503));
    println!("Part 2: {}.", part_2::solve(&input, 2503));
    println!();
}

mod part_1 {
    use regex::Regex;

    struct Reindeer {
        distance_travelled_per_second_in_kilometers: usize,
        stamina_in_seconds: usize,
        rest_period_in_seconds: usize,
    }

    pub fn solve(input: &str, race_duration_in_seconds: usize) -> usize {
        decode_input(&input).iter().map(|reindeer| {
            reindeer.distance_travelled_per_second_in_kilometers * reindeer.stamina_in_seconds * (race_duration_in_seconds / (reindeer.stamina_in_seconds + reindeer.rest_period_in_seconds))
                    + reindeer.distance_travelled_per_second_in_kilometers * reindeer.stamina_in_seconds.min(race_duration_in_seconds % (reindeer.stamina_in_seconds + reindeer.rest_period_in_seconds))
        }).max().unwrap()
    }

    fn decode_input(input: &str) -> Vec<Reindeer> {
        let re = Regex::new(r"^\w+ can fly (?P<distance_travelled_per_second_in_kilometers>\d+) km/s for (?P<stamina_in_seconds>\d+) seconds, but then must rest for (?P<rest_period_in_seconds>\d+) seconds\.$").unwrap();
        let mut reindeers = Vec::new();

        for line in input.lines() {
            let captures = re.captures(&line).unwrap();

            reindeers.push(Reindeer {
                distance_travelled_per_second_in_kilometers: captures.name("distance_travelled_per_second_in_kilometers").unwrap().as_str().parse().unwrap(),
                stamina_in_seconds: captures.name("stamina_in_seconds").unwrap().as_str().parse().unwrap(),
                rest_period_in_seconds: captures.name("rest_period_in_seconds").unwrap().as_str().parse().unwrap(),
            });
        }

        reindeers
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.", 1000), 1120);
    }
}

mod part_2 {
    use regex::Regex;

    struct Reindeer {
        distance_travelled_per_second_in_kilometers: usize,
        stamina_in_seconds: usize,
        rest_period_in_seconds: usize,
        distance_travelled_so_far_in_kilometers: usize,
        stamina_left_in_seconds: usize,
        rest_period_left_in_seconds: usize,
        points: usize,
    }

    pub fn solve(input: &str, race_duration_in_seconds: usize) -> usize {
        let mut reindeers = decode_input(&input);

        for _ in 0..race_duration_in_seconds {
            reindeers.iter_mut().for_each(|reindeer| {
                if reindeer.rest_period_left_in_seconds == 0 {
                    reindeer.distance_travelled_so_far_in_kilometers += reindeer.distance_travelled_per_second_in_kilometers;
                    reindeer.stamina_left_in_seconds -= 1;

                    if reindeer.stamina_left_in_seconds == 0 {
                        reindeer.rest_period_left_in_seconds = reindeer.rest_period_in_seconds;
                    }
                } else {
                    reindeer.rest_period_left_in_seconds -= 1;

                    if reindeer.rest_period_left_in_seconds == 0 {
                        reindeer.stamina_left_in_seconds = reindeer.stamina_in_seconds;
                    }
                }
            });

            let furthest_distance_travelled_so_far_in_kilometers = reindeers.iter().map(|reindeer| reindeer.distance_travelled_so_far_in_kilometers).max().unwrap();

            reindeers.iter_mut()
                    .filter(|reindeer| reindeer.distance_travelled_so_far_in_kilometers == furthest_distance_travelled_so_far_in_kilometers)
                    .for_each(|reindeer| reindeer.points += 1);
        }

        reindeers.iter().map(|r| r.points).max().unwrap()
    }

    fn decode_input(input: &str) -> Vec<Reindeer> {
        let re = Regex::new(r"^\w+ can fly (?P<distance_travelled_per_second_in_kilometers>\d+) km/s for (?P<stamina_in_seconds>\d+) seconds, but then must rest for (?P<rest_period_in_seconds>\d+) seconds\.$").unwrap();
        let mut reindeers = Vec::new();

        for line in input.lines() {
            let captures = re.captures(&line).unwrap();
            let stamina_in_seconds = captures.name("stamina_in_seconds").unwrap().as_str().parse().unwrap();

            reindeers.push(Reindeer {
                distance_travelled_per_second_in_kilometers: captures.name("distance_travelled_per_second_in_kilometers").unwrap().as_str().parse().unwrap(),
                stamina_in_seconds,
                rest_period_in_seconds: captures.name("rest_period_in_seconds").unwrap().as_str().parse().unwrap(),
                distance_travelled_so_far_in_kilometers: 0,
                stamina_left_in_seconds: stamina_in_seconds,
                rest_period_left_in_seconds: 0,
                points: 0
            });
        }

        reindeers
    }

    #[test]
    fn test_1() {
        assert_eq!(solve("Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.", 1000), 689);
    }
}
