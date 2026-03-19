import { ControllerRequest } from '@party/proto/controller-types-v1'

// Test encoding
const msg = ControllerRequest.fromJSON({ ping: { msg: 'hello' } })

console.log('Message object:', msg)

// Test binary encode/decode
const encoded = ControllerRequest.encode(msg).finish()
const decoded = ControllerRequest.decode(encoded)

console.log('Decoded:', decoded)
console.log('Message text:', decoded.ping?.msg)