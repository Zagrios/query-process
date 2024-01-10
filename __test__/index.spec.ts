/// <reference types="node" />

import test from 'ava'

import { isElevated } from '../index'

// elevated process

test("return false for a non elevated process", (t) => {
    const nonElevatedPID = process.pid
    const result = isElevated(nonElevatedPID)
    t.false(result)
})

test("throw on invalid pid", t => {
    const invalidPID = -1
    t.throws(() => isElevated(invalidPID))
});
