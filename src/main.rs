use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("RPCS3.log").expect("Failed to open RPCS3.log");
    let br = BufReader::new(file);

    let mut last_boot = 0;
    let mut cur_line = 0;

    // Determines last run first
    for line in br.lines() {
        cur_line += 1;
        let line = line.unwrap();
        if line.contains("LDR: Used configuration:") {
            last_boot = cur_line;
        }
    }

    if last_boot == 0 {
        println!("No boot was found");
        return;
    }

    println!("Last boot at line {}", last_boot);

    let mut last_activities: HashMap<String, VecDeque<String>> = HashMap::new();

    let file = File::open("RPCS3.log").expect("Failed to open RPCS3.log");
    let br = BufReader::new(file);

    cur_line = 0;
    for line in br.lines() {
        cur_line += 1;
        if cur_line < last_boot {
            continue;
        }

        let line = line.unwrap();

        let beg_name_off = line.find('{');
        if beg_name_off.is_none() {
            continue;
        }
        let beg_name_off = beg_name_off.unwrap() + 1;
        let end_name_off = line.find('}');
        if end_name_off.is_none() {
            continue;
        }
        let end_name_off = end_name_off.unwrap();
        let mut thr_name = line[beg_name_off..end_name_off].to_string().clone();

        if thr_name.len() == 0 {
            continue;
        }

        if &thr_name[thr_name.len() - 1..thr_name.len()] == "]" {
            let off_address = thr_name.rfind('[');
            if off_address.is_none() {
                continue;
            }
            let off_address = off_address.unwrap() - 1;
            thr_name.truncate(off_address);
        }

        let thr_activity = last_activities.entry(thr_name).or_insert(VecDeque::new());
        thr_activity.push_back(line);
        if thr_activity.len() > 10 {
            thr_activity.pop_front();
        }
    }

    for thr in &last_activities {
        println!("------------------------------------");
        println!("Last activity for {}", thr.0);
        for line in thr.1 {
            println!("{}", line);
        }
    }

}
