```
$ ~/trash/rust-algorithms/chapter-1/5/quick[master*]: cargo run < mediumUF.txt
3 node/s in Duration { secs: 0, nanos: 3499000 }
3 node/s in Duration { secs: 0, nanos: 3406000 }
3 node/s in Duration { secs: 0, nanos: 2903000 }

$ ~/trash/rust-algorithms/chapter-1/5/quick[master*]: cargo  run   --release < mediumUF.txt
3 node/s in Duration { secs: 0, nanos: 892000 }
3 node/s in Duration { secs: 0, nanos: 872000 }
3 node/s in Duration { secs: 0, nanos: 861000 }

$ ~/trash/rust-algorithms/chapter-1/5/quick[master*]: cargo run < tinyUF.txt
2 node/s in Duration { secs: 0, nanos: 564000 }
2 node/s in Duration { secs: 0, nanos: 767000 }
2 node/s in Duration { secs: 0, nanos: 574000 }


large file with --release was at
lines:   Time since start
1480000: Duration { secs: 4981, nanos: 410624000 }
1490000: Duration { secs: 5067, nanos: 660044000 }
1500000: Duration { secs: 5153, nanos: 631368000 }
1510000: Duration { secs: 5239, nanos: 594573000 }
1520000: Duration { secs: 5326, nanos: 536835000 }
there was another 500k lines to go. you can see how its growing
```
