# outline

life game rule

| action | description |
| :----- | :----- |
| underpopulation | Any live cell with fewer than two live neighbours dies |
| survive | Any live cell with two or three live neighbours |
| overpopulation | Any live cell with more than three live neighbours dies |
| reproduction | Any dead cell with exactly three live neighbours becomes a live cell |

# build and run

```shell
cargo build
cd ./target/debug
./life_game --generation 100
```
