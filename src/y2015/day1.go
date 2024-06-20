package y2015

import (
	"fmt"
	"os"
	"time"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func assert[T comparable](actual T, expected T) {
	if actual != expected {
		panic(fmt.Sprintf("expected (%+v) is not equal to actual (%+v)", expected, actual))
	}
}

func track() time.Time {
	return time.Now()
}

func duration(start time.Time) {
	fmt.Printf("Execution time: %v\n", time.Since(start))
}

func Day1() {
	test_data, err := os.ReadFile("../res/2015/1.test.txt")
	check(err)
	data, err := os.ReadFile("../res/2015/1.txt")
	check(err)

	p1 := part1(test_data)
	assert(p1, 3)

	p2 := part2(test_data)
	assert(p2, 1)

	defer duration(track())

	fmt.Printf("Part 1: %d\n", part1(data))
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
