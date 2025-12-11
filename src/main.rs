use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime,UNIX_EPOCH};
use std::process::Command;
use std::sync::OnceLock;
use tiny_http::{Server, Response, Header};
use markdown::to_html;
use clap::Parser;

/// A minimal markdown web server
#[derive(Parser, Debug)]
#[command(version, about = "Serve a markdown file as HTML")]
struct Args {
    /// IP Address to bind to
    #[arg(short='a', long="address", default_value="0.0.0.0")]
    address: String,

    /// Port to listen on
    #[arg(short='p', long="port", default_value_t = 3000)]
    port: u16,

    /// Text color HEX code
    #[arg(short='c', long="color", default_value="ffffff")]
    text_col: String,

    /// Background color HEX code
    #[arg(short='b', long="background-color", default_value = "2d3748")]
    bg_col: String,

    /// Title of the webpage
    #[arg(short='t', long="title", default_value="")]
    title: String,

    /// Path to the markdown file to host
    #[arg(default_value = "./index.md")]
    file: PathBuf,
}

// Get system time zone. This way `date` will be called just once
// on startup (if I understood how OnceLocks actually work) and store
// the result in TZ in memory and the logs will get the timezone from
// there instead of calling `date each time`
#[derive(Debug)]
struct TimeZoneInfo {
    offset_sec: i64,
    abbr: String,
}

static TZ: OnceLock<TimeZoneInfo> = OnceLock::new();

fn sys_timezone() -> TimeZoneInfo {
    // Time zone fetching
    let offset_output = Command::new("date")
        .arg("+%z")
        .output()
        .expect("Failed to get system time zone. Check manually if `date +%z` returns something");
    let offset_str: String = String::from_utf8_lossy(&offset_output.stdout).trim().to_string();

    // Calculate tz offset
    let offset: i64 = {
        let sign = if &offset_str[0..1] == "+" {1} else {-1};
        let hours: i64 = offset_str[1..3].parse().unwrap_or(0);
        let minutes: i64 = offset_str[3..5].parse().unwrap_or(0);
        sign * (hours*3600 + minutes*60)
    };

    // Get timestamp abbreviation
    let tz_output = Command::new("date")
        .arg("+%Z")
        .output()
        .expect("Failed to get system time zone code. Check manually if `date +%Z` returns something");
    let tz: String = String::from_utf8_lossy(&tz_output.stdout).trim().to_string();

    return TimeZoneInfo {
        offset_sec: offset,
        abbr: tz,
    };
}

fn get_tz() -> &'static TimeZoneInfo {
    return TZ.get_or_init(sys_timezone);
}

fn date() -> String {
    // Get current timestamp
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let tz = get_tz();
    let timestamp = (duration.as_secs() as i64 + tz.offset_sec) as u64;

    // Basic constants
    const SECONDS_PER_DAY: u64 = 86400;
    const SECONDS_PER_HOUR: u64 = 3600;
    const SECONDS_PER_MINUTE: u64 = 60;
    const DAYS_IN_NORMAL_YEAR: u64 = 365;
    const DAYS_IN_LEAP_YEAR: u64 = 366;

    // Calculate time components (HH:MM:SS)
    let total_days = timestamp / SECONDS_PER_DAY;
    let seconds_in_today = timestamp % SECONDS_PER_DAY;

    let hour = seconds_in_today / SECONDS_PER_HOUR;
    let minute = (seconds_in_today % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE;
    let second = seconds_in_today % SECONDS_PER_MINUTE;

    // Calculate Date (Year, Month, Day)
    // Iterate years from 1970, subtracting days until we find the current year
    let mut days_remaining = total_days;
    let mut year = 1970;

    loop {
        // Leap year rule: Divisible by 4, unless divisible by 100 but not 400
        let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        let days_in_year = if is_leap { DAYS_IN_LEAP_YEAR } else { DAYS_IN_NORMAL_YEAR };

        if days_remaining < days_in_year {
            break;
        }
        days_remaining -= days_in_year;
        year += 1;
    }

    // Determine month and day
    let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    // Days in months: Jan, Feb, Mar, ..., Dec
    let days_in_months = [
        31, if is_leap { 29 } else { 28 }, 31, 30, 31, 30, 
        31, 31, 30, 31, 30, 31
    ];

    let mut month = 1;
    for &days in &days_in_months {
        if days_remaining < days {
            break;
        }
        days_remaining -= days;
        month += 1;
    }

    // Remaining days is 0-indexed, so add 1 for the day of the month
    let day = days_remaining + 1;

    // Return formatted string
    return format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02} {}",
        year, month, day, hour, minute, second, tz.abbr
    );
}

// log macro (pretty sure this didn't need a comment but why not)
macro_rules! log {
    ($($arg:tt)*) => {
        println!("\x1b[90m{} |\x1b[0m {}", date(), format_args!($($arg)*));
    }
}

fn main() {
    // CLI args parsing
    let args = Args::parse();
    let bind_addr: String = format!("{}:{}", args.address, args.port);
    let text_col: String = args.text_col;
    let bg_col: String = args.bg_col;
    let title: String = if args.title != "" { format!("<title>{}</title>", args.title) } else { args.title };

    let server = Server::http(&bind_addr).unwrap();

    log!("Serving {} at http://{} ...", args.file.display(), bind_addr);

    for request in server.incoming_requests(){
        if let Some(addr) = request.remote_addr() {
            log!("Request received from \x1b[36m{}\x1b[0m", addr);
        } else {
            log!("Request received from \x1b[36msomewhere \x1b[90m(idk, maybe a unix socket, maybe somewhere in Lithuania)\x1b[0m");
        }
        let markdown = fs::read_to_string(&args.file)
            .unwrap_or_else(|_| "# File not found".into());
        let html_content = to_html(&markdown);

        let html_page = format!(
            "<!DOCTYPE html>
            <html>
                <head>
                    <meta charset=\"utf-8\">
                    {}
                    <style>
                        body {{ font-family: sans-serif; 
                            background-color: #{};
                            color: #{};
                            max-width: 800px; 
                            margin: auto;
                            padding: 2rem;
                        }}
                        a {{
                            color: #007acc;
                            text-decoration: none;
                        }}
                        a:hover {{ text-decoration: underline; }}
                    </style>
                </head>
                <body>{}</body>
            </html>", title, bg_col, text_col, html_content
            );

                    let response = Response::from_string(html_page)
                        .with_header(Header::from_bytes(
                                b"Content-Type",
                                b"text/html; charset=UTF-8",
                        ).unwrap());

                    let _ = request.respond(response);
    }
}
