use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_inputfile(file: File) -> Result<(Vec<isize>, Vec<isize>)> {
    let reader = BufReader::new(file);
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let splits: Vec<isize> = line
            .split_whitespace()
            .map(|s| s.parse().expect("should be integer"))
            .collect();

        if splits.len() != 2 {
            return Err(anyhow!("Each line must contain exactly two values"));
        }

        list1.push(splits[0]);
        list2.push(splits[1]);
    }

    Ok((list1, list2))
}

pub fn answer1(file: File) -> Result<()> {
    let (mut location_id_list1, mut location_id_list2) = parse_inputfile(file)?;
    location_id_list1.sort();
    location_id_list2.sort();
    let sum: isize = location_id_list1
        .into_iter()
        .zip(location_id_list2)
        .map(|(a, b)| (a - b).abs())
        .sum();
    println!("{sum}");
    Ok(())
}

pub fn answer2(file: File) -> Result<()> {
    let (location_id_list1, location_id_list2) = parse_inputfile(file)?;
    let similarity: isize = location_id_list1
        .into_iter()
        .map(|n1| n1 * location_id_list2.iter().filter(|n2| &&n1 == n2).count() as isize)
        .sum();
    println!("{similarity}");
    Ok(())
}
