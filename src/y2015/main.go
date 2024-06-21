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
	default:
		panic(fmt.Sprintf("Day %s not yet implemented!", day))
	}
}
