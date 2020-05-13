package main

import (
	"fmt"
)

func main() {
	fmt.Println("Greetings friend!")
}

// MessageFromName Get happy birthday message for given person
func MessageFromName(firstName string) string {
	return fmt.Sprintf("Happy birthday, dear %s!", firstName)
}
