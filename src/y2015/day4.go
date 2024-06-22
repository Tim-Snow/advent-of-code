package y2015

import (
	"crypto/md5"
	"encoding/hex"
	"fmt"
	"strings"
	"util"
)

func Day4() {
	test_data := util.GetDayTestData()

	c := make(chan []byte)
	go util.GetDayData(c)
	data := <-c

	util.Assert(day4_part1(test_data), 609043)

	defer util.Duration(util.Track())

	fmt.Printf("Part 1: %d\n", day4_part1(data))
	fmt.Printf("Part 2: %d\n", day4_part2(data))
}

func day4_part1(data []byte) int {
	return findNumberSuffixResultingInHashPrefix(data, "00000")
}

func day4_part2(data []byte) int {
	return findNumberSuffixResultingInHashPrefix(data, "000000")
}

func findNumberSuffixResultingInHashPrefix(secret_key []byte, prefix string) int {
	for i := 1; i < 100_000_000; i++ {
		concatenated := fmt.Sprintf("%s%v", string(secret_key), i)
		hash := md5.Sum([]byte(concatenated))
		str := hex.EncodeToString(hash[:])

		if strings.HasPrefix(str, prefix) {
			return i
		}
	}

	panic("Could not find a working number")
}
