import numpy as np

file = open("input.txt")
lines = file.read().splitlines()

file.close()


vecs = []
for vec in lines:
    elements = vec.split(",")
    vecs.append(np.array(vec.split(","), dtype=float))


n = len(vecs)
A = np.zeros((n,n))


# we are considering distance squared (since the square does not matter for us)
def dist2(a, b):
    return np.dot(a - b, a - b)

for i in range(0, n):
    for j in range(0, n):
        A[i, j] = dist2(vecs[i], vecs[j])



distances = A.flatten()
distances = list(set(distances.tolist()))

distances.sort()
distances.remove(0.)
# okay lets hope that there were no two sets of two vectors that had the same difference!
assert len(distances) == (n**2 - n)/ 2

distances.reverse()
connections = []


def get_conn(dist):
    for i in range(n):
        for j in range(n):
            if A[i, j] == dist:
                return [min(i,j), max(i,j)]


def handle_add_to_connections(conn):
    conn = set(conn)
    for c in connections.copy():
        conn_iter = set(c)
        if not conn.isdisjoint(conn_iter):
            conn = conn.union(conn_iter)
            connections.remove(c)


    connections.append(list(conn))



for i in range(1000):
    dist = distances.pop()
    conn_to_add = get_conn(dist)
    handle_add_to_connections(conn_to_add)



lens = [len(x) for x in connections]
lens.sort()
lens.reverse()
print("The product is: ", lens[0] * lens[1] * lens[2])
