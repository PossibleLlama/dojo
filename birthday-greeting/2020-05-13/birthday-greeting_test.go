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
		{"alice", "Happy birthday, dear Alice!"},
		{"bob", "Happy birthday, dear Bob!"},
		{"c", "Happy birthday, dear C!"},
		{"dd", "Happy birthday, dear Dd!"},
		{"edwina eucalyptus", "Happy birthday, dear Edwina Eucalyptus!"},
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
