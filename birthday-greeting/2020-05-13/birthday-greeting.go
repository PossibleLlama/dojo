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
	return fmt.Sprintf("Happy birthday, dear %s!", pascalCaseWithSpaces(firstName))
}

func pascalCaseWithSpaces(word string) string {
	allWords := strings.Split(word, " ")
	finalWord := ""

	for i := 0; i < len(allWords); i++ {
		if len(allWords[i]) < 2 {
			return strings.ToUpper(allWords[i])
		}

		finalWord += uppercaseFirstCharacter(allWords[i]) + " "
	}
	return strings.Trim(finalWord, " ")
}

func uppercaseFirstCharacter(word string) string {
	charArr := []byte(word)

	firstChar := bytes.ToUpper([]byte{charArr[0]})
	otherChars := charArr[1:]
	return string(bytes.Join([][]byte{firstChar, otherChars}, nil))
}
