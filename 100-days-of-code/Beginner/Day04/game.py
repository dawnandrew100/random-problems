import random

rock = """
    _______
---'   ____)
      (_____)
      (_____)
      (____)
---.__(___)
"""

paper = """
     _______
---'    ____)____
           ______)
          _______)
         _______)
---.__________)
"""

scissors = """
    _______
---'   ____)____
          ______)
       __________)
      (____)
---.__(___)
"""

hand = {"scissors": scissors, "paper": paper, "rock": rock}


def main():
    print("Welcome to a game of Rock, Paper, Scissors.")
    player_res = loop_assign()
    matches = {"rock": "scissors", "scissors": "paper", "paper": "rock"}
    computer_res = random.choice(list(matches.values()))

    print(f"You played {player_res}\n{hand[player_res]}")
    print(f"The computer played {computer_res}\n{hand[computer_res]}")

    if player_res == computer_res:
        print("Draw")
    elif player_res == matches[computer_res]:
        print("You lose")
    else:
        print("You win!")


def loop_assign():
    options = {"r": "rock", "p": "paper", "s": "scissors"}
    while True:
        choice = input("Pick Rock, Paper, or Scissors: ")
        if choice in options:
            choice = options[choice]
        if choice in options.values():
            return choice


if __name__ == "__main__":
    main()
