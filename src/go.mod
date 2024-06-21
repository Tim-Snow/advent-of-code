module advent_of_code

go 1.22.4

require y2015 v0.0.0-unpublished

require (
	github.com/joho/godotenv v1.5.1
	util v0.0.0-unpublished // indirect
)

replace (
	util v0.0.0-unpublished => ./util
	y2015 v0.0.0-unpublished => ./y2015
)
