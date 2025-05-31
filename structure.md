# Managers

## DeckManager
- Should handle all of the draw piles but only hold the card_id

## CardManager
- Should handle the loading of cards in and store a 'registry'
- Should have a list of all cards in play (ones with an instance id)

## PlayerManager
- Should have a list of player objects

    ### Player
    - Should not hold actual card objects, just their instance id's

## EventManager
- Should handle any incoming events (CardPlayed, NextRound)
- Handles the drawing of a card

## GameManager
- Handles the loading of a deck
- Handles the creation of players




# Important funtions

## ResolveId (Target) 
- Will take in the current player id and card instance id, imut ref to CardManager and PlayerManager
    - In the type, it could be player, hand, addon, character
    - If the owner or card field is self, then just return the id
    - If either is choose, then prompt the player to choose a target and return that id
    - If either is random, then choose a random player/card to affect
    - If for the card, it is attached_card then get look through the cardmanager for the card and try to get its attached card instance id

## Condition.is_met
- Will take in a condition, the player_id and the instance_id and a imut ref to CardManager, PlayerManager and GameState
