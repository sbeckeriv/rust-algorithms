```
$ ~/trash/rust-algorithms/chapter-1/5/quick-compressed[master*]: cargo run < tinyUF.txt
2 node/s in Duration { secs: 0, nanos: 777000 }
2 node/s in Duration { secs: 0, nanos: 539000 }
2 node/s in Duration { secs: 0, nanos: 548000 }
2 node/s in Duration { secs: 0, nanos: 934000 }

$ ~/trash/rust-algorithms/chapter-1/5/quick-compressed[master*]: cargo run  --release < tinyUF.txt
2 node/s in Duration { secs: 0, nanos: 576000 }
2 node/s in Duration { secs: 0, nanos: 826000 }
2 node/s in Duration { secs: 0, nanos: 668000 }

$ ~/trash/rust-algorithms/chapter-1/5/quick-compressed[master*]: cargo run  --release < mediumUF.txt
3 node/s in Duration { secs: 0, nanos: 927000 }
3 node/s in Duration { secs: 0, nanos: 776000 }
3 node/s in Duration { secs: 0, nanos: 986000 }
3 node/s in Duration { secs: 0, nanos: 932000 }

$ ~/trash/rust-algorithms/chapter-1/5/quick-compressed[master*]: cargo run  --release < largeUF.txt
6 node/s in Duration { secs: 1, nanos: 407044000 }
6 node/s in Duration { secs: 1, nanos: 389972000 }
6 node/s in Duration { secs: 1, nanos: 378996000 }
```
