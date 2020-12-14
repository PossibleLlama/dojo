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
