import { fork } from 'child_process'
import { init, setString, getString, clear } from './index'



const memId = "string.link"

init(memId, 4096)
function generateBigString() {
  let bigStr = '';
  for (let i = 0; i < 10; i++) {
    bigStr += 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. ';
  }
  return bigStr;
}


setString(memId, generateBigString())
console.log('xxx', getString(memId))

// const child = fork('./child')

// child.on('message', (msg: any) => {
//   if (msg.type === 'ready') {
//     child.send({
//       type: 'getData'
//     })
//   }
// });
