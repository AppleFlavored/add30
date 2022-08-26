# Add to 30

## Rules
1. Player 1 (user) inputs a number between 1 and 5.
2. Player 2 (computer) can add either 1, 2, 3, 4, or 5 to Player 1's number.
3. Player 1 can then add either 1, 2, 3, 4, or 5 to Player 2's number.
4. The game continues until one of the players reaches 30.

The numbers that the computer chooses are generated pseudorandomly. The only time
that the number is not random is if the computer has a change to win (`(30 - sum) <= 5`).

## Known Issues
* If the user inputs anything other than an integer, the program will crash. (Maybe I'll fix this later... or not)

## License
The source code is licensed under the MIT License.
See [LICENSE](LICENSE) file for more details.