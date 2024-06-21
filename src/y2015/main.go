package y2015

import (
	"fmt"
	"os"
)

func Run() {
	day := os.Getenv("DAY")

	switch day {
	case "1":
		Day1()
	case "2":
		Day2()
	default:
		panic(fmt.Sprintf("Day %s not yet implemented!", day))
	}
}
