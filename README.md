# Take A Break

Command-line application written in Rust that allows you to set a timer after which it performs a given operation.

## Usage

With Take A Break you can run commands such as:
<code>take_a_break -s 1h -o hibernate</code>
- '-s' : specify the amount of time after which the action will be performed (e.g. '1m' for 1 minute).
- '-o' : specify the action to perform (possible actions: shutdown, reboot, hibernate, sleep(linux only)).
- '-h' : prints helpful info

## Installation
1. Clone this repository
2. Build using 'cargo build --release'
3. Add the /target/release folder to environment variables PATH to be able to use it in any terminal.
