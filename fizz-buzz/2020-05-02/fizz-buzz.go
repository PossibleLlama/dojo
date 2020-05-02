package main

import (
	"fmt"
	"strings"
)

// IntReplacement used as a way to join a number to the string that replaces it
// E.g. 3 -> "Fizz"
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

	fmt.Println(ReplaceBetweenRange(1, 100, replacements, "\n"))
}

// AppendIfDivisible Check if the given number is divisible by passed divisor, and if so return the string instead
func AppendIfDivisible(number int, replaceable IntReplacement) string {
	if number%replaceable.divisor == 0 {
		return replaceable.replacement
	}
	return ""
}

// GetStringForNumber For a given number, check each replacement string and replace as needed
func GetStringForNumber(number int, replacements []IntReplacement) string {
	output := ""
	for _, element := range replacements {
		output += AppendIfDivisible(number, element)
	}

	if len(output) > 0 {
		return output
	}
	return fmt.Sprint(number)
}

// ReplaceBetweenRange For a starting number, list of replacements and a string to put between strings, put it all together
func ReplaceBetweenRange(start, end int, replacements []IntReplacement, intermediary string) string {
	output := ""
	for i := start; i <= end; i++ {
		output += GetStringForNumber(i, replacements) + intermediary
	}
	return strings.TrimSpace(output)
}
