# CS-128H-Final-Project-Splendor
A rust program that imitate the play of the board game splendor

Group Name: The guy doing Board Game
Group Member: Anjie Xu (anjiexu2)

Introduction:
This project is my work for CS 128 honor section final project. I plan to create a program that allows people to play the board game splendor on the computer. It's quite painful to carry the whole board game around, especially when you are travelling. So this program would be quite beneficial (at least for me and my friends).
This is a linke to the rulebook of Splendor: https://cdn.1j1ju.com/medias/7f/91/ba-splendor-rulebook.pdf

Technicals:
I'm planning on writing different structs to immiatate different mechanisms in the board game. And implement functions that perform these tasks.
The main components are made of different objects that immitate different piles in the original game. For example, the card piles that contains card objects. The stone pile that register the number of stone tokens left, and how players interact with these tokens. And also the noble pile that implement the mechanism of nobles in the board game. I make use of a enumerated datatype that represent different colors in the game. And uses many hashmaps the relate these colors to specific integer values. There are also a struct that represent players, and manage the flow of the game.

Checkpoint 1: I'm looking forward to finish the implementation of the Card struct, card_pile struct, stone_pile struct, noble_pile struct, player struct.

Checkpoint 2: I with to be able to finish implementing the main function, so that it can control the whole game flow. I'm also looking forward to finish writing a user-interface that allow actual players to interact with the game.

Possible Challenges: I think the biggest issue is to form a good user-interface in the program. However, my tools are limited to the simple input output in the terminal. It's going to be hard to formulate an interface.
