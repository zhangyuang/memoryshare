import { equal } from 'assert'
import { init, setString, getString, getBuffer } from './index'

const memId = "string.link"

const bufferMemId = "buffer.link"

function generateBigString() {
  let bigStr = '';
  for (let i = 0; i < 50; i++) {
    bigStr += 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. ';
  }
  return bigStr;
}
function generateBigBuffer() {
  return Buffer.from(generateBigString())
}

process.on('message', (msg: { type: string, data: any }) => {
  if (msg.type === 'share') {
    const data = getString(memId)
    if (!process.env.BENCH) {
      equal(data, generateBigString())
      process.send?.({
        type: 'exit'
      })
    }
  } else if (msg.type === 'shareBuffer') {
    const data = getBuffer(bufferMemId)
    if (!process.env.BENCH) {
      equal(data.toString(), generateBigBuffer().toString())
      process.send?.({
        type: 'exit'
      })
    }
  } else if (msg.type === 'ipc') {
    const str = msg.data
  }
})