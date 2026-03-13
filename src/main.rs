use std::env;
mod tasks;
mod lib;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let task_nr = env::args()
    .nth(1)
    .expect("Need to pass task number")
    .parse()
    .expect("Task number needs to be an integer");

    match task_nr {
        1 => {
            tasks::task1::run()?; // cargo run -- 1 < test_data/task1.txt
            Ok(())
        },
        2 => {
            tasks::task2::run()?; // cargo run -- 2. 1st: 1c0111001f010100061a024b53535009181c, 2nd: 686974207468652062756c6c277320657965
            Ok(())
        },
        3 => {
            tasks::task3::run()?; // cargo run -- 3. 1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
            Ok(())
        },
        4 => {
            tasks::task4::run()?; // cargo run -- 4
            Ok(())
        },
        5 => {
            tasks::task5::run()?;
            Ok(())
        },
        6 => {
            tasks::task6::run()?;
            Ok(())
        },
        7 => {
            tasks::task7::run()?;
            Ok(())
        },
        _ => unimplemented!("Task not implemented yet"),
    }
}
