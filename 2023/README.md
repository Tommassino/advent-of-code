<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2023 🎄

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->
## 2023 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2023/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2023/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2023/day/3) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2023/day/4) | ⭐ | ⭐ |
| [Day 5](https://adventofcode.com/2023/day/5) | ⭐ | ⭐ |
| [Day 6](https://adventofcode.com/2023/day/6) | ⭐ | ⭐ |
| [Day 7](https://adventofcode.com/2023/day/7) | ⭐ | ⭐ |
| [Day 8](https://adventofcode.com/2023/day/8) | ⭐ | ⭐ |
| [Day 9](https://adventofcode.com/2023/day/9) | ⭐ | ⭐ |
| [Day 10](https://adventofcode.com/2023/day/10) | ⭐ | ⭐ |
| [Day 11](https://adventofcode.com/2023/day/11) | ⭐ | ⭐ |
| [Day 12](https://adventofcode.com/2023/day/12) | ⭐ | ⭐ |
| [Day 13](https://adventofcode.com/2023/day/13) | ⭐ | ⭐ |
| [Day 14](https://adventofcode.com/2023/day/14) | ⭐ | ⭐ |
| [Day 15](https://adventofcode.com/2023/day/15) | ⭐ | ⭐ |
| [Day 16](https://adventofcode.com/2023/day/16) | ⭐ | ⭐ |
| [Day 17](https://adventofcode.com/2023/day/17) | ⭐ | ⭐ |
| [Day 18](https://adventofcode.com/2023/day/18) | ⭐ | ⭐ |
| [Day 19](https://adventofcode.com/2023/day/19) | ⭐ | ⭐ |
| [Day 20](https://adventofcode.com/2023/day/20) | ⭐ | ⭐ |
| [Day 21](https://adventofcode.com/2023/day/21) | ⭐ |   |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 |
| :---: | :---: | :---:  |
| [Day 1](./src/bin/01.rs) | `72.8µs` | `374.1µs` |
| [Day 2](./src/bin/02.rs) | `44.7µs` | `47.3µs` |
| [Day 3](./src/bin/03.rs) | `1.2ms` | `1.3ms` |
| [Day 4](./src/bin/04.rs) | `360.1µs` | `357.7µs` |
| [Day 5](./src/bin/05.rs) | `32.6µs` | `58.6µs` |
| [Day 6](./src/bin/06.rs) | `321.0ns` | `892.0ns` |
| [Day 7](./src/bin/07.rs) | `357.3µs` | `360.8µs` |
| [Day 8](./src/bin/08.rs) | `5.9ms` | `16.9ms` |
| [Day 9](./src/bin/09.rs) | `324.5µs` | `349.5µs` |
| [Day 10](./src/bin/10.rs) | `2.6ms` | `115.4ms` |
| [Day 11](./src/bin/11.rs) | `8.1ms` | `7.0ms` |
| [Day 12](./src/bin/12.rs) | `980.4µs` | `6.3ms` |
| [Day 13](./src/bin/13.rs) | `373.1µs` | `394.7µs` |
| [Day 14](./src/bin/14.rs) | `73.5µs` | `54.5ms` |
| [Day 15](./src/bin/15.rs) | `301.8µs` | `712.3µs` |
| [Day 16](./src/bin/16.rs) | `1.4ms` | `55.9ms` |
| [Day 17](./src/bin/17.rs) | `11.7ms` | `32.8ms` |
| [Day 18](./src/bin/18.rs) | `26.9µs` | `70.7µs` |
| [Day 19](./src/bin/19.rs) | `44.1ms` | `42.1ms` |
| [Day 20](./src/bin/20.rs) | `7.2ms` | `101.8ms` |

**Total: 521.87ms**
<!--- benchmarking table --->

---

## Template setup

This template supports all major OS (macOS, Linux, Windows).

## Usage

### Scaffold a day

```sh
# example: `cargo scaffold 1`
cargo scaffold <day>

# output:
# Created module file "src/bin/01.rs"
# Created empty input file "data/inputs/01.txt"
# Created empty example file "data/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
```

Individual solutions live in the `./src/bin/` directory as separate binaries. _Inputs_ and _examples_ live in the the `./data` directory.

Every [solution](https://github.com/fspoettel/advent-of-code-rust/blob/main/src/template/commands/scaffold.rs#L9-L35) has _tests_ referencing its _example_ file in `./data/examples`. Use these tests to develop and debug your solutions against the example input.

> [!TIP]
> If a day has different example inputs for both parts, you can use the `read_file_part()` helper in your tests instead of `read_file()`. For example, if this applies to day 1, you can create a second example file `01-2.txt` and invoke the helper like `let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));` to read it in `test_part_two`.

> [!TIP]
> when editing a solution, `rust-analyzer` will display buttons for running / debugging unit tests above the unit test blocks.

### Download input & description for a day

> [!IMPORTANT] 
> This command requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

```sh
# example: `cargo download 1`
cargo download <day>

# output:
# [INFO  aoc] 🎄 aoc-cli - Advent of Code command-line tool
# [INFO  aoc_client] 🎅 Saved puzzle to 'data/puzzles/01.md'
# [INFO  aoc_client] 🎅 Saved input to 'data/inputs/01.txt'
# ---
# 🎄 Successfully wrote input to "data/inputs/01.txt".
# 🎄 Successfully wrote puzzle to "data/puzzles/01.md".
```

### Run solutions for a day

```sh
# example: `cargo solve 01`
cargo solve <day>

# output:
#     Finished dev [unoptimized + debuginfo] target(s) in 0.13s
#     Running `target/debug/01`
# Part 1: 42 (166.0ns)
# Part 2: 42 (41.0ns)
```

The `solve` command runs your solution against real puzzle inputs. To run an optimized build of your code, append the `--release` flag as with any other rust program.

By default, `solve` executes your code once and shows the execution time. If you append the `--time` flag to the command, the runner will run your code between `10` and `10.000` times (depending on execution time of first execution) and print the average execution time.

For example, running a benchmarked, optimized execution of day 1 would look like `cargo solve 1 --release --time`. Displayed _timings_ show the raw execution time of your solution without overhead like file reads.

#### Submitting solutions

> [!IMPORTANT]
> This command requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

In order to submit part of a solution for checking, append the `--submit <part>` option to the `solve` command.

### Run all solutions

```sh
cargo all

# output:
#     Running `target/release/advent_of_code`
# ----------
# | Day 01 |
# ----------
# Part 1: 42 (19.0ns)
# Part 2: 42 (19.0ns)
# <...other days...>
# Total: 0.20ms
```

This runs all solutions sequentially and prints output to the command-line. Same as for the `solve` command, the `--release` flag runs an optimized build.

#### Update readme benchmarks

The template can output a table with solution times to your readme. In order to generate a benchmarking table, run `cargo all --release --time`. If everything goes well, the command will output "_Successfully updated README with benchmarks._" after the execution finishes and the readme will be updated.

Please note that these are not "scientific" benchmarks, understand them as a fun approximation. 😉 Timings, especially in the microseconds range, might change a bit between invocations.

### Run all tests

```sh
cargo test
```

To run tests for a specific day, append `--bin <day>`, e.g. `cargo test --bin 01`. You can further scope it down to a specific part, e.g. `cargo test --bin 01 part_one`.

### Format code

```sh
cargo fmt
```

### Lint code

```sh
cargo clippy
```

### Read puzzle description in terminal

> [!IMPORTANT]
> This command requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

```sh
# example: `cargo read 1`
cargo read <day>

# output:
# Loaded session cookie from "/Users/<snip>/.adventofcode.session".
# Fetching puzzle for day 1, 2022...
# ...the input...
```

## Optional template features

### Configure aoc-cli integration

1. Install [`aoc-cli`](https://github.com/scarvalhojr/aoc-cli/) via cargo: `cargo install aoc-cli --version 0.12.0`
2. Create an `.adventofcode.session` file in your home directory and paste your session cookie. To retrieve the session cookie, press F12 anywhere on the Advent of Code website to open your browser developer tools. Look in _Cookies_ under the _Application_ or _Storage_ tab, and copy out the `session` cookie value. [^1]

Once installed, you can use the [download command](#download-input--description-for-a-day), the read command, and automatically submit solutions via the [`--submit` flag](#submitting-solutions).
