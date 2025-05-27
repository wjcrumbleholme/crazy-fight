# Types of card
- Weapon
- Item
- Battle Card
- Power up
- Person
- Super person

# Class structure
- Cards
- FIELD: CARDID
- FIELD: IMGSRC
- METHOD: on_play
    - Item Card
        - Weapon
        - Battle card
        - General item
        - Event card
    - People Card
    - FIELD: POWERUP_CARD
    - FIELD: SUPER_CARD
        - Super person (if avaliable)
        - Power up

- Have all cards as json files and have functions that they can call

# Game structure
- Draw pile
    - List of card numbers
    - Current round draw pile
    - Past draw pile

- Player
    - Frozen status
        - Whether they have just called a fight
        - If they have gotten frozen by others
    - Hand
        - List of card objects not played
        - List of card objects played on table
    
- Rounds
    - At the end of each round, each player has to press a button
    - When round 5 is reached, a fight can be called, the round continues and at the end a fight commences - the player that calls that fight cannot place anymore cards

- Fight round
    - When in a fight round, all wepons need to be placed, and only battle cards can be played
    - The player that takes the most damage from another player looses (this is mainly the person with the lowest score but potions can save people)

# Card list
- General items
    - Freeze card: Make another player unable to play the next round
    - People picker: Pick a character out of the character draw pile
    - People chooser: Pick a specific character out of the draw pile
    - Draw potion: Draw 1 more card next round
- Battle items
    - Heal potion: Take 10 less damage from a player
    - Battle potion: Inflict 10 more damage on a player
- Power up
    - Self destruct: Add to a card to destroy it and the lowest damage card next battle
- Event 
    - Golf: Immediately call a battle, the player with the largest score looses instead




# Managers
- GameManager
    - DeckManager: Handles the different piles of cards + the loading of a deck
    - CardManager: Handles all of the cards and their abilities
    - PlayerManager: Handles all players and any debuffs they may have


# To load a deck in:
- First parse all of the cards
- Then add them all to the deck manager