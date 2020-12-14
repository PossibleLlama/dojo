import unittest
from main import read_numbers_from_string

class TestReadNumbersFromString(unittest.TestCase):
    def test_empty_string(self):
        numbers, errors = read_numbers_from_string("")
        self.assertEqual(numbers, [])
        self.assertEqual(errors, [])

    def test_space_string(self):
        numbers, errors = read_numbers_from_string(" ")
        self.assertEqual(numbers, [])
        self.assertEqual(errors, [])

    def test_single_number(self):
        numbers, errors = read_numbers_from_string("0")
        self.assertEqual(numbers, [0])
        self.assertEqual(errors, [])

    def test_two_seperated_numbers(self):
        char = "0\n1"
        numbers, errors = read_numbers_from_string(char)
        self.assertEqual(numbers, [0,1])
        self.assertEqual(errors, [])

    def test_two_seperated_numbers_with_spacing(self):
        char = "0 \n1 "
        numbers, errors = read_numbers_from_string(char)
        self.assertEqual(numbers, [0,1])
        self.assertEqual(errors, [])

    def test_one_alphabetic_character(self):
        char = "a"
        numbers, errors = read_numbers_from_string(char)
        self.assertEqual(numbers, [])
        self.assertEqual(errors, [generate_error(char)])

    def test_two_alphabetic_characters(self):
        char = "ab"
        numbers, errors = read_numbers_from_string(char)
        self.assertEqual(numbers, [])
        self.assertEqual(errors, [generate_error(char)])

    def test_two_seperated_alphabetic_characters(self):
        char = "a\nb"
        numbers, errors = read_numbers_from_string(char)
        self.assertEqual(numbers, [])
        self.assertEqual(errors, [generate_error("a"), generate_error("b")])

    def test_seperated_numbers_and_alphabetic_characters(self):
        char = "0\na\n1\nb\n3"
        numbers, errors = read_numbers_from_string(char)
        self.assertEqual(numbers, [0,1,3])
        self.assertEqual(errors, [generate_error("a"), generate_error("b")])

def generate_error(string):
    return string + " is not a number. Error " + generate_int_parse_error(string)

def generate_int_parse_error(string):
    return "invalid literal for int() with base 10: '{0}'".format(string)

if __name__ == '__main__':
    unittest.main()
