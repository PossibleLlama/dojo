package main

import "fmt"

func main() {
	fmt.Println("start")
}

func appendIfDivisible(number, divisible int, inputString, appendableString string) string {
	if number%divisible == 0 {
		return inputString + appendableString
	}
	return inputString
}

func GetStringForNumber(number int) string {
	output := ""
	output = appendIfDivisible(number, 3, output, "Fizz")
	output = appendIfDivisible(number, 5, output, "Buzz")

	if len(output) > 0 {
		return output
	}
	return fmt.Sprint(number)
}

func FizzBuzz(number int) string {
	return ""
}
