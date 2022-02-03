# Quantum Quartets
Quantum Quartets is a card game like any other Quartets. 
The only difference is that you know nothing about the cards.
The players have to create enough facts, that one can safely assume, that he has 4 of a kind.

The only facts you know at the start of a game are the following:
 - Every player starts with 4 cards
 - You don't know what kinds of cards (e.g. Heart, Tiles) and which Expressions (e.g. 9, 10, Jack, Queen) exist
 - There are as many kinds as players
 - For every kind, there are 4 Expressions
 - A player can only ask for cards that exist (we may not know yet, that it exists)
 - A player can only ask for a card, if he already has one of its kind

There is a collaborative aspect though. If the players manage to create a paradox (e.g. create five cards of one kind), 
everybody looses, as the whole game makes no sense anymore.

## Example start of a game
The game starts with two players (player A and B), so we know the following things:
 - There are two kinds of cards, with 4 Expressions each
 - We have know idea what kinds or expressions exist
 - Every player has 4 cards

Turn 1 (Player A):

Player A asks B if he has a card of the kind **Animal**, with the Expression **Tiger**.

With this question Player A created three facts:
 - There exists a kind **Animal** (it didn't exist before this question, but now it does)
 - There exists a card with the kind **Animal** and the Expression **Tiger** (same here, this cards didn't exist before this question)
 - Player A must have a card of the kind **Animal** (otherwise he wouldn't be allowed to ask for it)

Now player B can answer with Yes, or No. Let's look at both answers:
 - Player B says "Yes, I have this card."
   - As in every other Quartets, he now has to hand over the card in question. Now we have more facts:
     - Player B now only has 3 cards
     - Player A now has the card **Animal - Tiger** and still has another **Animal** card, with undefined Expression
 - Player B says "No, I don't have that card."
   - Now we concluded other facts:
     - Both players still have 4 cards
     - Player A already has the card **Animal - Tiger** (The card exists and player B doesn't have it, so player A must have it)

The rest of the game follows this pattern, at some point a player can conclude, that he must have 4 of a kind.

## About this project

**Project is still in progress.**

This project will include the logic for **Quantum Quartets** as a library for multiple languages. This way it can be integrated in any application.
