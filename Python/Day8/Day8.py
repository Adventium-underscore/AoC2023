import math


def getInput():
    network = {}
    starts = []
    with open("Day 8 Input.txt") as data:
        instructions = data.readline().strip()
        data.readline()
        for line in data:
            node, raw = line.split(" = ", 1)
            if node[2] == 'A':
                starts.append(node)
            l, r = raw.strip('()\n').split(", ", 1)
            network[node] = (l, r)
    return network, instructions, starts

def traverseNetwork(network, instructions):
    current = "AAA"
    i = 0
    while current != "ZZZ":
        d = 0 if instructions[i % len(instructions)] == "L" else 1
        current = network[current][d]
        i += 1
    return i

def findEnd(network, instructions, index, start):
    current = start
    while current[2] != "Z":
        d = 0 if instructions[index % len(instructions)] == "L" else 1
        current = network[current][d]
        index += 1
    return current, index

def findLoopLength(network, instructions, index, node):
    n = network[node][instructions[0] == "L"]
    return findEnd(network, instructions, index + 1, n)[1] - index

def main():
    network, instructions, starts = getInput()
    # print(traverseNetwork(network, instructions))
    ends = list(map(lambda n: findEnd(network, instructions, 0, n), starts))
    print(ends)
    # Not necessary because each loop has the same period as the length to get there,
    # so taking the lcm directly gives the answer
    # loops = list(map(lambda n: (n[0], n[1], findLoopLength(network, instructions, n[1], n[0])), ends))
    print(math.lcm(*map(lambda n: n[1], ends)))

if __name__ == "__main__":
    main()