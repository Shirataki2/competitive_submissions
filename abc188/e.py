from dataclasses import dataclass

@dataclass
class Edge:
    to: int
    weight: int

class Graph:
    def __init__(self, N, path):
        self.N = N
        self.path = path
        self.inf = (N, None)
        G = [[] for _ in range(N)]
        for p in path:
            u, v, w = p
            G[u].append(Edge(to=v, weight=w))
        self.G = G
        self.S = []
        self.D = []
        self.F = [0 for _ in range(N)]
        self.depth = [0 for _ in range(N)]
        self._dfs(Edge(to=0, weight=0), 0, 0)
        self._build()

    def _build(self):
        M = self.N * 2
        M0 = 1
        while M0 < M:
            M0 *= 2
        self.M0 = M0
        self.data = [self.inf for _ in range(2*M0)]
        for i, v in enumerate(self.S):
            self.data[M0-1+i] = (self.depth[v], i)
        for i in range(M0 - 2, -1, -1):
            self.data[i] = min(self.data[2*i+1], self.data[2*i+2])

    def _dfs(self, e, d, w):
        self.F[e.to] = len(self.S)
        self.depth[e.to] = d
        self.S.append(e.to)
        self.D.append(w)
        for v in self.G[e.to]:
            self._dfs(v, d+1, w+v.weight)
            self.S.append(e.to)
            self.D.append(w)

    def _lca(self, i, j):
        yield self.inf
        i += self.M0
        j += self.M0
        while i < j:
            if j & 1:
                j -= 1
                yield self.data[j - 1]
            if i & 1:
                yield self.data[i - 1]
                i += 1
            i >>= 1
            j >>= 1

    def lca(self, i, j):
        fi = self.F[i]
        fj = self.F[j]
        if fi > fj:
            fi, fj = fj, fi
        lca = min(self._lca(fi, fj + 1))[1]
        return self.D[fi] + self.D[fj] - 2 * self.D[lca]

n, m = list(map(int, input().split()))
a = list(map(int, input().split()))
pathes = []
for i in range(m):
    p, q = list(map(int, input().split()))
    p -= 1
    q -= 1
    pathes.append((p, q, a[q] - a[p]))
g = Graph(n, pathes)

print(g.data)