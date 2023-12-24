use clap::{Parser, ValueEnum};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    #[arg(
        name = "time",
        help = "Positive integer followed by a time unit (s, m, h) or 'now'.\nUp to 18446744073709551615 seconds.\n[examples: now, 30s, 1m, 1h]"
    )]
    set_timer: String,

    #[arg(short, long, name = "operation", value_enum)]
    operation: Operation,
}

#[derive(Clone, ValueEnum)]
enum Operation {
    Shutdown,
    Reboot,
    Hibernate,
    #[cfg(target_os = "linux")]
    Sleep,
}

fn parse_timer(set_timer: String) -> Result<u64, &'static str> {
    let mut seconds: u64 = 0;

    if set_timer == "now" {
        return Ok(seconds);
    }

    let (value, unit) = set_timer.trim().split_at(set_timer.len() - 1);
    if value.is_empty() {
        return Err("Invalid parameter for <time>");
    }
    if value.starts_with('0') && value != "0" {
        return Err("Invalid parameter for <time>");
    }

    let number: u64 = match value.parse::<u64>() {
        Ok(value_to_num) => value_to_num,
        Err(_) => return Err("Invalid parameter for <time> or maximum time exceeded"),
    };

    let unit_in_seconds: u64 = match unit {
        "s" => 1,
        "m" => 60,
        "h" => 3600,
        _ => return Err("Invalid parameter for <time>"),
    };

    if number > std::u64::MAX / unit_in_seconds {
        return Err("Maximum time exceeded");
    }
    seconds += number * unit_in_seconds;
    Ok(seconds)
}

#[cfg(target_os = "windows")]
fn execute(mut time: u64, operation: Operation) {
    //ctrlc crate
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut cmd = Command::new("shutdown");
    match operation {
        Operation::Shutdown => {
            cmd.arg("/s").arg("/t").arg("0");
            println!("System will shutdown in {time} seconds. Use Ctrl+C to cancel.");
        }
        Operation::Reboot => {
            cmd.arg("/r").arg("/t").arg("0");
            println!("System will reboot in {time} seconds. Use Ctrl+C to cancel.");
        }
        Operation::Hibernate => {
            cmd.arg("/h");
            println!(
                "System will go into hibernation mode in {time} seconds. Use Ctrl+C to cancel."
            );
        }
    }

    while running.load(Ordering::SeqCst) && time > 0 {
        thread::sleep(Duration::from_secs(1));
        time -= 1;
    }
    if time == 0 {
        if let Err(e) = cmd.spawn() {
            eprintln!("error: {}", e);
        }
    } else {
        println!("\nOperation has been cancelled.");
    }
}

#[cfg(target_os = "linux")]
fn execute(mut time: u64, operation: Operation) {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut cmd = Command::new("systemctl");
    match operation {
        Operation::Shutdown => {
            cmd.arg("poweroff");
            println!("System will shutdown in {time} seconds. Use Ctrl+C to cancel.");
        }
        Operation::Reboot => {
            cmd.arg("reboot");
            println!("System will reboot in {time} seconds. Use Ctrl+C to cancel.");
        }
        Operation::Hibernate => {
            cmd.arg("hibernate");
            println!(
                "System will go into hibernation mode in {time} seconds. Use Ctrl+C to cancel."
            );
        }
        Operation::Sleep => {
            cmd.arg("suspend");
            println!("System will go into sleep mode in {time} seconds. Use Ctrl+C to cancel.");
        }
    }

    while running.load(Ordering::SeqCst) && time > 0 {
        thread::sleep(Duration::from_secs(1));
        time -= 1;
    }
    if time == 0 {
        if let Err(e) = cmd.spawn() {
            eprintln!("error: {}", e);
        }
    } else {
        println!("\nOperation has been cancelled.");
    }
}

fn main() {
    let args = Args::parse();

    //process timer
    let time = parse_timer(args.set_timer);
    let timer = match time {
        Ok(t) => t,
        Err(err) => {
            eprintln!("{err}.\n\nFor more information, try '--help'");
            return;
        }
    };

    //process operation
    execute(timer, args.operation);
}
