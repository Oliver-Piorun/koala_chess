# Architecture

## Engine

TODO

## Board

### Regular board layout (point of view: white pieces)

<!-- language: lang-none -->
    |f1|..|..|..|..|..|..|f8|
    |..|..|..|..|..|..|..|..|
    |..|..|..|..|..|..|..|..|
    |..|..|..|..|..|..|..|..|
    |..|..|..|..|..|..|..|..|
    |..|..|..|..|..|..|..|..|
    |..|..|..|..|..|..|..|..|
    |a1|..|..|..|..|..|..|a8|

### Internal board layout (point of view: white pieces)

<!-- language: lang-none -->
    |70|..|..|..|..|..|..|77|
    |..|..|..|..|..|..|..|..|
    |..|..|..|..|..|..|..|..|
    |..|..|..|..|..|..|..|..|
    |..|..|..|..|..|..|..|..|
    |..|..|..|..|..|..|..|..|
    |..|..|..|..|..|..|..|..|
    |00|..|..|..|..|..|..|07|

The internal board layout is the same as the regular board layout.

### Internal board layout (point of view: black pieces)

<!-- language: lang-none -->
    |07|..|..|..|..|..|..|00|
    |..|..|..|..|..|..|..|..|
    |..|..|..|..|..|..|..|..|
    |..|..|..|..|..|..|..|..|
    |..|..|..|..|..|..|..|..|
    |..|..|..|..|..|..|..|..|
    |..|..|..|..|..|..|..|..|
    |77|..|..|..|..|..|..|70|

So when switching the point of view to the black pieces, the position of the pieces on the board remains the same.
The board and pieces are just being rotated by 180° (clock-wise).

## Pieces

TODO