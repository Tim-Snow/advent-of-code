package main

import (
	"fmt"
	"os"
	"y2015"

	"github.com/joho/godotenv"
)

func main() {
	if godotenv.Load("../.env") != nil {
		panic("Error loading .env file")
	}

	year := os.Getenv("YEAR")

	switch year {
	case "2015":
		y2015.Run()
	default:
		panic(fmt.Sprintf("Year %s not yet implemented!", year))
	}

}
