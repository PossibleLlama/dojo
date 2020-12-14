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
