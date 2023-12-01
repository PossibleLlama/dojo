package main

import (
	"testing"

	"gotest.tools/assert"
)

func TestGetIntegers(t *testing.T) {
	tests := []struct {
		input    string
		expected []int
	}{
		{"", []int{}},
		{"1", []int{1}},
		{"12", []int{1, 2}},
		{"123", []int{1, 2, 3}},
		{"1a", []int{1}},
	}

	for _, test := range tests {
		actual := GetIntegers(test.input)
		assert.Equal(t, len(test.expected), len(actual))
		for index, i := range test.expected {
			assert.Equal(t, i, actual[index])
		}
	}
}

func TestGetFirstAndLastIntegersCombined(t *testing.T) {
	tests := []struct {
		input    []int
		expected int
	}{
		{[]int{}, 0},
		{[]int{1}, 11},
		{[]int{1, 2}, 12},
		{[]int{1, 2, 3}, 13},
	}

	for _, test := range tests {
		actual := GetFirstAndLastIntegersCombined(test.input)
		assert.Equal(t, test.expected, actual)
	}
}

func TestStripNonIntegers(t *testing.T) {
	tests := []struct {
		input    string
		expected string
	}{
		{"", ""},
		{"1", "1"},
		{"12", "12"},
		{"a", ""},
		{"1a", "1"},
		{"a1", "1"},
		{"A1", "1"},
		{"!1", "1"},
		{"1\n", "1"},
	}

	for _, test := range tests {
		actual := StripNonIntegers(test.input)
		assert.Equal(t, test.expected, actual)
	}
}
