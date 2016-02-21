# Implmentation details

## WSPR

http://g4jnt.com/Coding/WSPR_Coding_Process.pdf

- _Data bits_: 50
- _Convoluted bits_: 50+31 * 2 = 81*2 = 162
- _Tranmitted symbols_: 162

Also there's extended wspr, see
http://w7ekb.com/pipermail/600mrg_w7ekb.com/2014-August/003220.html

## JT4

http://g4jnt.com/Coding/JT4_Coding_Process.pdf

- _Data bits_: 72
- _Convoluted bits_: 72+31 * 2 = 103*2 = 206
- _Tranmitted symbols_: 206+1 = 207

First transmitted bit is a zero for sync

## JT9

- _Data bits_: 72
- _Convoluted bits_: 72+31 * 2 = 103*2 = 206
- _Tranmitted symbols_: 206+1/3 + 16 = 69 + 16 = 85

## JT65

http://www.arrl.org/files/file/18JT65.pdf

- _Data bits_: 72
- _Encoded symbols_: 63
- _Tranmitted symbols_: 126
