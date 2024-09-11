# Share Memory

`memoryshare` is a module allow Node.js developers share memory between different process

It's high performance when share data is huge than transfer data by IPC

## features

- High performance ✨
- Simple api interface 💗
- Support share string andbuffer data 💗

## benchmark

```bash
$ yarn bench
Running "data transfer" suite...
Progress: 100%

  ipc:
    26 ops/s, ±96.96%       | slowest, 99.93% slower

  sharString:
    13 686 ops/s, ±72.23%   | 65.29% slower

  shareBuffer:
    39 428 ops/s, ±8.67%   | fastest

Finished 3 cases!
  Fastest: sharebuffer
  Slowest: ipc
✨  Done in 28.89s.
```

## Install

```bash
$ npm i memoryshare # or yarn add memoryshare
```

## Support Platform

Note: You need to make sure that the compilation environment of the dynamic
library is the same as the installation and runtime environment of the `memoryshare` call.

- darwin-x64
- darwin-arm64
- linux-x64-gnu
- win32-x64-msvc
- win32-ia32-msvc
- linux-arm64-gnu
- linux-arm64-musl

## How to use

`memoryshare` support for data that can be serialized as string

Here is an example to guide how to share string data between different process

```js
// main.js
import { fork } from 'child_process'
import { init, setString, getString, clear, setBuffer, getBuffer } from 'memoryshare'

const memId = "string.link"
const bufferMemId = "buffer.link"
clear(memId)
clear(bufferMemId)

init(memId, 4096) // init share memory block with max size,each memId should be called only once
init(bufferMemId, 4096)


function generateBigString() {
  let bigStr = '';
  for (let i = 0; i < 1; i++) {
    bigStr += 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. ';
  }
  return bigStr;
}


setString(memId, generateBigString())
setBuffer(bufferMemId, Buffer.from(generateBigString()))
fork('./child')

// child.js
const memId = "string.link"
const data = getString(memId)
const bufferData = getBuffer(bufferMemId)
```
