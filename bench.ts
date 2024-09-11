import b from 'benny'
import { testByIpc, testByShareMemory, testBufferByShareMemory } from './test'




async function run() {
  await b.suite(
    'data transfer',
    b.add('ipc', () => {
      testByIpc()
    }),
    b.add('shareString', () => {
      testByShareMemory()
    }),
    b.add('shareBuffer', () => {
      testBufferByShareMemory()
    }),
    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
}).then(() => {
  process.exit()
})
