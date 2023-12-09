from enum import IntEnum

# Part 1
# cardOrder = {"A": 14, "K": 13, "Q": 12, "J": 11, "T": 10, "9": 9, "8": 8, "7": 7, "6": 6, "5": 5, "4": 4, "3": 3, "2": 2}
# Part 2
cardOrder = {"A": 14, "K": 13, "Q": 12, "T": 10, "9": 9, "8": 8, "7": 7, "6": 6, "5": 5, "4": 4, "3": 3, "2": 2, "J": 1}

class HandType(IntEnum):
    HIGHCARD = 1
    ONEPAIR = 2
    TWOPAIR = 3
    THREEOFAKIND = 4
    FULLHOUSE = 5
    FOUROFAKIND = 6
    FIVEOFAKIND = 7

    @classmethod
    def findType(cls, hand: str):
        counts = []
        jokers = 0
        while len(hand) > 0:
            c = hand[0]
            if c == "J":
                jokers = hand.count(c)
            else:
                counts.append(hand.count(c))
            hand = hand.replace(c, "")
        if len(counts) == 0:
            counts = [5]
        else:
            counts.sort()
            counts[-1] += jokers
        match counts:
            case [1,1,1,1,1]: return cls.HIGHCARD
            case [1,1,1,2]: return cls.ONEPAIR
            case [1,2,2]: return cls.TWOPAIR
            case [1,1,3]: return cls.THREEOFAKIND
            case [2,3]: return cls.FULLHOUSE
            case [1,4]: return cls.FOUROFAKIND
            case [5]: return cls.FIVEOFAKIND

class Hand:
    hand = ""
    handtype = None
    bid = 0

    def __init__(self, hand, bid):
        self.hand = hand
        self.handtype = HandType.findType(hand)
        self.bid = bid

    def __lt__(self, other):
        if self.handtype == other.handtype:
            for c1, c2 in zip(self.hand, other.hand):
                if cardOrder[c1] != cardOrder[c2]:
                    return cardOrder[c1] < cardOrder[c2]
            # hands are identical, so self is not less than other
            return False
        else:
            return self.handtype < other.handtype


def getInput():
    hands = []
    with open("Day 7 Input.txt") as data:
        for line in data:
            hand, bid = line.split(' ', 1)
            hands.append(Hand(hand, int(bid)))
    return hands

def main():
    hands = getInput()
    hands.sort()
    # for hand in hands: print(f"{hand.hand}, {hand.handtype}, {hand.bid}")
    winnings = 0
    for i, hand in enumerate(hands):
        winnings += hand.bid * (i + 1)
    print(winnings)

if __name__ == "__main__":
    main()