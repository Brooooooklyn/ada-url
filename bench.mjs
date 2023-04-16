import b from 'benny'

import { validate } from './index.js'

await b.suite(
  'validate url',

  b.add('node', () => {
    new URL('http://google.com')
  }),

  b.add('ada-js', () => {
    validate('http://google.com')
  }),

  b.cycle(),
  b.complete(),
)