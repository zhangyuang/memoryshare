import { init, setString, getString, clear } from './index'

const memId = "string.link"



process.send?.({
  type: 'ready'
})

process.on('message', msg => {
  if (msg.type === 'getData') {
    const str = getString(memId)
    console.log('xxx', str)
  }
})
