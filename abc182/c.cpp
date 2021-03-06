#include <bits/stdc++.h>
using namespace std;


int main() {
    string N;
    cin >> N;
    int ans = 999;
    int del = 0;
    for (int i = 1; i < 1 << N.size(); i++) {
        int sum = 0;
        del = 0;
        for (int j = 0; j < N.size(); j++) {
            if (i >> j & 1) {
                sum += N[j] - '0';
            } else {
                del++;
            }
        }
        if(sum % 3 == 0 && sum > 0) {
            ans = min(ans, del);
        }
    }
    cout << (ans == 999 ? -1 : ans) << endl;
}