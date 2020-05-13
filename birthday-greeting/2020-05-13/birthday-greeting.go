package main

import (
	"bytes"
	"fmt"
	"strings"
)

// Contact object to hold details for a given contact
type Contact struct {
	firstName string
	lastName  string
	email     string
}

// GenContact Helper function to create a Contact
func GenContact(firstName, lastName, email string) Contact {
	return Contact{firstName: firstName, lastName: lastName, email: email}
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
