use std::{fs, path::Path};

fn main() {
    let path = Path::new("input.txt");
    let file_content = fs::read_to_string(path).expect("Unable to read file");

    let mut calories_per_elf: Vec<u32> = file_content
        .trim()
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .map(|x| x.parse::<u32>().expect("Non integer found"))
                .sum()
        })
        .collect();

    calories_per_elf.sort();
    calories_per_elf.reverse();

    let top_elf = calories_per_elf[0];
    println!("Top elf calories: {}", top_elf);

    let sum_top_three_elves: u32 = calories_per_elf[..3].iter().sum();
    println!("Sum of the top three elves: {}", sum_top_three_elves);
}
