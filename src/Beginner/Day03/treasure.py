def main():
    print(r'''
*******************************************************************************
          |                   |                  |                     |
 _________|________________.=""_;=.______________|_____________________|_______
|                   |  ,-"_,=""     `"=.|                  |
|___________________|__"=._o`"-._        `"=.______________|___________________
          |                `"=._o`"=._      _`"=._                     |
 _________|_____________________:=._o "=._."_.-="'"=.__________________|_______
|                   |    __.--" , ; `"=._o." ,-"""-._ ".   |
|___________________|_._"  ,. .` ` `` ,  `"-._"-._   ". '__|___________________
          |           |o`"=._` , "` `; .". ,  "-._"-._; ;              |
 _________|___________| ;`-.o`"=._; ." ` '`."\` . "-._ /_______________|_______
|                   | |o;    `"-.o`"=._``  '` " ,__.--o;   |
|___________________|_| ;     (#) `-.o `"=.`_.--"_o.-; ;___|___________________
____/______/______/___|o;._    "      `".o|o_.--"    ;o;____/______/______/____
/______/______/______/_"=._o--._        ; | ;        ; ;/______/______/______/_
____/______/______/______/__"=._o--._   ;o|o;     _._;o;____/______/______/____
/______/______/______/______/____"=._o._; | ;_.--"o.--"_/______/______/______/_
____/______/______/______/______/_____"=.o|o_.--""___/______/______/______/____
/______/______/______/______/______/______/______/______/______/______/[TomekK]
*******************************************************************************
          ''')
    print("Welcome to Treasure Island.\nYour mission is to find the treasure.")
    game()


def game():
    res = input(
        'You\'re at a crossroad. Where do you want to go? Type "left" or "right". '
    )
    if res.lower() != "left":
        print("You fell into a hole! Game over")
        return

    print(
        "You've come to a lake. There is an island in the middle of the lake.", end=" "
    )
    res = input('Type "wait" to wait for a boat. Type "swim" to swim across. ')
    if res.lower() != "wait":
        print("You got attacked by an angry trout. Game over.")
        return

    print(
        "You arrive at the island unharmed. There is a house with 3 doors."
        "One red, one yellow, and one blue."
    )
    res = input("Which colour do you choose? ").lower()
    if res == "red":
        print("It's a room full  of fire. Game over.")
    elif res == "yellow":
        print("You found the treasure! You win!")
    elif res == "blue":
        print("You enter a room of beasts. Game over.")
    else:
        print("You chose a door that doesn't exist. Game over.")
    return


if __name__ == "__main__":
    main()
