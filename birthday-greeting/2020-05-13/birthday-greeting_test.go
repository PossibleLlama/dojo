package main

import (
	"fmt"
	"testing"
)

func TestMessageFromName(t *testing.T) {
	var tests = []struct {
		input    Contact
		expected string
	}{
		{GenContact(""), nameInMessage("")},
		{GenContact("alice"), nameInMessage("Alice")},
		{GenContact("bob"), nameInMessage("Bob")},
		{GenContact("c"), nameInMessage("C")},
		{GenContact("dd"), nameInMessage("Dd")},
		{GenContact("edwina eucalyptus"), nameInMessage("Edwina Eucalyptus")},
	}

	for _, testItem := range tests {
		testName := fmt.Sprintf("%s", testItem.input)
		t.Run(testName, func(t *testing.T) {
			actual := MessageFromName(testItem.input)
			if actual != testItem.expected {
				t.Errorf("Got %s, expected %s", actual, testItem.expected)
			}
		})
	}
}

func nameInMessage(name string) string {
	return fmt.Sprintf("Happy birthday, dear %s!", name)
}
