### The Ceres Search

The possible arrangements in the 4 by 4 boxes are

| x | x | x | x |
|---|---|---|---|
| x | x |   |   |
| x |   | x |   |
| x |   |   | x |

scan across the entire table size

Find an efficient way of fetching the data by reading 4 lines and then processing it.
Then read one more line. Pop the top line.

Then add to the count every time a function returns a success

Process the 4x4 square at a time.
