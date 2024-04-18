#include<iostream>
#include<vector>
using namespace std;

int main() {
    int x, n;
    cin >> x >> n;
    int p_sum = 0;
    for (int i = 0; i < n; i++) {
        int pi;
        cin >> pi;
        p_sum += pi;
    }
    cout << x * (n + 1) - p_sum << endl;
    return 0;
}