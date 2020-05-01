# Birthday greeting Kata

## Origin

This kata came from [coding dojo](http://codingdojo.org/kata/birthday-greetings/).

## Problem

I need to be able to send a message to a person on their birthday.

I have a csv file with their `last_name`, `first_name`, `data_of_birth` and `email` in, and want to send a message in the following format.

``` text
Subject: Happy birthday!
Happy birthday, dear <first_name>!
```

### CSV file

``` csv
last_name, first_name, date_of_birth, email
Apple, Alice, 1982/10/08, alice.apple@foobar.com
Banana, Bob, 1975/09/11, bob.banana@foobar.com
```

### Additional considerations

- Make the methods for retreiving the data from the CSV file and sending the email, not specific to each.
How easy will it be to convert it to an SQL db, or send via SMS?

### Additional features

- Send a note to friends, reminding them of another persons birthday.

``` text
Subject: Birthday Reminder

Dear <first_name>,

Today is <someone_else_first_name> <someone_else_last_name>'s birthday.
Don't forget to send them a message !
```

- Send a single reminder to all other friends at once.

``` text
Subject: Birthday Reminder

Dear <first_name>,

Today is <full_name_1>, <full_name_2> and <full_name_3>'s birthday.
Don't forget to send them a message !
```
