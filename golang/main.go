package main

import (
	"fmt"
	"strconv"
)

// isA115569 checks if a number is a Lynch-Bell number
func isA115569(num int) bool {
	digitCount := make(map[int]int)

	digits := strconv.Itoa(num)
	for _, c := range digits {
		digit, _ := strconv.Atoi(string(c))
		digitCount[digit]++
		if digit == 0 || digitCount[digit] > 1 || num%digit != 0 {
			return false
		}
	}
	return true
}

func main() {
	// No "Closed form" for straightforward approach so using Predicate
	// Reproducing... https://oeis.org/A115569/list
	fmt.Print("[")
	for i := 1; i <= 1900; i++ {
		if isA115569(i) {
			if i > 1 {
				fmt.Print(", ")
			}
			fmt.Print(i)
		}
	}
	fmt.Print("]")
}
