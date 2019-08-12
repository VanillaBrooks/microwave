

//  
//  1   2   3
//  4   5   6
//  7   8   9
//      0
//

trait TotalSeconds{
    fn total_seconds(&self) -> u16; 
}

impl TotalSeconds for (u16, u16){
    fn total_seconds(&self) -> u16{
        let (min,sec) = self;
        let total_secs = ((*min as u16) * 60_u16) + *sec as u16;
        
        total_secs
    } 
}

impl TotalSeconds for Vec<u8> {
    fn total_seconds(&self) -> u16{

        let make_str = |x: &[u8]| x.iter().map(|x| x.to_string()).collect::<String>().parse().unwrap();
        
        let (min, sec) = 
        if self.len() > 2{

            let (mins, secs) = self.split_at(self.len() - 2);

            let mins : u16= make_str(mins);
            let secs : u16= make_str(secs);

            (mins, secs)
        }
        else if self.len() == 2{
            let mins = 0;
            let secs = make_str(self.as_slice());

            (mins, secs)


        }
        else if self.len() == 1 {
            let secs = "0".to_owned() + &self[0].to_string();

            (0, secs.parse().expect("secs not parsed"))

        }
        else{
            (0,0)
        };

        (min, sec).total_seconds()

    }
}


// 3 seconds + 5 % total seconds
const PERCENT_ALLOWANCE: f32 = 0.05;
const BASE_ALLOWANCE: u16= 3;

// Time to move one number up / down
const ONE_UNIT_MOVE_TIME: f32 = 0.2;

#[derive(Copy, Clone)]
struct Location {
    x: u8,
    y: u8
}
impl Location {
    fn new(x: u8, y: u8) -> Self {
        Self{x: x, y: y}
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
        Self{
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

#[derive(Clone, Debug)]
struct Time{
    min: u8,
    sec: u8,
    bounds: Bounds
}
impl Time {
    fn new(min: u8, sec: u8) -> Self{
        let total_secs = ((min as u16) * 60_u16) + sec as u16;
        Self{
            min: min,
            sec: sec,
            bounds: Bounds::new(total_secs)
        }
    }

    fn bounds_check(&self, nums: &Vec<u8>) -> BoundType{
        let total_secs = nums.total_seconds();

        return self.bounds.inside(&total_secs);
    } 
}

enum BoundType{
    Above,
    Below,
    Inside
}

#[derive(Debug, Clone)]
struct Bounds {
    upper: u16,
    lower: u16
}
impl Bounds {
    fn new(total_seconds: u16) -> Self{
        let diff = ((total_seconds as f32) * PERCENT_ALLOWANCE).ceil() as u16;
        let total_diff = BASE_ALLOWANCE + diff;

        let upper = total_seconds + total_diff;
        let lower = total_seconds - total_diff;

        Self{
            upper:upper,
            lower:lower
        }
    }

    fn inside(&self, seconds_calc: &u16) -> BoundType{
        if *seconds_calc <= self.upper && *seconds_calc >= self.lower{
            BoundType::Inside
        } 
        else if *seconds_calc <= self.lower {
            BoundType::Below
        }
        else if *seconds_calc >= self.upper {
            BoundType::Above
        }
        else{
            BoundType::Inside
        }
    }
}


#[derive(Debug)]
enum Error{
    MissingChar,    // missing ":" or "." in the string
    NotNumber,      // the sides of the string separated by ":" "." dont have a number
    BadSlice,       // not utf8 string we can slice (also NotNumber)
    InvalidSeconds  // length of the number of seconds passed in was greater than 2
}   


/// Helper function for slicing strings to check thier contents for numbers
fn _matcher(slice: &str, start: usize, end: usize) -> Result<u8, Error> {
    match slice.get(start..end){
        Some(potential_time) => 
            match potential_time.parse() {
                Ok(num) => Ok(num),
                Err(_) => Err(Error::NotNumber)
            },
        None => Err(Error::BadSlice)
    }
}

// Build a Time struct from a string
fn parse_time(arg: &str) -> Result<Time, Error> {
    let split_index = 
        if let Some(index) = arg.find(":"){index}
        else if let Some(index) = arg.find("."){index}
        else{return Err(Error::MissingChar)};

    if arg.len() - (split_index+1) > 2 {return Err(Error::InvalidSeconds)}

    let minutes = _matcher(&arg, 0, split_index)?;
    let seconds = _matcher(&arg, split_index + 1, arg.len())?;

    Ok(Time::new(minutes, seconds))
}

fn make_combinations(time: Time) -> () {
    let mut combinations : Vec<Vec<u8>> = Vec::with_capacity(100);

    loop {
        let mut curr_nums : Vec<u8>= Vec::with_capacity(4);

        match time.bounds_check(&curr_nums){
            BoundType::Above => continue,
            BoundType::Below => (),
            BoundType::Inside=> ()
        }


    }

} 

fn main() {

    let time = parse_time("134:672");

    let seconds : u8 = 12;

    let shift = seconds >> 1;


    dbg!{&shift};


}
