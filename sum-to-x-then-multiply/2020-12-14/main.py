def read_numbers_from_string(string):
    stringExceptions = []

    if len(string.strip()) == 0:
        return [], stringExceptions

    stringItems = string.split("\n")
    numberItems = []

    for element in stringItems:
        try:
            numberItems.append(int(element))
        except ValueError as err:
            stringExceptions.append("{0} is not a number. Error {1}".format(element, err))

    return numberItems, stringExceptions

def find_numbers_that_sum_to(sum, number_list):
    sum_to = []
    if len(number_list) == 0: pass
    elif len(number_list) == 1:
        if sum == number_list[0]:
            sum_to.append([number_list[0]])
        else: pass
    else:
        for x in sorted(number_list):
            for y in sorted(number_list):
                if x < y or (x + y) > sum:
                    break
                elif x == y and number_list.count(x) == 1:
                    break
                elif sum_to.count([x, x]) == 1:
                    break
                elif x + y == sum:
                    sum_to.append(sorted([x, y]))
            else:
                continue

    return sum_to

file = open("../input.txt")
file_contents = file.read()

list_of_numbers, errors = read_numbers_from_string(file_contents)
matching_pairs = find_numbers_that_sum_to(2020, list_of_numbers)

if (len(matching_pairs) > 0):
    print("Found a pair. {0} and {1}. Multiplied together get {2}".
        format(
            matching_pairs[0][0],
            matching_pairs[0][1],
            matching_pairs[0][0] * matching_pairs[0][1]))
