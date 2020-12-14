import unittest
from main import read_numbers_from_string, find_numbers_that_sum_to

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

class TestFindNumbersThatSumTo(unittest.TestCase):
    def test_empty_list(self):
        sum = 0
        input_list = []
        numbers_that_sum_to = find_numbers_that_sum_to(sum, input_list)

        self.assertEqual(len(numbers_that_sum_to), 0)

    def test_single_item_list_equals_sum(self):
        sum = 1
        input_list = [1]
        numbers_that_sum_to = find_numbers_that_sum_to(sum, input_list)

        self.assertEqual(len(numbers_that_sum_to), 1)
        self.assertEqual(numbers_that_sum_to[0], [sum])

    def test_single_item_list_unequal_sum(self):
        sum = 2
        input_list = [1]
        numbers_that_sum_to = find_numbers_that_sum_to(sum, input_list)

        self.assertEqual(len(numbers_that_sum_to), 0)

    def test_two_item_list_equals_sum(self):
        sum = 3
        input_list = [1,2]
        numbers_that_sum_to = find_numbers_that_sum_to(sum, input_list)

        assert_sum_list(self, sum, numbers_that_sum_to)
        self.assertEqual(len(numbers_that_sum_to), 1)

    def test_multiple_item_list_has_one_match(self):
        sum = 10
        input_list = [5,6,8,3,4]
        numbers_that_sum_to = find_numbers_that_sum_to(sum, input_list)

        assert_sum_list(self, sum, numbers_that_sum_to)
        self.assertEqual(len(numbers_that_sum_to), 1)
        self.assertEqual(numbers_that_sum_to[0], [4,6])

    def test_multiple_items_list_has_duplicate(self):
        sum = 10
        input_list = [2,3,5,9,5,6]
        numbers_that_sum_to = find_numbers_that_sum_to(sum, input_list)

        assert_sum_list(self, sum, numbers_that_sum_to)
        self.assertEqual(len(numbers_that_sum_to), 1)
        self.assertEqual(numbers_that_sum_to[0], [5,5])

def generate_error(string):
    return string + " is not a number. Error " + generate_int_parse_error(string)

def generate_int_parse_error(string):
    return "invalid literal for int() with base 10: '{0}'".format(string)

def assert_sum_list(self, sum, sum_list):
    self.assertGreaterEqual(len(sum_list), 1)
    for pair in sum_list:
        self.assertEqual(pair[0] + pair[1], sum)

if __name__ == '__main__':
    unittest.main()
