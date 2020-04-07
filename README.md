# Life

![Sample image](https://github.com/brundonsmith/life/raw/master/sample.png)

This is an implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) 
that I wrote in Rust purely for fun.

It uses [Piston](https://www.piston.rs/) to create a window and draw the world
state to it.

The simulation can be parameterized with the initial configurations found under
`src/configurations/`. Each is a text file describing a grid where dots (`.`) 
are "dead" cells and stars (`*`) are "alive" cells. The grid can be any size and
everything outside of it is assumed to be dead. The format was adopted from the
[Life Lexicon](https://conwaylife.com/ref/lexicon/lex_home.htm), and the 
configurations in this repository were taken directly from the ASCII version of 
that document. Importing new ones is easy, but there are far too many for me to 
have bothered to do all of them.

## Running the program

Assuming you have `cargo` insalled, just execute `cargo run <CONFIG>` where
`<CONFIG>` is the name of one of the configurations under `src/configurations/`
(without `.txt`). For example, to run `glider.txt`, type:

```
cargo run glider
```

## Implementation notes

At first glance Life seems very straightforward to implement, but there are two
interesting wrinkles:
1. Each new world state depends on the previous state, in a way that can't be 
localized completely, so you can't just overwrite state in-place
2. Ideally the board is "infinite"; the "creatures" can progress infinitely in 
any direction, so it's best if they don't hit a wall

I solved the first problem with a dual-buffer system. The next board state is 
construted based off of the current one, in an independent data structure, and 
then the two are "flipped" and the new one becomes the "current" one, their
roles reversed in the next cycle.

The second problem was tricker. The most obvious data structure for Life is a
2D array. But that will have walls, and while it could be re-allocated as 
necessary to grow indefinitely, this would get extremely memory-inefficient for,
say, a glider that shoots off in one direction and leaves nothing behind.

What I landed on was using a HashMap whose keys are locations (row/column) and whose values are booleans, indicating aliveness state. This allows the structure
to be incredibly "sparse"; i.e., memory usage is tied to how many cells are 
alive, not where they are or where they've been before. It only needs to 
actually record the ones that are alive right now, and the ones that might 
*become* alive next cycle (direct neighbors of the currently-alive). These keys
of the current HashMap are then iterated over as the only candidates for being 
"possibly alive" in the next cycle.