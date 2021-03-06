N = int(input())
A = list(map(int, input().split()))

s = [0]
for i in range(N):
    s.append(s[i] + A[i])

ss = [0]
for i in range(N + 1):
    ss.append(ss[i] + s[i])

sm = [0]
for i in range(N + 1):
    sm.append(max(sm[i], s[i]))


sssm = [a + b for a, b in zip(ss[:-1], sm[1:])]
print(max(sssm))
