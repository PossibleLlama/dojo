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
		{generateContactWithJustFirstName(""), nameInMessage("")},
		{generateContactWithJustFirstName("alice"), nameInMessage("Alice")},
		{generateContactWithJustFirstName("bob"), nameInMessage("Bob")},
		{generateContactWithJustFirstName("c"), nameInMessage("C")},
		{generateContactWithJustFirstName("dd"), nameInMessage("Dd")},
		{generateContactWithJustFirstName("edwina eucalyptus"), nameInMessage("Edwina Eucalyptus")},
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

func generateContactWithJustFirstName(firstName string) Contact {
	return GenContact(firstName, "", "")
}

func nameInMessage(name string) string {
	return fmt.Sprintf("Happy birthday, dear %s!", name)
}
