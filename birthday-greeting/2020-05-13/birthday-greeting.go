package main

import (
	"bytes"
	"fmt"
	"strings"
)

type Contact struct {
	firstName string
	lastName  string
	email     string
}

func GenContact(firstName string) Contact {
	c := Contact{firstName: firstName}
	return c
}

func main() {
	fmt.Println("Greetings friend!")
}

// MessageFromName Get happy birthday message for given person
func MessageFromName(person Contact) string {
	return fmt.Sprintf("Happy birthday, dear %s!", pascalCaseWithSpaces(person.firstName))
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
	return string(bytes.Join([][]byte{firstChar, charArr[1:]}, nil))
}
