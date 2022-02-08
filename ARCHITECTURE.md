# Architecture

## Engine

### Rendering pipeline

#### Coordinate systems

- Local space coordinates
  - Relative to the objects origin/center
  - Also known as "object space coordinates"
- World space coordinates
  - Relative to the worlds origin/center
- View space coordinates
  - Also known as "eye space coordinates"
- Clip space coordinates
- Normalized device coordinates (NDC)
  - Screen independent display coordinates
  - Clip-space coordinates `clip.x` and `clip.y` divided by `clip.w` (perspective division)
- Screen space coordinates
  - Rasterized normalized space coordinates
  - Individual components range from `-1.0` to `1.0`
  - Coordinates outside this range will be clipped and therefore not visible
  - Also known as "window space coordinates"

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
