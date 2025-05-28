# Card types
- Character
- SuperCharacter
- Addon
- Item
- BattleItem
- Weapon


# Common to all
- name: the name of the card
- id: the id of the card
- card_type: the type of the card (this also changes this file format)
- img_path: the image path to the card relative to this directory (just put a file name here)
- description: the description of the card
- play_time: when the card is able to be played


# Common to Item, Addon, BattleItem, Character
- ability 
    - ability_type: what kind it is (passive / one_time / response)
    - trigger: what triggers the cards ability (on_uncover (addon cards), on_click, etc)
    - conditions: Any conditions that must be met in order to play the card
    - effects:
        - action: the action the effect has
        - status: (optional) specify the staus for the action
        - amount: (optional) Specify parameters for the action
        - conditions: Any conditions that have to be met for this ability to work
        - destination_target: what the addon affects
            - type: The type to target (could be an actual player e.g damage potion) - this decides the next bit
                - IF PLAYER IS IN TYPE:
                - owner: which player to target (all, choose, self, random)

                - IF CHARACTER IS IN TYPE:
                - owner: which player to target (self, choose, random)
                - card: which card to transfer (choose, all, attached_card, weakest_card, random)

        - source_target: similar to destination_target but only used for certain cards 
        - duration: how long the effect should last for (round, etc)
            

## Specific for cards with ability type response
        - response_action: action to respond with
        - response_target: target to respond to (incoming_card, etc)

## Specific to Addon
- reveal_time: When to reveal the addon (on_steal, start_of_battle, etc)

## Specific to SuperCharacter
- additional_damage:

## Specific to character
- damage: How much base damage this card does
- super_char_id: Id of the super_character if there is one

## Weapon
- damage: How much base damage this card does
- synergy_card_id: Id of the character card that gives this damage
- synergy_damage: How much more damage this synergy does