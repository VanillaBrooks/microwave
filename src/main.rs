// |y axis
// |
// |  1   2   3
// |  4   5   6
// |  7   8   9
// |      0
// |------------------------- x axis
// ^
// | (0,0)

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
                .unwrap()
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

        // if the first character is zero (for minutes) we dont need to respect it
        if previous != 0{
            running_time += time_to_move(previous, current);
        }

        for i in 2..data.len() {

            let previous = current;
            let current = data[i];
            running_time += time_to_move(previous, current);
        
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

// 3 seconds + 5 % total seconds
const PERCENT_ALLOWANCE: f32 = 0.05;
const BASE_ALLOWANCE: u16 = 3;

// Time to move one number up / down
const ONE_UNIT_MOVE_TIME: f32 = 0.2;

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
    digits: Vec<u8>
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
        let seconds = seconds_sep.get(1..).expect("was not UTF8 text");

        let min = minutes.parse().expect("ouabsd");
        let sec = seconds.parse().expect("aosbd");

        let total_secs = (min as u16, sec as u16).total_seconds();
        let digits = digits_to_compose(total_secs);

        let x = Self {
            min: min,
            sec: sec,
            bounds: Bounds::from_total_secs(total_secs),
            digits: digits
        };

        Ok(x)
    }

    fn base_time(&self) -> f32{
        combination_time(&self.digits)
    }
}

#[derive(Clone, Debug)]
struct Bounds {
    upper: u16,
    lower: u16,
}
impl Bounds {
    fn new(minutes: u8, seconds: u8) -> Self {
        let total_secs = (minutes as u16, seconds as u16).total_seconds();

        Self::from_total_secs(total_secs)
    }
    fn from_total_secs(total_secs: u16) -> Self {
        let diff = ((total_secs as f32) * PERCENT_ALLOWANCE).ceil() as u16;
        let total_diff = BASE_ALLOWANCE + diff;

        let upper = total_secs + total_diff;
        let lower = total_secs - total_diff;

        Self {
            upper: upper,
            lower: lower,
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

fn make_combinations(time: Time) -> (Vec<u8>, f32) {
    let mut min_time = time.base_time();
    let mut number_combo = time.digits;

    for i in time.bounds.lower..time.bounds.upper {
        dbg!{i};

        let digits = digits_to_compose(i);

        dbg!{&digits};

        let combo_time = combination_time(&digits);
        dbg!{combo_time};

        if combo_time < min_time {
            min_time = combo_time;
            number_combo = digits;
        }

    }

    (number_combo, min_time)
}

fn digits_to_compose(total_seconds: u16) -> Vec<u8> {

    let chars_to_vec = |x: String| x.chars().map(|x| x.to_string().parse().unwrap()).collect::<Vec<_>>();

    // if less than 90 seconds you can input 90 seconds and the microwave understands that
    // it is 1minute ... seconds
    if total_seconds < 90{
        return chars_to_vec(total_seconds.to_string())
    }
    
    let minutes = total_seconds / 60;
    let seconds = total_seconds - (60 * minutes);
 
    let min = minutes.to_string();
    // add a zero for single digit seconds
    // 1 min + 5 seconds -> 1:05
    let sec = 
        if seconds > 9 {
            seconds.to_string()
        }
        else{
            "0".to_owned() + &seconds.to_string()
        };

    let mut min = chars_to_vec(min);
    let mut sec = chars_to_vec(sec);
    
    min.append(&mut sec);
    min
}

fn main() {
    let mut time = Time::new_str("5:30").unwrap();

    let combo = make_combinations(time);
    dbg!{"result: \n\n\n"};
    dbg!{combo};
}
