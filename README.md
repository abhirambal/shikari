# shikari
Problem-solving tracker using SQLite in Rust

## Overview

shikari helps you monitor your progress through DSA problems by:
- Tracking problems attempted, solutions, time spent, and difficulty levels
- Storing everything locally in a SQLite database for privacy and portability
- Providing insights into your strengths and improvement areas (TBD)
- Generating statistics on your problem-solving journey (TBD)

## Usage
```
Usage: shikari [OPTIONS] <COMMAND>

Commands:
  add            Add a new problem
  show           Show a specific problem by ID
  list           List all problems
  review         List problems that need review
  by-category    List problems by category
  by-pattern     List problems by pattern
  by-difficulty  List problems by difficulty
  search         Search problems by keyword
  update-time    Update a problem's solve time
  toggle-review  Toggle a problem's review flag
  delete         Delete a problem
  help           Print this message or the help of the given subcommand(s)

Options:
  -d, --database <DATABASE>  Path to the SQLite database file [default: problems.db]
  -h, --help                 Print help
  -V, --version              Print version
```

## Examples
### Add a new problem
```
> ./target/debug/shikari add --help
Add a new problem

Usage: shikari add [OPTIONS] <DESCRIPTION>

Arguments:
  <DESCRIPTION>  Problem description

Options:
  -l, --link <LINK>              Problem link
  -C, --category <CATEGORY>      Problem category
  -p, --pattern <PATTERN>        Problem pattern
  -d, --difficulty <DIFFICULTY>  Problem difficulty
  -t, --time <TIME>              Time to solve (first attempt) in minutes
  -c, --comments <COMMENTS>      Comments about the problem
  -r, --review                   Should solve again
  -h, --help                     Print help
  -V, --version                  Print version
```

> ./target/debug/shikari add "Buy and sell stock II" --link "https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/" --category "Arrays" --pattern "greedy" --difficulty "medium" --time 30 --comments "track the min to max increase greedily"

### List problems you have solved

> ./target/debug/shikari list
All Problems (7)

Problem #1: Majority Element (Easy) - Category: Arrays - Pattern: logic
  Link: https://leetcode.com/problems/majority-element/
  Solve times: 20min, -, -
  Comments: boyer-moore algorithm
  [REVIEW NEEDED]

Problem #2: best-time-to-buy-and-sell-stock (Easy) - Category: Arrays - Pattern: logic
  Link: https://leetcode.com/problems/best-time-to-buy-and-sell-stock/editorial/
  Solve times: 20min, -, -
  Comments: you need to buy low, sell high. two pointer apporach to calculate max profit and move the buy to the sell when you hit negative profit. this means you have seen a low buy
  [REVIEW NEEDED]

...

Problem #6: product except itself (medium) - Category: Arrays - Pattern: prefix sum
  Link: https://leetcode.com/problems/product-of-array-except-self/
  Solve times: 30min, 10min, -
  Comments: brute force is easy, optimize by calculating prefix and suffix products. space saving trick is to use same result array to store prefix in one pass, then do a suffix from reverse in the second pass

Problem #7: Buy and sell stock II (medium) - Category: Arrays - Pattern: greedy
  Link: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/
  Solve times: 30min, -, -
  Comments: track the min to max increase greedily

### Update time
```
./target/debug/shikari update-time --help
Update a problem's solve time

Usage: shikari update-time <ID> <ATTEMPT> <MINUTES>

Arguments:
  <ID>       Problem ID
  <ATTEMPT>  Attempt number (1, 2, or 3)
  <MINUTES>  Time to solve in minutes
```
./target/debug/shikari update-time 6 2 10

