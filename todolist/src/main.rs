use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!(
            r##"USAGE: binary <option> "<list>" 'list should be separated by , and placed in quotes' "##
        );
        return;
    }

    let name = "tasks.txt";

    let mut file = match OpenOptions::new().append(true).create(true).open(&name) {
        Err(why) => {
            println!("Couldn't create a file to save tasks {}", why);
            return;
        }
        Ok(file) => file,
    };

    let cmd = args[1].as_str();
    let tasks = &args[2];

    match cmd {
        "do" => add_task(&mut file, tasks),
        "see" => see_task(name),
        "done" => remove_task(name, tasks),
        _ => println!("INVALID FLAG"),
    }
}

fn add_task(file: &mut File, tasks: &str) {
    let task: Vec<&str> = tasks.split(',').collect();
    for task in task {
        if let Err(e) = writeln!(file, "{}", task) {
            println!("error writing to file {}", e);
        }
    }
}

fn see_task(name: &str) {
    let mut contents = String::new();
    let mut file = match File::open(name) {
        Ok(file) => file,
        Err(e) => {
            println!("error {}", e);
            return;
        }
    };

    match file.read_to_string(&mut contents) {
        Ok(_) => println!("{}", contents),
        Err(e) => {
            println!("error {e}");
            return;
        }
    }
}

fn remove_task(name: &str, task: &str) {
    let mut contents = String::new();
    let mut file = match OpenOptions::new().read(true).write(true).open(name) {
        Ok(file) => file,
        Err(e) => {
            println!("error opening file: {e}");
            return;
        }
    };

    file.read_to_string(&mut contents).expect("failed to read");

    let new_contents: String = contents
        .lines()
        .filter(|line| *line != task)
        .collect::<Vec<&str>>()
        .join("\n");

    // Truncate the file and write the new contents
    file.set_len(0).expect("failed to truncate file");
    file.write_all(new_contents.as_bytes())
        .expect("failed to write to file");
}
