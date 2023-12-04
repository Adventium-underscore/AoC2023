import re


def getInput():
    p = re.compile("\d+")
    cards = []
    with open("Day 4 Input.txt") as data:
        for line in data:
            winning, nums = line.strip().split(": ", 1)[1].split(" | ", 1)
            winning = set(map(int, p.findall(winning)))
            nums = set(map(int, p.findall(nums)))
            cards.append([winning, nums, 1])
    return cards

def scoreCard(card):
    matches = card[0].intersection(card[1])
    return 0 if len(matches) == 0 else 2 ** (len(matches) - 1)

def main():
    cards = getInput()
    # total = sum(map(scoreCard, cards))
    # print(total)
    for i, card in enumerate(cards):
        wins = len(card[0].intersection(card[1]))
        for x in range(wins):
            cards[i+x+1][2] += card[2]
    total = sum(map(lambda c: c[2], cards))
    print(total)

if __name__ == "__main__":
    main()