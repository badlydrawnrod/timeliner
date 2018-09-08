# timeliner
timeliner is a simple command-line tool to display the contents of multiple log files in timestamp order.
The log files can have different encodings, and different timestamp formats.

Dual-licensed under MIT or the [UNLICENSE](http://unlicense.org).

## Installation
timeliner is a Rust program. The easiest way to install it is to get cargo, then run the following:
```
$ cargo install timeliner
```


## Example
Display the contents of some logs with different encodings and interleaved timestamps.
```console
$ timeliner samples/ascii.txt samples/utf16bebom.txt samples/utf16lebom.txt samples/utf8.txt
samples\utf8.txt: 2018-01-27 22:11:20 UTC       -- Opening UTF8 log --
samples\utf16lebom.txt: 2018-01-27 22:11:20.100 UTC     -- Opening UTFLE log --
samples\utf16bebom.txt: 2018-01-27 22:11:20.200 UTC     -- Opening UTFBE log --
samples\ascii.txt: 2018-01-27 22:11:20.300 UTC  -- Opening ASCII log --
samples\utf8.txt: 2018-01-27 22:11:21 UTC       UTF8 Log Program version 3.74
samples\utf16lebom.txt: 2018-01-27 22:11:21.100 UTC     UTFLE Log Program version 3.74
samples\utf16bebom.txt: 2018-01-27 22:11:21.200 UTC     UTFBE Log Program version 3.74
samples\ascii.txt: 2018-01-27 22:11:21.300 UTC  ASCII Log Program version 3.74
samples\utf8.txt: 2018-01-27 22:11:22 UTC       Customer ID: 524f4421
samples\utf16lebom.txt: 2018-01-27 22:11:22.100 UTC     Customer ID: 524f4421
samples\utf16bebom.txt: 2018-01-27 22:11:22.200 UTC     Customer ID: 524f4421
samples\ascii.txt: 2018-01-27 22:11:22.300 UTC  Customer ID: 524f4421
samples\utf8.txt: 2018-01-27 22:11:23 UTC       Customer URL: http://www.example.com
samples\utf16lebom.txt: 2018-01-27 22:11:23.100 UTC     Customer URL: http://www.example.com
samples\utf16bebom.txt: 2018-01-27 22:11:23.200 UTC     Customer URL: http://www.example.com
samples\ascii.txt: 2018-01-27 22:11:23.300 UTC  Customer URL: http://www.example.com
samples\utf8.txt: 2018-01-27 22:11:24 UTC       -- Closing UTF8 log --
samples\utf16lebom.txt: 2018-01-27 22:11:24.100 UTC     -- Closing UTF16LE log --
samples\utf16bebom.txt: 2018-01-27 22:11:24.200 UTC     -- Closing UTF16BE log --
samples\ascii.txt: 2018-01-27 22:11:24.300 UTC  -- Closing ASCII log --
```
