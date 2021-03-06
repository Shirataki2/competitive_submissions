def f(l):
    if l == 0: return 1
    return l * f(l-1)

def c(n, r):
    return f(n) // f(r) // f(n - r)

l = int(input())

print(c(l-1, 11))
