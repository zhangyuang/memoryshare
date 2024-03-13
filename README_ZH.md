# Share Memory

`memoryshare` 用于在不同的 Node.js 进程间共享内存

当数据量庞大的时候 `memoryshare` 具有更高的性能相比于使用 IPC 通道的方式传递数据

## 功能

- 高性能 ✨
- 更简单的 api 接口 💗

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
import { init, setString, getString, clear } from 'memoryshare'

const memId = "string.link"

init(memId, 4096) // 每一个 memId 的初始化操作只需要做一次

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
