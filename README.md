# tetris

## Ideas for interworkings

- Grid will hold refrences to Cell's
- To rotate or move a tetromino, you call a rotate or move method on the tetromino itself
- The rotate and move methods on tetrominos will take a refrence to the Playfield to check if it is possible to move or rotate in the given direction