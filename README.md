# Guessing game

`source : The Rust Programming Language book (chapter 2)`

this is a guessing game where the user tries to guess the secret number (generated with a crate called rand) and the program indicates at each time he guesses if the secret number is greater then the user's guess or not, at the end when the user finds the actual secret number, the program prints the number of tries the user had to make to guess the value of the secret number.

### output (-> for user input)

```
--- Guessing game ---
Guess the secret number
-> 50
The secret number is lower than 50
Guess the secret number
-> 25
The secret number is greater than 25
Guess the secret number
-> 35
The secret number is greater than 35
Guess the secret number
-> 45
The secret number is lower than 45
Guess the secret number
-> 40
You guessed the secret number, its 40
The secret number is: 40, number of tries 5
```
