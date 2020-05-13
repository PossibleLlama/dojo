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
		{"", "Happy birthday, dear !"},
		{"alice", "Happy birthday, dear alice!"},
		{"bob", "Happy birthday, dear bob!"},
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
