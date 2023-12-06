import functools
import operator


def getInput():
    with open("Day 6 Input.txt") as data:
        # times = map(int, data.readline().strip().split()[1:])
        # distances = map(int, data.readline().strip().split()[1:])

        # Weird unneccesary data formatting specifically to be able to still run part 1
        times = [int(data.readline().split(':')[1].replace(' ', ''))]
        distances = [int(data.readline().split(':')[1].replace(' ', ''))]
        return list(zip(times, distances))

def countWins(race):
    time = race[0]
    record = race[1]
    lowerBound = 0
    for t in range(1, time):
        if t * (time - t) > record:
            lowerBound = t
            break
    upperBound = 0
    for t in range(time, lowerBound, -1):
        if t * (time - t) > record:
            upperBound = t
            break
    return upperBound - lowerBound + 1

def main():
    races = getInput()
    # score = functools.reduce(operator.mul, map(countWins, races))
    score = countWins(races[0])
    print(score)

if __name__ == "__main__":
    main()