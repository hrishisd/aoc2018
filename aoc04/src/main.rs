use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use serde_scan::scan;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type SleepHistogram = [usize; 60];

#[derive(Debug)]
enum Event {
    BeginsShift { guard_id: usize },
    FallsAsleep { sleep_time: usize },
    WakesUp { wake_time: usize },
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input")?;
    let events: Vec<Event> = input
        .lines()
        .sorted()
        .map(parse_event)
        .collect::<Result<Vec<Event>>>()?;
    let guard_to_sleep_histograms = make_guard_to_sleep_histogram_map(events);
    println!("Part 1: {}", part1(&guard_to_sleep_histograms)?);
    println!("Part 2: {}", part2(&guard_to_sleep_histograms)?);
    Ok(())
}

fn part1(guard_to_sleep_historgram: &HashMap<usize, SleepHistogram>) -> Result<usize> {
    let sleepiest_guard = guard_to_sleep_historgram
        .iter()
        .max_by_key(|(_, sleep_histogram)| sleep_histogram.iter().sum::<usize>())
        .map(|(guard, _total_sleep_time)| guard)
        .ok_or("unable to find guard with max sleep time")?;

    let minute_asleep_most = guard_to_sleep_historgram
        .get(sleepiest_guard)
        .ok_or("guard missing from map")?
        .iter()
        .enumerate()
        .max_by_key(|(_idx, val)| *val)
        .map(|(idx, _val)| idx)
        .ok_or("unable to find max val in histogram")?;

    Ok(sleepiest_guard * minute_asleep_most)
}

fn part2(guard_to_sleep_histogram: &HashMap<usize, SleepHistogram>) -> Result<usize> {
    // find the guard that is most frequenty asleep on the same minute * that minute
    let mut guard_most_freq_asleep_on_same_minute: usize = 0;
    let mut res_minute: usize = 0;
    let mut max_times_asleep: usize = 0;
    for (guard, sleep_histogram) in guard_to_sleep_histogram {
        let (minute, times_asleep) = sleep_histogram
            .iter()
            .enumerate()
            .max_by_key(|(_minute, times)| *times)
            .ok_or("err")?;
        if *times_asleep > max_times_asleep {
            max_times_asleep = *times_asleep;
            res_minute = minute;
            guard_most_freq_asleep_on_same_minute = *guard;
        }
    }
    Ok(res_minute * guard_most_freq_asleep_on_same_minute)
}

fn make_guard_to_sleep_histogram_map(events: Vec<Event>) -> HashMap<usize, SleepHistogram> {
    let mut guard_to_sleep_histogram = HashMap::new();
    let mut current_guard = 0;
    let mut sleep_start_time = 0;
    for event in events {
        match event {
            Event::BeginsShift { guard_id } => current_guard = guard_id,
            Event::FallsAsleep { sleep_time } => sleep_start_time = sleep_time,
            Event::WakesUp { wake_time } => {
                let histogram = guard_to_sleep_histogram
                    .entry(current_guard)
                    .or_insert_with(|| [0; 60]);
                for minute in sleep_start_time..wake_time {
                    histogram[minute] += 1;
                }
            }
        }
    }
    guard_to_sleep_histogram
}

fn parse_event(s: &str) -> Result<Event> {
    lazy_static! {
        static ref EVENT_REGEX: Regex =
            Regex::new(r"\[\d{4}-\d{2}-\d{2} \d{2}:(\d{2})\] (.*)").unwrap();
    }
    let capture_groups = EVENT_REGEX
        .captures_iter(s)
        .next()
        .ok_or("err while parsing regex")?;
    let minute = capture_groups[1].parse()?;
    let msg = &capture_groups[2];

    if msg.starts_with("Guard") {
        let guard_id: usize = scan!("Guard #{} begins shift" <- msg)?;
        Ok(Event::BeginsShift { guard_id })
    } else if msg.starts_with("falls") {
        Ok(Event::FallsAsleep { sleep_time: minute })
    } else if msg.starts_with("wake") {
        Ok(Event::WakesUp { wake_time: minute })
    } else {
        Err(From::from(format!("Can't parse message: {}", msg)))
    }
}
