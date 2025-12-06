file = open("input.txt")



def prod(l):
    out = 1
    for x in l:
        out *= x
    return out

lines = file.read().splitlines()[:-1]

file.close()


total = 0
string = ""
for x in range(len(lines[0]) - 1, -1, -1):
    for y in range(len(lines)):
        string += lines[y][x]


buffer = []
operator = None
for num in string.split():
    if num == "":
        continue
    if num[-1] in ["+", "*"]:
        operator = num[-1]
        num = num[:-1]
    if num != "":
        num = int(num)
        buffer.append(num)
    if not operator is None:
        if operator == "+":
            total += sum(buffer)
        else:
            total += prod(buffer)
        operator = None
        buffer = []

print("Total = ", total)



