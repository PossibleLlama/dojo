package main

import (
	"fmt"
	"testing"
)

func TestFunc(t *testing.T) {
	var tests = []struct {
		input    int
		expected string
	}{
		{1, "1"},
		{2, "2"},
		{3, "Fizz"},
		{4, "4"},
	}

	for _, testItem := range tests {
		testName := fmt.Sprintf("%d", testItem.input)
		t.Run(testName, func(t *testing.T) {
			actual := FizzBuzz(testItem.input)
			if actual != testItem.expected {
				t.Errorf("Got %s, expected %s", actual, testItem.expected)
			}
		})
	}
}
