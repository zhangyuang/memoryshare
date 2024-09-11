import { fork } from 'child_process'
import { init, setString, getString, clear, setBuffer, getBuffer } from './index'

const memId = "string.link"
const bufferMemId = "buffer.link"
clear(memId)
clear(bufferMemId)

init(memId, 4096)
init(bufferMemId, 4096)

function generateBigString() {
  let bigStr = '';
  for (let i = 0; i < 50; i++) {
    bigStr += 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. ';
  }
  return bigStr;
}



const child = fork('./child')

export const testByShareMemory = () => {
  setString(memId, generateBigString())
  child.send({
    type: 'share',
  })
}
export const testBufferByShareMemory = () => {
  const buffer = Buffer.from(generateBigString());
  setBuffer(bufferMemId, buffer);
  child.send({
    type: 'shareBuffer',
  });
}
export const testByIpc = () => {
  const child = fork('./child')
  child.send({
    type: 'ipc',
    data: generateBigString()
  })
}
if (process.env.TEST) {
  testByShareMemory()
  testBufferByShareMemory();
}
child.on('message', (msg: { type: string, data: any }) => {
  if (msg.type === 'exit') {
    child.kill()
    process.exit()
  }
})

const errHandleFactory = (eventName) => (err) => {
  console.log(eventName)
  console.error(err)
  if (['SIGINT', 'SIGQUIT', 'SIGHUP'].includes(eventName)) {
    process.exit(0)
  } else {
    process.exit(1)
  }
}
process.on('uncaughtException', errHandleFactory('uncaughtException'));
process.on('unhandledRejection', errHandleFactory('unhandledRejection'));
process.on('SIGINT', errHandleFactory('SIGINT'));
process.on('SIGQUIT', errHandleFactory('SIGQUIT'));
process.on('SIGHUP', errHandleFactory('SIGHUP'));
