print("Day one advent of code 2025!")


file = open("input.txt")

puzzleInput = file.read().splitlines()
file.close()
pointing = 50
password = 0

for line in puzzleInput:
    line = line.replace("R", "").replace("L", "-")
    line = int(line)
    pointing += line

    pointing = pointing % 100
    if pointing == 0:
        password += 1


print("Password is: ", password)

