file = open("input.txt")
text = file.read().splitlines()[0]
print("Day 2: Part one")

def valid(number):
    number = str(number)
    if len(number) % 2 != 0:
        return True
    n = len(number)
    firstHalf = number[:n//2]
    secondHalf = number[n//2:]
    return not(firstHalf == secondHalf)



def sum_invalid(low, high):
    out = 0 
    for x in range(low, high+1):
        if not valid(x):
            out += x
    return out


total = 0 

for interval in text.split(","):
    begin, end = interval.split("-")
    begin = int(begin)
    end = int(end)
    total += sum_invalid(begin, end)


print("Total: ", total)

