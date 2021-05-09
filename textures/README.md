| subject             | width/height | formula                 | additional info                                    |
| ------------------- | ------------:| ----------------------- | -------------------------------------------------- |
| board bmp/texture   | 2048px       |                         |                                                    |
| border              | 12px         |                         |                                                    |
| tile                | 253px        | (2048px - 2 * 12px) / 8 | 2048px = board, 12px = border, 8 = number of tiles |
| pieces bmp/texture  | 1024px       | 4 * 253px + 12px        | 4 = number of pieces, 253px = tile, 12px = padding |
