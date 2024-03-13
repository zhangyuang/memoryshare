# Share Memory

`memoryshare` is a module allow Node.js developers share memory between different process

It's high performance when share data is huge than transfer data by IPC

## features

- High performance ✨
- Simple api interface 💗

## benchmark

```bash
$ yarn bench

ipc:
  34 ops/s, ±55.24%       | slowest, 99.73% slower

sharememory:
  12 412 ops/s, ±44.21%   | fastest

Finished 2 cases!
Fastest: sharememory
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
import { init, setString, getString, clear } from 'memoryshare'

const memId = "string.link"

init(memId, 4096) //  Each memId only should be call init function once

function generateBigString() {
  let bigStr = '';
  for (let i = 0; i < 1; i++) {
    bigStr += 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. ';
  }
  return bigStr;
}

setString(memId, generateBigString())

fork('./child')

// child.js
const memId = "string.link"
const data = getString(memId)
```
