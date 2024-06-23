package y2015

import (
	"fmt"
	"slices"
	"strings"
	"util"
)

func Day5() {
	test_data := util.GetDayTestData()

	c := make(chan []byte)
	go util.GetDayData(c)
	data := <-c

	util.Assert(day5_part1(test_data), 2)
	util.Assert(day5_part2(test_data), 3)

	defer util.Duration(util.Track())

	fmt.Printf("Part 1: %d\n", day5_part1(data))
	fmt.Printf("Part 2: %d\n", day5_part2(data))
}

func day5_part1(data []byte) int {
	total_nice_strings := 0

	lines := strings.Split(string(data), "\n")

	for _, line := range lines {
		if contains_three_vowels(line) && contains_two_consecutive_letters(line) && !contains_naughty_strings(line) {
			total_nice_strings += 1
		}
	}

	return total_nice_strings
}

func day5_part2(data []byte) int {
	total_nice_strings := 0

	lines := strings.Split(string(data), "\n")

	for _, line := range lines {
		if contains_two_letters_twice_no_overlap(line) && contains_one_letter_twice_with_one_letter_between(line) {
			total_nice_strings += 1
		}
	}

	return total_nice_strings
}

func contains_three_vowels(s string) bool {
	number_of_vowels := 0
	vowels := []string{"a", "e", "i", "o", "u"}

	for _, c := range s {
		if slices.Contains(vowels, string(c)) {
			number_of_vowels += 1
		}
	}

	return number_of_vowels >= 3
}

func contains_two_consecutive_letters(s string) bool {
	var previous_letter rune

	for _, c := range s {
		if previous_letter == c {
			return true
		}

		previous_letter = c
	}

	return false
}

func contains_naughty_strings(s string) bool {
	naughty_strings := []string{"ab", "cd", "pq", "xy"}

	for i := 0; i < len(s)-1; i++ {
		for _, naughty_string := range naughty_strings {
			if strings.Contains(naughty_string, fmt.Sprintf("%s%s", string(s[i]), string(s[i+1]))) {
				return true
			}
		}
	}

	return false
}

func contains_two_letters_twice_no_overlap(s string) bool {
	var two_letter_pairs []string

	for i := 0; i < len(s)-1; i++ {
		pair := fmt.Sprintf("%s%s", string(s[i]), string(s[i+1]))

		for j := 0; j < len(two_letter_pairs)-1; j++ {
			if two_letter_pairs[j] == pair {
				return true
			}
		}

		two_letter_pairs = append(two_letter_pairs, pair)
	}

	return false
}

func contains_one_letter_twice_with_one_letter_between(s string) bool {
	for i := 2; i < len(s); i++ {
		if s[i] == s[i-2] {
			return true
		}
	}

	return false
}
