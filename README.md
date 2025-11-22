# R&S Loot Generator Simulator CLI (CSV)
Small hobby project to practice coding skills and work with libraries.

The program (on my specs) now creates a 100,000 entries in 1.446s.

I have no plans to release this as an executable, but I'll provide
an examples folder with these files if that is all you care for.

## Roadmap
I consider this program in a good state, I have potential items
to do if I rework this code again:
- [x] Proper unit testing
  - [x] Matches what other users have statistically
- [x] Check hashtables match 1:1 to the game (1.4.x)
- [x] Multithreading 
- [ ] Include a working flag to use only indices for Treasurespheres and Items.
- [ ] Update to game patch 1.5.x items on release
- [ ] Add gems and items encountered in shops
- [ ] Add encountered biomes
- [ ] Test on Windows

## Description
Multithreading CLI program that prints or generates a `.csv` file simulating
the loot you'd find over a number of runs for the game *Rabbit and Steel*.

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

### CSV file format
Currently the `.csv` file is limited to the following fields:
- player_count: number of players
- ts_{0..=5}: the found Treasuresphere color
- it_{0..5}_{0..4}: the item found by their Treasuresphere and index in that Treasuresphere.

``` csv
player_count,ts_0,...,ts_5,it_0_0,it_0_1,it_0_2,...,it_5_2,it_5_3,it_5_4
4,opal,...,normal,it_blackwing_staff,it_lullaby_harp,it_twinstar_earrings,...,it_divine_mirror,it_vega_spear,it_mountain_staff
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
