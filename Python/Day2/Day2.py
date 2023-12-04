def getInput():
    with open("Day 2 Input.txt") as data:
        games = {}
        for line in data:
            game, hands = line.strip().split(": ", 1)
            red, green, blue = 0, 0, 0
            for hand in hands.split("; "):
                for cube in hand.split(", "):
                    n, c = cube.split(' ', 1)
                    n = int(n)
                    match c:
                        case "red": red = max(red, n)
                        case "green": green = max(green, n)
                        case "blue": blue = max(blue, n)
            # print(f"{game=}\n{hands=}\n{red=}    {green=}    {blue=}\n")
            games[int(game.split(' ')[1])] = (red, green, blue)
    return games

def checkLimits(games, red, green, blue):
    total = 0
    for id, game in games.items():
        if game[0] <= red and game[1] <= green and game[2] <= blue:
            total += id
    return total

def calcPower(games):
    total = 0
    for game in games.values():
        total += game[0] * game[1] * game[2]
    return total

def main():
    games = getInput()
    # total = checkLimits(games, 12, 13, 14)
    total = calcPower(games)
    print(total)

if __name__ == "__main__":
    main()