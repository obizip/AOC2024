use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

fn parse_file<T: FromStr + Clone>(file: File) -> io::Result<(Vec<T>, Vec<T>)>
where
    T::Err: std::fmt::Debug,
{
    let reader = BufReader::new(file);
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let splits: Vec<T> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse value"))
            .collect();

        if splits.len() != 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Each line must contain exactly two values",
            ));
        }

        list1.push(splits[0].clone());
        list2.push(splits[1].clone());
    }

    Ok((list1, list2))
}

pub fn answer1(file: File) -> io::Result<()> {
    match parse_file::<isize>(file) {
        Ok((mut location_id_list1, mut location_id_list2)) => {
            location_id_list1.sort();
            location_id_list2.sort();
            let sum: isize = location_id_list1
                .into_iter()
                .zip(location_id_list2)
                .map(|(a, b)| (a - b).abs())
                .sum();
            println!("{sum}");
        }
        Err(e) => eprintln!("Error: {e}"),
    }

    Ok(())
}

pub fn answer2(file: File) -> io::Result<()> {
    match parse_file::<isize>(file) {
        Ok((location_id_list1, location_id_list2)) => {
            let similarity: isize = location_id_list1
                .into_iter()
                .map(|n1| n1 * location_id_list2.iter().filter(|n2| &&n1 == n2).count() as isize)
                .sum();
            println!("{similarity}");
        }
        Err(e) => eprintln!("Error: {e}"),
    }

    Ok(())
}
