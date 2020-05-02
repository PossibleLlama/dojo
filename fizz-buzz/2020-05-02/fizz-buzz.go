package main

import "fmt"

func main() {
	fmt.Println("start")
}

func FizzBuzz(number int) string {
	output := ""
	if number%3 == 0 {
		output += "Fizz"
	}
	if number%5 == 0 {
		output += "Buzz"
	}

	if len(output) > 0 {
		return output
	}
	return fmt.Sprint(number)
}
