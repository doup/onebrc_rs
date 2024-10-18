use core::f32;
use std::{
    collections::{BTreeMap, HashMap},
    env, fs,
    sync::Arc,
    thread::{self, available_parallelism},
};

#[derive(Debug, Clone)]
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

    let max_cores = available_parallelism().unwrap().get().next_power_of_two() / 2;
    let data = Arc::new(fs::read(format!("{file}.txt"))?);
    let slice_ranges = get_slices(&data, max_cores);
    let mut threads = vec![];

    for (from, to) in slice_ranges {
        let data = Arc::clone(&data);

        threads.push(thread::spawn(move || {
            let slice = &data[from..to];
            let lines = slice.split(|&byte| byte == b'\n');
            let mut stats: HashMap<&str, Station> = HashMap::new();

            for line in lines {
                if let Ok(line) = std::str::from_utf8(line) {
                    if line == "" {
                        break;
                    }

                    let (station, temp) = parse_line(line);

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
                } else {
                    eprintln!("Invalid UTF-8 sequence found in line.");
                }
            }

            stats
                .into_iter()
                .map(|(key, value)| (key.to_string(), value))
                .collect::<HashMap<String, Station>>()
        }));
    }

    // Calculate final stats
    let mut stats: BTreeMap<String, Station> = BTreeMap::new();

    for thread in threads {
        let thread_stats = thread.join().unwrap();

        for (station, station_stats) in thread_stats {
            if let Some(item) = stats.get_mut(&station) {
                if station_stats.max > item.max {
                    item.max = station_stats.max;
                }

                if station_stats.min < item.min {
                    item.min = station_stats.min;
                }

                item.sum += station_stats.sum;
                item.total += station_stats.total;
            } else {
                stats.insert(station, station_stats);
            }
        }
    }

    write_result(&file, &stats)?;

    Ok(())
}

fn find_next_line_break(data: &[u8], start_idx: usize) -> usize {
    for (idx, byte) in data.iter().enumerate().skip(start_idx) {
        if *byte == b'\n' {
            return idx;
        }
    }

    panic!("Index not found");
}

fn get_slices(data: &[u8], total: usize) -> Vec<(usize, usize)> {
    let mut slices = vec![(0, data.len())];

    while slices.len() < total {
        let mut new_slices = vec![];

        for slice in slices.iter() {
            let half_len = slice.0 + ((slice.1 - slice.0) / 2);
            let split_at = find_next_line_break(data, half_len);

            new_slices.push((slice.0, split_at));
            new_slices.push((split_at + 1, slice.1));
        }

        slices = new_slices;
    }

    slices
}

fn parse_line(line: &str) -> (&str, f32) {
    let mut parts = line.split(';');
    let station = parts.next().unwrap();
    let temp = parts.next().unwrap();
    let temp = &temp[0..temp.len()];
    let temp = temp.parse().unwrap();

    (station, temp)
}

fn write_result(file: &str, stats: &BTreeMap<String, Station>) -> std::io::Result<()> {
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
            writeln!(out, "}}")?;
        }
    }

    out.flush()?;

    Ok(())
}
