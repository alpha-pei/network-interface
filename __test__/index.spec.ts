import test from 'ava'

import { interfaces,IFF_ETH } from '../index.js'

test('sync function from native code', (t) => {
  t.log(interfaces(IFF_ETH));
  t.pass();
})
