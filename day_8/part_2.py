import numpy as np

class Distance:
    distance: int
    vec_i: int
    vec_j: int

    def __init__(self, distance, i, j):
        self.distance = distance
        self.vec_i = i
        self.vec_j = j


file = open("input.txt")
lines = file.read().splitlines()

file.close()


vecs = []
for vec in lines:
    elements = vec.split(",")
    vecs.append(np.array(vec.split(","), dtype=float))


n = len(vecs)
A = []


# we are considering distance squared (since the square does not matter for us)
def dist2(a, b):
    return np.dot(a - b, a - b)

for i in range(0, n):
    for j in range(0, i):
        A.append(Distance(dist2(vecs[i], vecs[j]), i, j))


distances = sorted(A, key=lambda x: x.distance)
# okay lets hope that there were no two sets of two vectors that had the same difference!
assert len(distances) == (n**2 - n)/ 2

distances.reverse()
connections = []


def conn_print():
    for c in connections:
        print([vecs[a].tolist() for a in c])

def handle_add_to_connections(conn):
    times = 0
    conn = set(conn)
    for c in connections.copy():
        conn_iter = set(c)
        if not conn.isdisjoint(conn_iter):
            conn = conn.union(conn_iter)
            connections.remove(c)
            times += 1
            if times == 2:
                break


    connections.append(list(conn))

out = 1

for i in range(len(distances)):
    print((i/1000) * 100, "%")
    dist = distances.pop()
    print(dist.distance)
    conn_to_add = [dist.vec_i, dist.vec_j]
    handle_add_to_connections(conn_to_add)
    if len(connections) == 1:
        if len(connections[0]) == n:
            out = vecs[conn_to_add[0]][0] * vecs[conn_to_add[1]][0]
            break;


print("The product is: ", out)
