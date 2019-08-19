// |y axis
// |
// |  1   2   3
// |  4   5   6
// |  7   8   9
// |      0
// |------------------------- x axis
// ^
// | (0,0)

/*
    Statics
*/

use lazy_static::lazy_static;
lazy_static! {

    // geometric locations of all the numbers
    static ref NUM_ARR : [Location; 10] = [
        Location::new(2,1),  //zero
        Location::new(1,4), // one
        Location::new(2,4), // two
        Location::new(3,4), // three
        Location::new(1,3), // four
        Location::new(2,3), // five
        Location::new(3,3), // six
        Location::new(1,2), // seven
        Location::new(2,2), // eight
        Location::new(3,2), // nine
    ];

}

// 5 seconds + 2 % total seconds
const PERCENT_ALLOWANCE: f32 = 0.2;
const BASE_ALLOWANCE: u16 = 5;

// Time to move one number up / down
const ONE_UNIT_MOVE_TIME: f32 = 0.3;
const KEY_PRESS_TIME: f32 = 0.1;

trait TotalSeconds {
    fn total_seconds(&self) -> u16;
}

impl TotalSeconds for (u16, u16) {
    fn total_seconds(&self) -> u16 {
        let (min, sec) = self;
        let total_secs = ((*min as u16) * 60_u16) + *sec as u16;

        total_secs
    }
}

impl TotalSeconds for Vec<u8> {
    fn total_seconds(&self) -> u16 {
        let make_str = |x: &[u8]| {
            x.iter()
                .map(|x| x.to_string())
                .collect::<String>()
                .parse()
                .expect("err parsing already good string")
        };

        let (min, sec) = if self.len() > 2 {
            let (mins, secs) = self.split_at(self.len() - 2);

            let mins: u16 = make_str(mins);
            let secs: u16 = make_str(secs);

            (mins, secs)
        } else if self.len() == 2 {
            let mins = 0;
            let secs = make_str(self.as_slice());

            (mins, secs)
        } else if self.len() == 1 {
            let secs = "0".to_owned() + &self[0].to_string();

            (0, secs.parse().expect("secs not parsed"))
        } else {
            (0, 0)
        };

        (min, sec).total_seconds()
    }
}

/// Time to complete a combination of numbers
fn combination_time(data: &Vec<u8>) -> f32 {
    let mut running_time = 0.;
    if data.len() <= 1 {
        running_time
    } else {
        let mut previous = data[0];
        let mut current = data[1];

        running_time += KEY_PRESS_TIME * data.len() as f32;

        running_time += time_to_move(previous, current);

        for i in 2..data.len() {
            previous = current;
            current = data[i];
            running_time += time_to_move(previous, current);
            running_time += KEY_PRESS_TIME;
        }

        running_time
    }
}

/// time to move between two numbers
fn time_to_move(num_1: u8, num_2: u8) -> f32 {
    let a = NUM_ARR[num_1 as usize];
    let b = NUM_ARR[num_2 as usize];
    a.distance(b) * ONE_UNIT_MOVE_TIME
}


#[derive(Copy, Clone)]
struct Location {
    x: i8,
    y: i8,
}
impl Location {
    fn new(x: i8, y: i8) -> Self {
        Self { x: x, y: y }
    }
    fn norm(&self) -> f32 {
        let square_sum = self.x.pow(2) + self.y.pow(2);
        (square_sum as f32).sqrt()
    }

    fn distance(self, other: Self) -> f32 {
        (self - other).norm()
    }
}

impl std::ops::Sub for Location {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Clone, Debug)]
struct Time {
    min: u8,
    sec: u8,
    bounds: Bounds,
    digits: Vec<u8>,
}
impl Time {
    fn new_str(arg: &str) -> Result<Self, Error> {
        let split_index = if let Some(index) = arg.find(":") {
            index
        } else if let Some(index) = arg.find(".") {
            index
        } else {
            return Err(Error::MissingChar);
        };

        if arg.len() - (split_index + 1) > 2 {
            return Err(Error::InvalidSeconds);
        }

        let (minutes, seconds_sep) = arg.split_at(split_index);
        let seconds = match seconds_sep.get(1..) {
            Some(good_string) => good_string,
            None => return Err(Error::BadSlice),
        };

        let min = minutes.parse()?;
        let sec = seconds.parse()?;

        let total_secs = (min as u16, sec as u16).total_seconds();
        let digits = digits_to_compose(total_secs);

        let x = Self {
            min: min,
            sec: sec,
            bounds: Bounds::from_total_secs(total_secs),
            digits: digits,
        };

        Ok(x)
    }

    fn base_time(&self) -> f32 {
        combination_time(&self.digits)
    }
}

#[derive(Clone, Debug)]
struct Bounds {
    upper: u16,
    lower: u16,
    total_sec: u16,
}
impl Bounds {
    fn from_total_secs(total_secs: u16) -> Self {
        let diff = ((total_secs as f32) * PERCENT_ALLOWANCE).ceil() as u16;
        let total_diff = BASE_ALLOWANCE + diff;

        let upper = total_secs + total_diff;
        let lower = total_secs - total_diff;

        Self {
            upper: upper,
            lower: lower,
            total_sec: total_secs,
        }
    }
}

#[derive(Debug)]
enum Error {
    MissingChar,    // missing ":" or "." in the string
    NotNumber,      // the sides of the string separated by ":" "." dont have a number
    BadSlice,       // not utf8 string we can slice (also NotNumber)
    InvalidSeconds, // length of the number of seconds passed in was greater than 2
}
impl Error {
    fn print_err(&self) {
        match self {
            Error::MissingChar => eprintln! {"Missing a `:` or `.` character for a timestamp"},
            Error::NotNumber => {
                eprintln! {"Non-Numeric character (or : / .) included in timestamp"}
            }
            Error::BadSlice => eprintln! {"Non UTF8 characters included in timer"},
            Error::InvalidSeconds => eprintln! {"Must be a maximum of two characters of seconds"},
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(_: std::num::ParseIntError) -> Self {
        Error::NotNumber
    }
}

/// Helper function for slicing strings to check thier contents for numbers
fn _matcher(slice: &str, start: usize, end: usize) -> Result<u8, Error> {
    match slice.get(start..end) {
        Some(potential_time) => match potential_time.parse() {
            Ok(num) => Ok(num),
            Err(_) => Err(Error::NotNumber),
        },
        None => Err(Error::BadSlice),
    }
}

fn make_combinations(time: &Time) -> (Vec<u8>, f32) {
    let mut min_time = time.base_time();
    let mut number_combo = time.digits.clone();

    // previous difference between the original total seconds & new total seconds
    let mut previous_difference = 0;

    for i in time.bounds.lower..time.bounds.upper {
        let digits = digits_to_compose(i);

        let combo_time = combination_time(&digits);

        if combo_time <= min_time
            && allow_update(time.bounds.total_sec, i, &mut previous_difference)
        {
            min_time = combo_time;
            number_combo = digits;
        }
    }

    (number_combo, min_time)
}

fn allow_update(base_time: u16, candidate_time: u16, previous_difference: &mut u16) -> bool {
    let new_diff = ((base_time as i16) - (candidate_time as i16)).abs() as u16;

    if new_diff < *previous_difference || *previous_difference == 0 {
        *previous_difference = new_diff;

        true
    } else {
        false
    }
}

fn digits_to_compose(total_seconds: u16) -> Vec<u8> {
    let chars_to_vec = |x: String| {
        x.chars()
            .map(|x| x.to_string().parse().expect("this err should not happen"))
            .collect::<Vec<_>>()
    };

    // if less than 90 seconds you can input 90 seconds and the microwave understands that
    // it is 1minute ... seconds
    if total_seconds < 90 {
        return chars_to_vec(total_seconds.to_string());
    }

    let minutes = total_seconds / 60;
    let seconds = total_seconds - (60 * minutes);

    // make min an empty string if its zero instead of `0`
    let min = if minutes == 0 {
        "".to_string()
    } else {
        minutes.to_string()
    };
    // add a zero for single digit seconds
    // 1 min + 5 seconds -> 1:05
    let sec = if seconds > 9 {
        seconds.to_string()
    } else {
        "0".to_owned() + &seconds.to_string()
    };

    let mut min = chars_to_vec(min);
    let mut sec = chars_to_vec(sec);

    min.append(&mut sec);
    min
}

use clap::{load_yaml, App};
fn main() {
    let yaml = load_yaml! {r"..\command_line.yml"};
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(timer_value) = matches.value_of("INPUT") {
        let time = match Time::new_str(timer_value) {
            Ok(timer) => timer,
            Err(err) => {
                err.print_err();
                panic! {""}
            }
        };

        let (key_combo, time_to_execute) = make_combinations(&time);

        // verbose output
        if matches.is_present("verbose") {
            print! {"most effective combination is "}
            for i in 0..key_combo.len() {
                print! {"{}", key_combo[i]}
                if i != key_combo.len() - 1 {
                    print! {"-"}
                }
            }
            print! {" with an estimated runtime of {} seconds", time_to_execute}
        }
        // non verbose output
        else {
            let key_combo = key_combo.iter().map(|x| x.to_string()).collect::<String>();

            print! {"combo: {} runtime est: {}", key_combo, time_to_execute}
        }

        // optional statistics
        if matches.is_present("stats") {
            let total_time = ((time.min as u16), (time.sec as u16)).total_seconds();
            let microwave_time = key_combo.total_seconds();

            let percent = 100. * (total_time as f32 - microwave_time as f32) / (total_time as f32);

            print! {"\noriginal combination_time: {}", time.base_time()}
            print! {"\ntime saved: {}", time.base_time() - time_to_execute}
            print! {"\npercent error: {} ", percent.abs()}
        }
    }
}
