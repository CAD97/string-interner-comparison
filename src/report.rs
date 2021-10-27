use crate::alloc;
use alloc::Event;
use argh::FromArgs;
use bytesize::ByteSize;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};
use textplots::{Chart, Plot, Shape};

#[derive(FromArgs)]
/// Analyze report
#[argh(subcommand, name = "report")]
pub struct Report {
    #[argh(positional)]
    path: PathBuf,
}

trait Delta {
    fn delta(self) -> isize;
}

impl Delta for alloc::Event {
    fn delta(self) -> isize {
        match self {
            Event::Alloc { size, .. } => size as isize,
            Event::Freed { size, .. } => -(size as isize),
            Event::Point {} => 0,
        }
    }
}

impl Report {
    pub fn run(self) {
        let f = BufReader::new(File::open(&self.path).unwrap());
        let mut events: Vec<alloc::Event> = Default::default();

        for line in f.lines() {
            let line = line.unwrap();
            let ev: Event = serde_json::from_str(&line).unwrap();
            if !matches!(ev, Event::Point {}) {
                events.push(ev);
            }
        }
        println!("found {} events", events.len());

        let mut points = vec![];
        let mut curr_bytes = 0;
        let mut peak_bytes = 0;
        let mut alloc_events = 0;
        let mut alloc_bytes = 0;
        let mut freed_events = 0;
        let mut freed_bytes = 0;
        for (i, ev) in events.iter().copied().enumerate() {
            curr_bytes += ev.delta();
            points.push((i as f32, curr_bytes as f32));

            if peak_bytes < curr_bytes {
                peak_bytes = curr_bytes;
            }
            match ev {
                Event::Alloc { size, .. } => {
                    alloc_events += 1;
                    alloc_bytes += size;
                }
                Event::Freed { size, .. } => {
                    freed_events += 1;
                    freed_bytes += size;
                }
                Event::Point {} => unreachable!(),
            }
        }
        Chart::new(120, 80, 0.0, points.len() as f32)
            .lineplot(&Shape::Steps(&points[..]))
            .nice();

        println!("     total events | {}", events.len());
        println!("      peak bytes  | {}", ByteSize(peak_bytes as _));
        println!("     ----------------------------");
        println!("     alloc events | {}", alloc_events);
        println!("     alloc bytes  | {}", ByteSize(alloc_bytes as _));
        println!("     ----------------------------");
        println!("     freed events | {}", freed_events);
        println!("     freed bytes  | {}", ByteSize(freed_bytes as _));
    }
}
