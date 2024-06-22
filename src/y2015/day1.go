package y2015

import (
	"fmt"
	"util"
)

func Day1() {
	test_data := util.GetDayTestData()

	c := make(chan []byte)
	go util.GetDayData(c)
	data := <-c

	util.Assert(part1(test_data), 3)
	util.Assert(part2(test_data), 1)

	defer util.Duration(util.Track())

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
