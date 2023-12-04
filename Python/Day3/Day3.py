import re
import string

p1 = re.compile("\d+")
p2 = re.compile("\*")

def getInput():
    # add a layer of dots on every side to avoid needing to check bounds
    schematic = []
    with open("Day 3 Input.txt") as data:
        for line in data:
            schematic.append('.' + line.strip() + '.')
    l = len(schematic[0])
    schematic.insert(0, '.' * l)
    schematic.append('.' * l)
    return schematic

def isPartNum(schematic, num, y, span):
    # print(f"{num=}, {y=}, {span=}")
    if schematic[y][span[0] - 1] != '.': return True
    if schematic[y][span[1]] != '.': return True
    for x in range(span[0] - 1, span[1] + 1):
        if schematic[y-1][x] != '.': return True
    for x in range(span[0] - 1, span[1] + 1):
        if schematic[y+1][x] != '.': return True
    return False

def getNum(line, x):
    # print(f"{line=}, {x=}")
    for m in p1.finditer(line):
        if m.span()[0] <= x <= m.span()[1]:
            # print(f"num={m[0]}")
            return int(m[0])

def findRatio(schematic, x, y):
    # print(f"Gear at {x=}, {y=}")
    nums = set()
    for yi in range(y-1, y+2):
        for xi in range(x-1, x+2):
            if schematic[yi][xi] in string.digits: nums.add(getNum(schematic[yi], xi))
    if len(nums) == 2:
        return nums.pop() * nums.pop()

def main():
    schematic = getInput()
    total = 0
    # for line in schematic: print(line)
    for y, line in enumerate(schematic):
        # for m in p1.finditer(line):
        #     num = int(m[0])
        #     if isPartNum(schematic, num, y, m.span()):
        #         total += num
        for m in p2.finditer(line):
            ratio = findRatio(schematic, m.span()[0], y)
            if ratio is not None:
                total += ratio
    print(total)

if __name__ == "__main__":
    main()