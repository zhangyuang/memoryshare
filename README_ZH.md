# Share Memory

`memoryshare` 用于在不同的 Node.js 进程间共享内存

当数据量庞大的时候 `memoryshare` 具有更高的性能相比于使用 IPC 通道的方式传递数据

## 功能

- 高性能 ✨
- 更简单的 api 接口 💗
- 支持 String/Buffer 类型 💗

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

## 安装

```bash
$ npm i memoryshare # or yarn add memoryshare
```

## 如何使用

`memoryshare` 支持传递能够被序列化为字符串类型的数据

下面的例子描述了如何使用 `memoryshare` 来共享内存

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
