use std::fs;
use std::io::Error;
use std::path::Path;
use std::time::Instant;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

fn main() -> Result<(), Error> {
    println!("Advent of code 2015.");
    let before = Instant::now();

    if file_exists("day_1") {
        let before_1 = Instant::now();
        day_1::solve(&read_file("day_1")?);
        let after_1 = before_1.elapsed();
        println!(" Duration: {} ms.", after_1.as_secs() * 1000 + (after_1.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_2") {
        let before_2 = Instant::now();
        day_2::solve(&read_file("day_2")?);
        let after_2 = before_2.elapsed();
        println!(" Duration: {} ms.", after_2.as_secs() * 1000 + (after_2.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_3") {
        let before_3 = Instant::now();
        day_3::solve(&read_file("day_3")?);
        let after_3 = before_3.elapsed();
        println!(" Duration: {} ms.", after_3.as_secs() * 1000 + (after_3.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_4") {
        let before_4 = Instant::now();
        day_4::solve(&read_file("day_4")?);
        let after_4 = before_4.elapsed();
        println!(" Duration: {} ms.", after_4.as_secs() * 1000 + (after_4.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_5") {
        let before_5 = Instant::now();
        day_5::solve(&read_file("day_5")?);
        let after_5 = before_5.elapsed();
        println!(" Duration: {} ms.", after_5.as_secs() * 1000 + (after_5.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_6") {
        let before_6 = Instant::now();
        day_6::solve(&read_file("day_6")?);
        let after_6 = before_6.elapsed();
        println!(" Duration: {} ms.", after_6.as_secs() * 1000 + (after_6.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_7") {
        let before_7 = Instant::now();
        day_7::solve(&read_file("day_7")?);
        let after_7 = before_7.elapsed();
        println!(" Duration: {} ms.", after_7.as_secs() * 1000 + (after_7.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_8") {
        let before_8 = Instant::now();
        day_8::solve(&read_file("day_8")?);
        let after_8 = before_8.elapsed();
        println!(" Duration: {} ms.", after_8.as_secs() * 1000 + (after_8.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_9") {
        let before_9 = Instant::now();
        day_9::solve(&read_file("day_9")?);
        let after_9 = before_9.elapsed();
        println!(" Duration: {} ms.", after_9.as_secs() * 1000 + (after_9.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_10") {
        let before_10 = Instant::now();
        day_10::solve(&read_file("day_10")?);
        let after_10 = before_10.elapsed();
        println!(" Duration: {} ms.", after_10.as_secs() * 1000 + (after_10.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_11") {
        let before_11 = Instant::now();
        day_11::solve(&read_file("day_11")?);
        let after_11 = before_11.elapsed();
        println!(" Duration: {} ms.", after_11.as_secs() * 1000 + (after_11.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_12") {
        let before_12 = Instant::now();
        day_12::solve(&read_file("day_12")?);
        let after_12 = before_12.elapsed();
        println!(" Duration: {} ms.", after_12.as_secs() * 1000 + (after_12.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_13") {
        let before_13 = Instant::now();
        day_13::solve(&read_file("day_13")?);
        let after_13 = before_13.elapsed();
        println!(" Duration: {} ms.", after_13.as_secs() * 1000 + (after_13.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_14") {
        let before_14 = Instant::now();
        day_14::solve(&read_file("day_14")?);
        let after_14 = before_14.elapsed();
        println!(" Duration: {} ms.", after_14.as_secs() * 1000 + (after_14.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_15") {
        let before_15 = Instant::now();
        day_15::solve(&read_file("day_15")?);
        let after_15 = before_15.elapsed();
        println!(" Duration: {} ms.", after_15.as_secs() * 1000 + (after_15.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_16") {
        let before_16 = Instant::now();
        day_16::solve(&read_file("day_16")?);
        let after_16 = before_16.elapsed();
        println!(" Duration: {} ms.", after_16.as_secs() * 1000 + (after_16.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_17") {
        let before_17 = Instant::now();
        day_17::solve(&read_file("day_17")?);
        let after_17 = before_17.elapsed();
        println!(" Duration: {} ms.", after_17.as_secs() * 1000 + (after_17.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_18") {
        let before_18 = Instant::now();
        day_18::solve(&read_file("day_18")?);
        let after_18 = before_18.elapsed();
        println!(" Duration: {} ms.", after_18.as_secs() * 1000 + (after_18.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_19") {
        let before_19 = Instant::now();
        day_19::solve(&read_file("day_19")?);
        let after_19 = before_19.elapsed();
        println!(" Duration: {} ms.", after_19.as_secs() * 1000 + (after_19.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_20") {
        let before_20 = Instant::now();
        day_20::solve(&read_file("day_20")?);
        let after_20 = before_20.elapsed();
        println!(" Duration: {} ms.", after_20.as_secs() * 1000 + (after_20.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_21") {
        let before_21 = Instant::now();
        day_21::solve(&read_file("day_21")?);
        let after_21 = before_21.elapsed();
        println!(" Duration: {} ms.", after_21.as_secs() * 1000 + (after_21.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_22") {
        let before_22 = Instant::now();
        day_22::solve(&read_file("day_22")?);
        let after_22 = before_22.elapsed();
        println!(" Duration: {} ms.", after_22.as_secs() * 1000 + (after_22.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_23") {
        let before_23 = Instant::now();
        day_23::solve(&read_file("day_23")?);
        let after_23 = before_23.elapsed();
        println!(" Duration: {} ms.", after_23.as_secs() * 1000 + (after_23.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_24") {
        let before_24 = Instant::now();
        day_24::solve(&read_file("day_24")?);
        let after_24 = before_24.elapsed();
        println!(" Duration: {} ms.", after_24.as_secs() * 1000 + (after_24.subsec_nanos() / 1_000_000) as u64);
    }

    if file_exists("day_25") {
        let before_25 = Instant::now();
        day_25::solve(&read_file("day_25")?);
        let after_25 = before_25.elapsed();
        println!(" Duration: {} ms.", after_25.as_secs() * 1000 + (after_25.subsec_nanos() / 1_000_000) as u64);
    }

    let after = before.elapsed();
    println!("Total duration: {} ms", after.as_secs() * 1000 + (after.subsec_nanos() / 1_000_000) as u64);
    Ok(())
}

fn file_exists(path: &str) -> bool {
    let mut full_path = String::from("./res/");
    full_path.push_str(path);
    Path::new(&full_path).exists()
}

fn read_file(path: &str) -> Result<String, Error> {
    let mut full_path = String::from("./res/");
    full_path.push_str(path);
    let mut buffer = fs::read_to_string(full_path)
            .expect(&format!("Could not read file \"{}\"", path));
    buffer = buffer.replace('\r', "").trim_right().to_owned();
    Ok(buffer)
}
