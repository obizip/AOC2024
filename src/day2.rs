use anyhow::Result;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn parse_inputfle(file: File) -> Result<Vec<Vec<isize>>> {
    let reader = BufReader::new(file);
    let mut reports = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let levels: Vec<isize> = line
            .split_whitespace()
            .map(|s| s.parse().expect("should be number"))
            .collect();
        reports.push(levels);
    }
    Ok(reports)
}

fn satisfy_all_pairs<F>(report: &[isize], cmp: F) -> bool
where
    F: Fn(isize, isize) -> bool,
{
    report.windows(2).all(|pair| cmp(pair[0], pair[1]))
}

fn is_increase(report: &[isize]) -> bool {
    satisfy_all_pairs(report, |a, b| a < b)
}

fn is_decrease(report: &[isize]) -> bool {
    satisfy_all_pairs(report, |a, b| a > b)
}

fn is_valid_range(report: &[isize]) -> bool {
    satisfy_all_pairs(report, |a, b| (1..=3).contains(&(a - b).abs()))
}

fn is_safe(report: &[isize]) -> bool {
    (is_increase(report) || is_decrease(report)) && is_valid_range(report)
}

pub fn answer1(file: File) -> Result<()> {
    let reports = parse_inputfle(file)?;
    let count = reports.into_iter().filter(|report| is_safe(report)).count();
    println!("{count}");
    Ok(())
}

pub fn answer2(file: File) -> Result<()> {
    let reports = parse_inputfle(file)?;

    let mut count = 0;
    for report in reports {
        for i in 0..report.len() {
            let mut removed_report = report.clone();
            removed_report.remove(i);
            if is_safe(&removed_report) {
                count += 1;
                break;
            }
        }
    }
    println!("{count}");
    Ok(())
}
