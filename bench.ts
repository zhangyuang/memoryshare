import b from 'benny'
import { testByIpc, testByShareMemory } from './test'




async function run() {
  await b.suite(
    'data transfer',
    b.add('ipc', () => {
      testByIpc()
    }),
    b.add('sharememory', () => {
      testByShareMemory()
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
