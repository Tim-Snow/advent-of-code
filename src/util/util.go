package util

import (
	"fmt"
	"io"
	"net/http"
	"os"
	"time"
)

func GetDayData(c chan<- []byte) {
	data, err := os.ReadFile(fmt.Sprintf("../res/%s/%s.txt", os.Getenv("YEAR"), os.Getenv("DAY")))

	if err != nil {
		client := http.Client{}
		req, err := http.NewRequest("GET", fmt.Sprintf("https://adventofcode.com/%s/day/%s/input", os.Getenv("YEAR"), os.Getenv("DAY")), nil)
		Check(err)

		req.Header.Set("Cookie", fmt.Sprintf("session=%s", os.Getenv("COOKIE")))
		req.Header.Set("User-Agent", os.Getenv("USER_AGENT"))

		res, err := client.Do(req)
		Check(err)

		body, err := io.ReadAll(res.Body)
		Check(err)

		Check(os.WriteFile(fmt.Sprintf("../res/%s/%s.txt", os.Getenv("YEAR"), os.Getenv("DAY")), body, 0644))

		c <- body
	} else {
		c <- data
	}
}

func GetDayTestData() []byte {
	test_data, err := os.ReadFile(fmt.Sprintf("../res/%s/%s.test.txt", os.Getenv("YEAR"), os.Getenv("DAY")))
	Check(err)

	return test_data
}

func Check(e error) {
	if e != nil {
		panic(e)
	}
}

func Assert[T comparable](actual T, expected T) {
	if actual != expected {
		panic(fmt.Sprintf("expected (%+v) is not equal to actual (%+v)", expected, actual))
	}
}

func Track() time.Time {
	return time.Now()
}

func Duration(start time.Time) {
	fmt.Printf("Execution time: %v\n", time.Since(start))
}
