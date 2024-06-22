package y2015

import (
	"fmt"
	"util"
)

func Day3() {
	test_data := util.GetDayTestData()

	c := make(chan []byte)
	go util.GetDayData(c)
	data := <-c

	util.Assert(day3_part1(test_data), 4)
	util.Assert(day3_part2(test_data), 3)

	defer util.Duration(util.Track())

	fmt.Printf("Part 1: %d\n", day3_part1(data))
	fmt.Printf("Part 2: %d\n", day3_part2(data))
}

func day3_part1(data []byte) int {
	return len(get_positions(data))
}

func day3_part2(data []byte) int {
	var santa_instructions []byte
	var robo_instructions []byte

	for i, c := range data {
		if i%2 == 0 {
			santa_instructions = append(santa_instructions, c)
		} else {
			robo_instructions = append(robo_instructions, c)
		}
	}

	santa_visited_positions := get_positions(santa_instructions)
	robo_visited_positions := get_positions(robo_instructions)

	combined := santa_visited_positions

	for k, v := range robo_visited_positions {
		_, found := combined[k]

		if found {
			combined[k] += v
		} else {
			combined[k] = v
		}
	}

	return len(combined)
}

func get_positions(data []byte) map[string]int {
	position := map[string]int{
		"x": 0,
		"y": 0,
	}

	visited_positions := map[string]int{
		fmt.Sprintf("%d,%d", position["x"], position["y"]): 1,
	}

	for _, char := range data {
		switch char {
		case '<':
			position["x"] -= 1
		case '>':
			position["x"] += 1
		case '^':
			position["y"] += 1
		case 'v':
			position["y"] -= 1
		}

		new_position := fmt.Sprintf("%d,%d", position["x"], position["y"])
		visits, found := visited_positions[new_position]

		if found {
			visited_positions[new_position] = visits + 1
		} else {
			visited_positions[new_position] = 1
		}
	}

	return visited_positions
}
