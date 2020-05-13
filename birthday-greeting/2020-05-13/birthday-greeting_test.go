package main

import (
	"fmt"
	"testing"
)

func TestMessageFromName(t *testing.T) {
	var tests = []struct {
		input    string
		expected string
	}{
		{"", nameInMessage("")},
		{"alice", nameInMessage("Alice")},
		{"bob", nameInMessage("Bob")},
		{"c", nameInMessage("C")},
		{"dd", nameInMessage("Dd")},
		{"edwina eucalyptus", nameInMessage("Edwina Eucalyptus")},
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
