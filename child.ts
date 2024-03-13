import { equal } from 'assert'
import { init, setString, getString, clear } from './index'

const memId = "string.link"

function generateBigString() {
  let bigStr = '';
  for (let i = 0; i < 100; i++) {
    bigStr += 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. ';
  }
  return bigStr;
}


process.on('message', msg => {
  if (msg.type === 'share') {
    const data = getString(memId)
    if (!process.env.BENCH) {
      equal(data, generateBigString())
      process.send({
        type: 'exit'
      })
    }

  }
  if (msg.type === 'ipc') {
    const str = msg.data
  }
})
