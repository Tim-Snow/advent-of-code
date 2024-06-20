package y2015

import (
	"fmt"
	"os"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func assert(cond bool, msg string) {
	if !cond {
		panic("Assertion failure! " + msg)
	}
}

func Day1() {
	test_data, err := os.ReadFile("../res/2015/1.test.txt")
	check(err)
	data, err := os.ReadFile("../res/2015/1.txt")
	check(err)

	p1 := part1(test_data)
	assert(p1 == 3, fmt.Sprintf("Part 1, expected 3, got %d", p1))
	fmt.Printf("Part 1: %d\n", part1(data))

	p2 := part2(test_data)
	assert(p2 == 1, fmt.Sprintf("Part 2, expected 1, got %d", p2))
	fmt.Printf("Part 2: %d\n", part2(data))
}

func part1(data []byte) int {
	floor := 0
	for _, char := range data {
		if char == '(' {
			floor += 1
		} else {
			floor -= 1
		}
	}

	return floor
}

func part2(data []byte) int {
	floor := 0
	for pos, char := range data {
		if char == '(' {
			floor += 1
		} else {
			floor -= 1
		}

		if floor == -1 {
			return pos + 1
		}
	}

	panic("Unreachable!")
}
