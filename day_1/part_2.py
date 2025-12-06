print("Day one advent of code 2025 (part two)!")


file = open("input.txt")

puzzleInput = file.read().splitlines()
file.close()

pointing = 50
password = 0
def timesCrossed(start, end):
    if start == 0 and end < 0:
        return abs(end // 100) - 1
    if end == 0:
        return 1
    if end < 0 and end % 100 == 0:
        return abs(end // 100) + 1
    return abs(end // 100)




for line in puzzleInput:
    line = line.replace("R", "").replace("L", "-")
    line = int(line)
    password += timesCrossed(pointing, pointing + line)
    pointing += line
    pointing = pointing % 100


print("Password is: ", password)

