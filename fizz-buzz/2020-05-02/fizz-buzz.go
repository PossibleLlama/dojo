package main

import "fmt"

func main() {
	fmt.Println("start")
}

func FizzBuzz(number int) string {
	if number == 3 {
		return "Fizz"
	} else if number == 5 {
		return "Buzz"
	}
	return fmt.Sprint(number)
}