import re


def main():
    p = re.compile('(?=(\d|one|two|three|four|five|six|seven|eight|nine)).')
    toInt = {"one":1, "two":2, "three":3, "four":4, "five":5, "six":6, "seven":7, "eight":8, "nine":9}

    calibration = 0
    with open("Day 1 Input.txt") as data:
        for line in data:
            raw = p.findall(line)
            tens, ones = raw[0], raw[-1]
            calibration += 10*(int(tens) if tens.isdigit() else toInt[tens]) + (int(ones) if ones.isdigit() else toInt[ones])

    print(calibration)

if __name__ == "__main__":
    main()