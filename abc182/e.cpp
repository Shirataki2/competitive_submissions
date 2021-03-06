#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<ll,ll> pll;
typedef vector<ll> vll;
typedef vector<pll> vpll;
typedef vector<vll> vvll;

int main() {
    ll H, W, N, M;
    cin >> H >> W >> N >> M;
    vpll bulbs;
    vpll blocks;
    vvll m = vvll(H, vll(W, 0));
    for (int i = 0; i < N; i++) {
        ll a, b;
        cin >> a >> b;
        a--;b--;
        m[b][a] = 1;
        bulbs.push_back(make_pair(b, a));
    }
    for (int i = 0; i < M; i++) {
        ll a, b;
        cin >> a >> b;
        a--;b--;
        m[b][a] = -1;
        blocks.push_back(make_pair(b, a));
    }
    for(auto bulb: bulbs) {
        for(int dx = 1; dx < W; dx++) {
            if(dx < 0 || bulb.first + dx >= W) break; 
            if(m[bulb.first + dx][bulb.second] == -1) break;
            m[bulb.first + dx][bulb.second] = 1;
        }
        for(int dx = -1; dx > -W; dx--) {
            if(bulb.first + dx < 0 || dx >= W) break; 
            if(m[bulb.first + dx][bulb.second] == -1) break;
            m[bulb.first + dx][bulb.second] = 1;
        }
        for(int dy = 1; dy < H; dy++) {
            if(dy < 0 || bulb.second + dy >= H) break; 
            if(m[bulb.first][bulb.second + dy] == -1) break;
            m[bulb.first][bulb.second + dy] = 1;
        }
        for(int dy = -1; dy > -H; dy--) {
            if(bulb.second + dy < 0 || dy >= H) break; 
            if(m[bulb.first][bulb.second + dy] == -1) break;
            m[bulb.first][bulb.second + dy] = 1;
        }
    }
    ll ans = 0;
    for(int i = 0; i < W; i++) {
        for(int j = 0; j < H; j++) {
            if(m[i][j] == 1) ans++;
        }
    }
    cout << ans << endl;
}