def getInput():
    with open("Day 9 Input.txt") as data:
        return [list(map(int, line.strip().split(' '))) for line in data]

def findDiffList(history):
    dl = []
    for i in range(len(history) - 1):
        dl.append(history[i+1] - history[i])
    return dl

def isZeroes(difflist):
    for d in difflist:
        if d != 0:
            return False
    return True

def main():
    oasis = getInput()
    total = 0
    for history in oasis:
        nextValue = history[-1]
        difflists = []
        dl = findDiffList(history)
        while not isZeroes(dl):
            nextValue += dl[-1]
            difflists.append(dl)
            dl = findDiffList(dl)
        prevValue = 0
        for dl in reversed(difflists):
            prevValue = dl[0] - prevValue
        prevValue = history[0] - prevValue
        # total += nextValue
        total += prevValue
    print(total)

if __name__ == "__main__":
    main()