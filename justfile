# Throughout the file, you'll see a lot of this:
#
# {{ if day =~ '^\d{1}$' { "0" + day } else { day } }}
#
# All of these instances could be replaced with {{day}}. That said, I kept forgetting to left-pad the single digit
# days, and this solves that. The advantage of doing this is that just will actually calculate the day before printing
# it. I am not currently aware of a way in just to separate that into its own function, so duplicated it will remain.

[private]
default: help

# Output the available recipes and their descriptions
help:
    @just --list

# Scaffold the day and download its puzzle and input
create day:
    just scaffold {{ if day =~ '^\d{1}$' { "0" + day } else { day } }}
    just download {{ if day =~ '^\d{1}$' { "0" + day } else { day } }}

# Create a new folder for the specified day with the preconfigured template
scaffold day:
    cargo generate --path ./template --name day-{{ if day =~ '^\d{1}$' { "0" + day } else { day } }}

# Download the specified day's puzzle description and input
download day:
    aoc download --year 2023 --day {{ if day =~ '^\d{1}$' { "0" + day } else { day } }} --input-file day-{{ if day =~ '^\d{1}$' { "0" + day } else { day } }}/data/input.txt --puzzle-file day-{{ if day =~ '^\d{1}$' { "0" + day } else { day } }}/data/puzzle.md --overwrite

# Output the specified day's puzzle
read day:
    aoc read --year 2023 --day {{ if day =~ '^\d{1}$' { "0" + day } else { day } }}

# Show the Advent of Code calendar and stars collected
calendar:
    aoc calendar

# Solve the given day and part
solve day part:
    cargo run -p day-{{ if day =~ '^\d{1}$' { "0" + day } else { day } }} --bin part{{part}} --release

# Submit an answer for the given day and part
submit day part answer:
    aoc submit --year 2023 --day {{ if day =~ '^\d{1}$' { "0" + day } else { day } }} {{part}} {{answer}}

# Run the tests for a given day and part (part is optional)
test day part=" ":
    cargo nextest run -p day-{{ if day =~ '^\d{1}$' { "0" + day } else { day } }} part{{part}} --nocapture

# Lint the code for a given day
lint day:
    cargo clippy -p day-{{ if day =~ '^\d{1}$' { "0" + day } else { day } }}

# Format the code for a given day
format day:
    cargo fmt -p day-{{ if day =~ '^\d{1}$' { "0" + day } else { day } }}

# Run the benchmarks and puts them in a file corresponding to the given day (part is optional)
bench day part=" ":
    cargo bench -q --bench day-{{ if day =~ '^\d{1}$' { "0" + day } else { day } }}-bench part{{part}} 2> /dev/null >> day-{{ if day =~ '^\d{1}$' { "0" + day } else { day } }}.bench.txt

# Same as bench, but verbose. Logs to stderr/stdout as well as saving to a file
bench-v day part=" ":
    cargo bench --bench day-{{ if day =~ '^\d{1}$' { "0" + day } else { day } }}-bench part{{part}} | tee -a day-{{ if day =~ '^\d{1}$' { "0" + day } else { day } }}.bench.txt


# Run the criterion benchmarks for the given day and part (part is optional)
bench-criterion day part=" ":
    cargo bench --bench day-{{ if day =~ '^\d{1}$' { "0" + day } else { day } }}-bench-criterion part{{part}} | tee -a day-{{ if day =~ '^\d{1}$' { "0" + day } else { day } }}.bench-criterion.txt

# Run all the benchmarks
bench-all:
    cargo bench --quiet | rg --multiline "(?s)day.*?part2.*?\n\n" | tee benchmarks.txt

# Watch the files for a given day, linting, testing, and benchmarking as you go
work day part=" ":
    cargo watch -w day-{{ if day =~ '^\d{1}$' { "0" + day } else { day } }} -x "check -p day-{{ if day =~ '^\d{1}$' { "0" + day } else { day } }}" -s "just test {{ if day =~ '^\d{1}$' { "0" + day } else { day } }} {{part}}" -s "just lint {{ if day =~ '^\d{1}$' { "0" + day } else { day } }}" -s "just bench {{ if day =~ '^\d{1}$' { "0" + day } else { day } }} {{part}}"

# Same as work, but verbose. Uses the verbose bench recipe
work-v day part=" ":
    cargo watch -w day-{{ if day =~ '^\d{1}$' { "0" + day } else { day } }} -x "check -p day-{{ if day =~ '^\d{1}$' { "0" + day } else { day } }}" -s "just test {{ if day =~ '^\d{1}$' { "0" + day } else { day } }} {{part}}" -s "just lint {{ if day =~ '^\d{1}$' { "0" + day } else { day } }}" -s "just bench-v {{ if day =~ '^\d{1}$' { "0" + day } else { day } }} {{part}}"
