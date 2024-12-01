use anyhow::{Context, Result};
use std::fs::File;
use std::path::Path;

mod day1;

type Answer = fn(File) -> Result<()>;

fn main() -> Result<()> {
    let open_inputfile = |filename: &str| -> Result<File> {
        let inputs_path = Path::new("./inputs");
        let filepath = inputs_path.join(filename);
        File::open(filepath).with_context(|| "failed to open inputfile {filepath}")
    };

    let answers: Vec<(Answer, &str)> = vec![
        (day1::answer1, "day1.ex.txt"),
        (day1::answer1, "day1.txt"),
        (day1::answer2, "day1.txt"),
    ];

    let (answer, filename) = answers[answers.len() - 1];
    let inputfile = open_inputfile(filename)?;
    answer(inputfile)?;

    Ok(())
}
