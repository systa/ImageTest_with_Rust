# ImageTest_with_Rust

This simple program was mainly written to learn basics of the Rust programming language.
The initial version evolved as my learned more about Rust and as I got new ideas about
the functionality.  Naturally, this means that I'm not very proud about the results.
 
The purpose of software is to read an image and radically limit the number of used
color shades. The algoritm is based on five steps
   1) a set of candidate colors is defined. See function createRefColors. The number of
      candidate colors is parametrized (n*n*n).
   2) Iterate through all pixels of the image:
       Function updateDistanceData:
        - calculate distance (see the fn "distance") to each candidate colors
        - update counter of the closest color
   3) Take the most popular one from the reference colors. Function popularColors
   4) Iterate through all pixels of the image and replace with the closest in list of popular
      colors.
   5) If the fourh parameter exists and in greater than zero, a kind of simultaneus annealing is run on
      the results from step 4.

See folder examples for example uses.

Notes: 
   - Rust compiler nags me on using "snake_case". I'm not convinced.
   - Selection between integer types (u9, i32, u32) is a bit arbitrary. One goal was to minimize
     the casts with "as"
   - Some data structures are almost static, but because their content depends on
     command-line parameters they are mutable. This lead to several "unsafe blocks"
   - Instead of clean code my interest has been on the effects of the algorithm.


Copyright Kari Systä, 2022-2023.
Thanks to https://lib.rs/crates/image and example for giving me a starting point.
