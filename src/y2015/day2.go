package y2015

import (
	"fmt"
	"slices"
	"strconv"
	"strings"
	"util"
)

func Day2() {
	test_data := util.GetDayTestData()
	data := util.GetDayData()

	util.Assert(day2_part1(test_data), 58)
	util.Assert(day2_part2(test_data), 34)

	defer util.Duration(util.Track())

	fmt.Printf("Part 1: %d\n", day2_part1(data))
	fmt.Printf("Part 2: %d\n", day2_part2(data))
}

func day2_part1(data []byte) int {
	total := 0

	lines := strings.Split(string(data), "\n")

	for _, line := range lines {
		split := strings.Split(line, "x")
		l, err := strconv.Atoi(split[0])
		util.Check(err)
		w, err := strconv.Atoi(split[1])
		util.Check(err)
		h, err := strconv.Atoi(split[2])
		util.Check(err)

		total += required_wrapping_paper(l, w, h)

	}

	return total
}

func day2_part2(data []byte) int {
	total := 0

	lines := strings.Split(string(data), "\n")

	for _, line := range lines {
		split := strings.Split(line, "x")
		l, err := strconv.Atoi(split[0])
		util.Check(err)
		w, err := strconv.Atoi(split[1])
		util.Check(err)
		h, err := strconv.Atoi(split[2])
		util.Check(err)

		total += required_ribbon(l, w, h)

	}

	return total
}

func required_wrapping_paper(length int, width int, height int) int {
	side_1 := length * width
	side_2 := width * height
	side_3 := height * length

	smallest := min(min(side_1, side_2), side_3)

	return 2*side_1 + 2*side_2 + 2*side_3 + smallest
}

func required_ribbon(length int, width int, height int) int {
	sizes := []int{length, width, height}
	slices.Sort(sizes)
	wrap := sizes[0]*2 + sizes[1]*2
	bow := length * width * height

	return wrap + bow
}
