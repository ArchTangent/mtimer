# Minimal Timer (mtimer)

Minimal Timer (mtimer) with data-driven schedules for Windows and Linux.  Has optional data-driven scheduling via external 'timer plan' text files.  Each timer instance is a simple window with no GUI.  Useful for:

- Interval timers:  warmup, on/off intervals, cooldown
- Repeating schedules:  50 minuts of work, 10 minutes of break
- Reminders: get up and exercise every 30 minutes

Also provides a command-line interface for simple timers that play a sound after the specified time (e.g. 30 seconds).

## Usage

Create a timer using the `time` or `plan` subcommands:

- `mtimer time 30 `: simple 30-second timer
- `mtimer time 10 -v 50 `: simple 10-second timer at 50% volume
- `mtimer time 15 -c `: simple 15-second timer with countdown
- `mtimer plan workout_5 `: play the timer plan in "{EXE_PATH}/plans/workout_5.txt"

Options:

```
--volume    -v :  sets volume for the timer, from 0 to 200% (default 50)
```

Flags:

```text
--countdown -c :  adds a countdown sound at 10, 5, 4, 3, 2, and 1 second remaining
```

## File Paths

- `sound/`: contains all sound files in `.wav/.flac/.vorbis` format
- `plans/`: contains all timer plans in `.txt` format

Default sounds (for the `new` subcommand) are stored in the `sound/default` folder.

## Timer Plan Format

A list of `sound_file: time_delay` pairs, separated by a colon, inside of a `.txt` file.
Comments are expressed with a `#` at the start of a line (no in-line comment allowed).
Blank lines, like comments, are ignored by the parser.  If `time_delay` is not
provided, it defaults to 1 second. An example `interval.txt` timer plan:

```text
warmup.wav: 30
# 1st Loop
interval_on.wav: 10
interval_off.wav: 20
# 2nd Loop
interval_on.wav: 10
interval_off.wav: 20
# End of Workout
cooldown.wav: 30
end_of_workout.wav
```

The above schedule will play a warmup sound, then wait 30 seconds until the interval
(on) begins.  After 10 seconds, the interval (off) will play.  Repeat for the 2nd loop.
The schedule ends with a 30 second cooldown and an end of workout sound.