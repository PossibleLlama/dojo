package main

import "fmt"

func main() {
	fmt.Println("start")
}

func FizzBuzz(number int) string {
	if number == 3 {
		return "Fizz"
	}
	return fmt.Sprint(number)
}
