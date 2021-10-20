## Advent of Code 2020

### Organization

Project contains multiple packages in a cargo workspace.
- The [`advent_2020`](/tree/main/advent_2020) package contains Advent of Code problem solutions
- The [`parser`](/tree/main/parser) package contains a simple parser combinator library that seemed
  rather relevant. It's a port of the F# example found on [fsharpforfunandprofit.com](https://fsharpforfunandprofit.com/series/understanding-parser-combinators/).
  For any serious work you should probably consult one of the parser combinator crates on crates.io.

Notes:
- Day 13 pt 2: Particularly unpleasant to discover that I apparently should have a priori knowledge
  of a mathematical theorem I've never had any reason to encounter.
- Day 17 pt 1: The examples in which the "frame of view follows the active cells in each cycle" are
  completely unreadable to me. I couldn't write the tests I wanted and spent a number of extra
  minutes scratching my head because one specific unit test was failing, all the while not noticing
  that my test of the overall solution was already passing.
