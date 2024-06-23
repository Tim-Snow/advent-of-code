package y2015

import (
	"fmt"
	"slices"
	"strconv"
	"strings"
	"util"
)

func Day6() {
	test_data := util.GetDayTestData()

	c := make(chan []byte)
	go util.GetDayData(c)
	data := <-c

	util.Assert(day6_part1(test_data), 998996)
	util.Assert(day6_part2(test_data), 1001996)

	defer util.Duration(util.Track())

	fmt.Printf("Part 1: %d\n", day6_part1(data))
	fmt.Printf("Part 2: %d\n", day6_part2(data))
}

type instruction struct {
	command string
	from_x  int
	from_y  int
	to_x    int
	to_y    int
}

func day6_part1(data []byte) int {
	grid := [1000][1000]bool{}
	lines := strings.Split(string(data), "\n")

	for _, line := range lines {
		i := parse_instruction(line)

		for x := i.from_x; x <= i.to_x; x++ {
			for y := i.from_y; y <= i.to_y; y++ {
				switch i.command {
				case "on":
					grid[x][y] = true
				case "off":
					grid[x][y] = false
				case "toggle":
					grid[x][y] = !grid[x][y]
				}
			}
		}
	}

	total_lit := 0

	for x := range grid {
		for y := range grid[x] {
			if grid[x][y] {
				total_lit += 1
			}
		}
	}

	return total_lit
}

func day6_part2(data []byte) int {
	grid := [1000][1000]int{}
	lines := strings.Split(string(data), "\n")

	for _, line := range lines {
		i := parse_instruction(line)

		for x := i.from_x; x <= i.to_x; x++ {
			for y := i.from_y; y <= i.to_y; y++ {
				switch i.command {
				case "on":
					grid[x][y] += 1
				case "off":
					if grid[x][y] > 0 {
						grid[x][y] -= 1
					}
				case "toggle":
					grid[x][y] += 2
				}
			}
		}
	}

	total_brightness := 0

	for x := range grid {
		for y := range grid[x] {
			total_brightness += grid[x][y]
		}
	}

	return total_brightness
}

func parse_instruction(s string) *instruction {
	var command string

	if strings.Contains(s, "turn on") {
		command = "on"
	} else if strings.Contains(s, "turn off") {
		command = "off"
	} else {
		command = "toggle"
	}

	b, a, _ := strings.Cut(s, "through")

	f := strings.Split(strings.TrimSpace(b), " ")
	slices.Reverse(f)

	from := strings.Split(strings.TrimSpace(f[0]), ",")
	from_x, _ := strconv.Atoi(from[0])
	from_y, _ := strconv.Atoi(from[1])

	to := strings.Split(strings.TrimSpace(a), ",")
	to_x, _ := strconv.Atoi(to[0])
	to_y, _ := strconv.Atoi(to[1])

	return &instruction{
		command: command,
		from_x:  from_x,
		from_y:  from_y,
		to_x:    to_x,
		to_y:    to_y,
	}
}
