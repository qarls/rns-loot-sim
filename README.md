# Loot Generator Simulator CLI in CSV for Vanilla Rabbit & Steel Game Runs
Small hobby project to practice coding skills and work with libraries.

I have no plans to release this as an executable, but I'll provide
an examples folder with these files if that is all you care for.

I consider this still __unfinished__ due to:
- Requires unit testing
- Have not checked if my hashtables match 1:1 to the game
- Patch 1.5.x is coming out soon (with new items)
- No gem shops or items included
- Not tested on Windows

## Description
Small CLI program that prints or generates a `.csv` file simulating
the loot you'd find over a number of runs.

Currently, there's a coded limit of 200,000 entries (85MB).
```
rns-loot-sim [options] 
```

### Options
```
  -n, --run-count <RUN_COUNT>        Number of game runs (samples) [default: 1]
  -p, --player-count <PLAYER_COUNT>  Player count [default: 1]
  -o, --output-file <OUTPUT_FILE>    Output file (csv), if not used, print to stdout
  -s, --seed <SEED>                  Use a positive interger (u64) seed for RNG (non-compliant)
  -h, --help                         Print help
  -V, --version                      Print version
```

## Disclaimer
I am not affiliated, associated, authorized, endorsed by, or in any way
officially connected with the roguelike game *Rabbit & Steel*, `mino_dev`, or
any of its subsidiaries or its affiliates.

If you'd like to check their website or the game, you can find them here:
- [mino_dev games](https://www.minodevgames.com/)
- [Rabbit & Steel Steam Page](https://store.steampowered.com/app/2132850/Rabbit_and_Steel/)

### Small Note
Much thanks to `mino_dev` and community for the fun game. I suspect some players
will instantly recognize who I am there.
