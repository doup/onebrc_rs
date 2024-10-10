use core::f32;
use std::{
    collections::BTreeMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

struct Station {
    min: f32,
    sum: f32,
    max: f32,
    total: usize,
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = format!(
        "data/measurements-{}",
        args.get(1)
            .expect("Argument not provided. Specify test data size number. e.g. 1000000000")
    );
    let f = File::open(format!("{file}.txt"))?;
    let mut buf = String::with_capacity(32);
    let mut reader = BufReader::new(f);
    let mut stats = BTreeMap::new();

    while reader.read_line(&mut buf)? != 0 {
        let (station, temp) = parse_line(&buf);

        let item = stats.entry(station).or_insert(Station {
            min: temp,
            sum: 0.,
            max: temp,
            total: 0,
        });

        if temp > item.max {
            item.max = temp;
        } else if temp < item.min {
            item.min = temp;
        }

        item.sum += temp;
        item.total += 1;

        buf.clear();
    }

    write_result(&file, stats)?;

    Ok(())
}

fn parse_line(line: &str) -> (String, f32) {
    let mut parts = line.split(';');
    let station = parts.next().unwrap().into();
    let temp = parts.next().unwrap();
    let temp = &temp[0..temp.len() - 1];
    let temp = temp.parse().unwrap();

    (station, temp)
}

fn write_result(file: &str, stats: BTreeMap<String, Station>) -> std::io::Result<()> {
    use std::io::{BufWriter, Write};
    let mut out = BufWriter::new(std::fs::File::create(format!("{file}.out.result"))?);
    let mut it = stats.iter().peekable();

    write!(out, "{{")?;

    while let Some((station, stats)) = it.next() {
        write!(
            out,
            "{station}={:.1}/{:.1}/{:.1}",
            stats.min,
            stats.sum / stats.total as f32,
            stats.max
        )?;

        if it.peek().is_some() {
            write!(out, ", ")?;
        } else {
            write!(out, "}}\n")?;
        }
    }

    out.flush()?;

    Ok(())
}
