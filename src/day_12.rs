use serde_json::Value;

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

fn get_value_sum(value: &Value, ignore_red: bool) -> i64 {
    let mut sum = 0;

    if value.is_array() {
        for inner_value in value.as_array().unwrap() {
            sum += get_value_sum(&inner_value, ignore_red);
        }
    } else if value.is_object() {
        let mut has_red = false;

        if ignore_red {
            for inner_value in value.as_object().unwrap().values() {
                if inner_value.is_string() && inner_value.as_str().unwrap() == "red" {
                    has_red = true;
                    break;
                }
            }
        }

        if !ignore_red || !has_red {
            for (_, inner_value) in value.as_object().unwrap() {
                sum += get_value_sum(&inner_value, ignore_red);
            }
        }
    } else if value.is_number() {
        sum += value.as_i64().unwrap() as i64;
    } else if value.is_string() {
        // Do nothing :)
    } else {
        panic!();
    }

    sum
}

mod part_1 {
    use serde_json::Value;
    use crate::day_12::get_value_sum;

    pub fn solve(input: &str) -> i64 {
        let value: Value = serde_json::from_str(input).unwrap();
        get_value_sum(&value, false)
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("[1,2,3]"), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(r#"{"a":2,"b":4}"#), 6);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("[[[3]]]"), 3);
    }

    #[test]
    fn test_4() {
        assert_eq!(solve(r#"{"a":{"b":4},"c":-1}"#), 3);
    }

    #[test]
    fn test_5() {
        assert_eq!(solve(r#"{"a":[-1,1]}"#), 0);
    }

    #[test]
    fn test_6() {
        assert_eq!(solve(r#"[-1,{"a":1}]"#), 0);
    }

    #[test]
    fn test_7() {
        assert_eq!(solve("[]"), 0);
    }

    #[test]
    fn test_8() {
        assert_eq!(solve("{}"), 0);
    }
}

mod part_2 {
    use serde_json::Value;
    use crate::day_12::get_value_sum;

    pub fn solve(input: &str) -> i64 {
        let value: Value = serde_json::from_str(input).unwrap();
        get_value_sum(&value, true)
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("[1,2,3]"), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(r#"[1,{"c":"red","b":2},3]"#), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
    }

    #[test]
    fn test_4() {
        assert_eq!(solve(r#"[1,"red",5]"#), 6);
    }
}
