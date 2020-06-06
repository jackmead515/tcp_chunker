# tcp_chunker

Just me playing around with Rust's low level TCP networking.

tcp_chunker is supposed to be a lightweight and lazy TCP streaming protocol
for large files that don't have any need of being transfered immediately 
(or even in the next hour).

Basically:

```
// my file
[1, 2, 3, 4, 5]

// client side chunks data
[[1], [2], [3], [4], [5]]

// client sends one chunk, along with info about the chunks
// data, length of data, total chunks
[1], 1, 5 -> server

// server ingests data and returns a uuid for the matching file
uuid -> client

// client can now send any chunk in any order to the server
// whenever it feels like it! As long as server doesn't time
// out the request.
```