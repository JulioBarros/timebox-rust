use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use std::{thread, time};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TimeBoxItem {
    duration: f64,
    say: String,
    play: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TimeBoxList {
    timeboxes: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum TimeBox {
    Item(TimeBoxItem),
    List(TimeBoxList),
}

impl TimeBox {
    fn run(&self, boxes: &HashMap<String, TimeBox>) {
        match self {
            TimeBox::Item(i) => {
                let sound = "/System/Library/Sounds/".to_string() + &i.play + &".aiff".to_string();
                let d = i.duration * 60.0;
                println!("  Playing {}", sound);
                Command::new("/usr/bin/afplay")
                    .arg(sound)
                    .output()
                    .expect("Could not play sound");
                println!("  Saying {}", i.say);
                Command::new("/usr/bin/say")
                    .arg(&i.say)
                    .output()
                    .expect("Could not say a thing");
                println!("  Sleeping {}", i.duration);
                let duration = time::Duration::new(d.round() as u64, 0);
                thread::sleep(duration);
            }
            TimeBox::List(l) => {
                println!("Running a list of {}", l.timeboxes.len());
                for b in &l.timeboxes {
                    let bx = boxes.get(b).expect("Did not get box");
                    println!("- {:?}", bx);
                    bx.run(&boxes);
                }
            }
        }
    }
}

fn read_box_file(filename: &str) -> HashMap<String, TimeBox> {
    let file = std::fs::File::open(filename).expect("Could not read yaml file");
    let c: HashMap<String, TimeBox> = serde_yaml::from_reader(file).expect("could not parse yaml");
    c
}

fn main() {
    let boxes = read_box_file("timebox.yaml");

    let bx = boxes.get("work_box").unwrap();
    println!("{:?}", bx);
    bx.run(&boxes);
}
