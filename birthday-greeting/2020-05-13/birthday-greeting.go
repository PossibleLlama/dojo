package main

import (
	"bytes"
	"fmt"
	"strings"
)

func main() {
	fmt.Println("Greetings friend!")
}

// MessageFromName Get happy birthday message for given person
func MessageFromName(firstName string) string {
	return fmt.Sprintf("Happy birthday, dear %s!", firstLetterUppercase(firstName))
}

func firstLetterUppercase(word string) string {
	if len(word) < 2 {
		return strings.ToUpper(word)
	}

	charArr := []byte(word)

	firstChar := bytes.ToUpper([]byte{charArr[0]})
	otherChars := charArr[1:]
	return string(bytes.Join([][]byte{firstChar, otherChars}, nil))
}
