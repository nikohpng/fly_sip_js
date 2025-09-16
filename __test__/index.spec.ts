import test from 'ava'

import { plus100 } from '../index'

test('sync function from native code', (t) => {
  const fixture = 42
  t.is(plus100(fixture), fixture + 100)
})

// test('init SDK', (t) => {
//   t.is(flySipInitSdkJs("192.168.1.77", "192.168.1.77", "pbx", "123456", "", "", "", 0), 0)
// })