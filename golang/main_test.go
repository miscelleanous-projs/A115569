package main

import (
	"testing"
)

func TestA115569(t *testing.T) {
	assert := func(condition bool) {
		if !condition {
			t.Fatal("Assertion failed")
		}
	}

	// no 0 in digits
	assert(!isA115569(0))
	assert(!isA115569(0))
	assert(!isA115569(10))
	assert(!isA115569(123450789))

	// // duplicate digit(s)
	assert(!isA115569(11))
	assert(!isA115569(999))
	assert(!isA115569(5555))

	// // Some Lynch-Bell numbers
	assert(isA115569(1))
	assert(isA115569(36))
	assert(isA115569(315))
	assert(isA115569(3195))
	assert(isA115569(367248))
	assert(isA115569(9718632))

}
