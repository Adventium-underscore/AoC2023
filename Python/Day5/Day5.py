from dataclasses import dataclass

@dataclass
class SeedGroup:
    lowerBound: int
    upperBound: int

@dataclass
class Mapping:
    lowerBound: int
    upperBound: int
    offset: int

    def contains(self, num):
        return self.lowerBound <= num <= self.upperBound

def getInput():
    seeds = []
    mapsets = []
    with open("Day 5 Input.txt") as data:
        nums = list(map(int, data.readline().split(' ')[1:]))
        for i in range(len(nums) // 2):
            upperBound = nums[2*i] + nums[2*i+1] - 1
            seeds.append(SeedGroup(nums[2*i], upperBound))
        data.readline()

        maps = []
        for line in data:
            if line == "\n":
                mapsets.append(maps)
                maps = []
                continue
            values = line.split(' ')
            if len(values) == 3:
                startD, startS, length = map(int, values)
                upperBound = startS + length - 1
                offset = startD - startS
                maps.append(Mapping(startS, upperBound, offset))
        mapsets.append(maps)
    return seeds, mapsets

def advanceSeeds(maps: list[Mapping], seedgroups: list[SeedGroup]):
    newSeeds = []
    while len(seedgroups) > 0:
        seeds = seedgroups.pop(0)
        for m in maps:
            if m.contains(seeds.lowerBound):
                if m.contains(seeds.upperBound):
                    newSeeds.append(SeedGroup(seeds.lowerBound + m.offset, seeds.upperBound + m.offset))
                else:
                    newSeeds.append(SeedGroup(seeds.lowerBound + m.offset, m.upperBound + m.offset))
                    seedgroups.append(SeedGroup(m.upperBound + 1, seeds.upperBound))
                break
            elif m.contains(seeds.upperBound):
                newSeeds.append(SeedGroup(m.lowerBound + m.offset, seeds.upperBound + m.offset))
                seedgroups.append(SeedGroup(seeds.lowerBound, m.lowerBound - 1))
                break
        else:
            newSeeds.append(seeds)

    return newSeeds

def main():
    seeds, mapsets = getInput()
    for maps in mapsets:
        seeds = advanceSeeds(maps, seeds)
    lowest = seeds[0].lowerBound
    for seed in seeds:
        lowest = min(lowest, seed.lowerBound)
    print(lowest)

if __name__ == "__main__":
    main()