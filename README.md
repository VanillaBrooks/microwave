# microwave
optimizing time to enter microwave cooking timers

## Usage

This is a command line utility for finding the fastest way to input a time into a microwave pannel based on an acceptable tolerance. For example, In order to cook food for one minute you would input `1:00` whereas you could otherwise input `:55`, `:65`, or `:66`. 

The optimal numbers to input can be calculated with `microwave`:

`microwave 1:00`

> combo: 55 runtime est: 0.2

### Tolerance

An input `x`  is a candidate if it is within `x +/- (3 + 0.05*x)`. This means plus or minus 5% of the total numebr of seconds, as well as a flat 3 seconds no matter what.

Percentage differences between total cooktime of the input value and approximation output value can be found with `--stats` : 


`microwave 0:20 -s`

> combo: 22 runtime est: 0.2 percent error: 10
 

### Estimations

Estimations are based on real-world data with n ~= 100;


## Building

`git clone https:://github.com/vanillabrooks/microwave && cd microwave`

`cargo b --release`

Use through cargo:

`cargo r --release 1:38`

> combo: 144 runtime est: 0.70000005

Use the binary:

`cd target/release`

`microwave 0:45`

> combo: 44 runtime est: 0.2

## Releases

Windows 64-bit binaries are available [here](https://github.com/vanillabrooks/microwave/releases)
