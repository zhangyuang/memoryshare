import { fork } from 'child_process'
import { init, setString, getString, clear } from './index'

const memId = "string.link"

clear(memId)
init(memId, 4096)

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
export const testByIpc = () => {
  const child = fork('./child')
  child.send({
    type: 'ipc',
    data: generateBigString()
  })
}
if (process.env.TEST) {
  testByShareMemory()
}
child.on('message', msg => {
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
