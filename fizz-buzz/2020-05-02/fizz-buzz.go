package main

import (
	"fmt"
	"strings"
)

type IntReplacement struct {
	divisor     int
	replacement string
}

func main() {
	replacements := []IntReplacement{
		{
			divisor:     3,
			replacement: "Fizz"},
		{
			divisor:     5,
			replacement: "Buzz"},
	}

	fmt.Println(ReplaceBetweenRange(1, 100, replacements))
}

func appendIfDivisible(number int, replaceable IntReplacement) string {
	if number%replaceable.divisor == 0 {
		return replaceable.replacement
	}
	return ""
}

func GetStringForNumber(number int, replacements []IntReplacement) string {
	output := ""
	for _, element := range replacements {
		output += appendIfDivisible(number, element)
	}

	if len(output) > 0 {
		return output
	}
	return fmt.Sprint(number)
}

func ReplaceBetweenRange(start, end int, replacements []IntReplacement) string {
	output := ""
	for i := start; i <= end; i++ {
		output += GetStringForNumber(i, replacements) + " "
	}
	return strings.TrimSpace(output)
}
