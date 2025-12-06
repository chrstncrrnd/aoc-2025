file = open("input.txt")
text = file.read().splitlines()[0]
print("Day 2, part two")
file.close()
def valid(number):
    number = str(number)
    n = len(number)
    for i in range(1, n//2 + 1):
        if n % i == 0:
            slices = [number[x:x+i] for x in range(0, n, i)]
            if len(set(slices)) == 1:
                return False
    return True




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

