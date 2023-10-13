# Devlog
A simple CLI which allows you to clock in and out, keeping track of time spent coding in a CSV.

## Usage
```
dlog <COMMAND>
```
Commands:
* `in` - Clocks you in.
* `out` - Clocks you out.
* `time` - Reports the total time spent developing.

Simply clock in before you start work and out before you stop.

## Notes
* This tool is not smart. It tracks time based on the current directory you are in when the command is called. Always call it from the root directory of your project.
* If using this with git, clock out before you commit changes so that your timelog is added to the commit history.

## Installation
Manually install the binary and put it in your PATH.

## Future Work
* Make the program smarter: like git, have an init command that tracks the root directory. Then, as long as you are within a subdirectory, the commands should be able to find the hidden .devlog file and update it.
