use std::fs::File;
use std::io;
use std::path::Path;

mod day1;

type Answer = fn(File) -> io::Result<()>;

fn main() -> io::Result<()> {
    let inputs_path = Path::new("./inputs");
    let open_file = |filename: &str| -> io::Result<File> {
        let filepath = inputs_path.join(filename);
        File::open(filepath)
    };

    let answers: Vec<(Answer, &str)> = vec![
        (day1::answer1, "day1.ex.txt"),
        (day1::answer1, "day1.txt"),
        (day1::answer2, "day1.txt"),
    ];

    let (answer, filename) = answers[answers.len() - 1];
    let inputfile = open_file(filename)?;
    answer(inputfile)?;

    Ok(())
}
