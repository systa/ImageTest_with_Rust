This folder has some example files.  Original files are car.jpg and TRE.jpg. File filename of others document the used command-line parameters.
For instance, a command

    cargo run -- car.jpg 9 20

has produced file car.jpgP-9-20.jpg. This means that there where 9*9*9 initial colors from which 12 most popular were yusesed.

Similarly, command

   cargo run -- TRE.jpg 7 12 50

has produced file car.jpgP-7-12-50.jpg.  The last parameter tells that 50 random changes were applied to each resulting (20) colors on each step of simultaneous annealing.  This image is a good example of an image that benefits from simultaneous annealing.



