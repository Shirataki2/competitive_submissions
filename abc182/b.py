N = input()
A = list(map(int, input().split()))

gi = 0
g = -1
for n in range(2, 1001):
    aa = [i % n == 0 for i in A]
    if sum(aa) > gi:
        g = n
        gi = sum(aa)
print(g)