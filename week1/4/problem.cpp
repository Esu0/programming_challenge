#include<iostream>
using namespace std;

int main() {
    int n;
    cin >> n;
    int ans;
    if (n <= 3) {
        ans = 1;
    } else {
        ans = n - 2;
    }
    cout << ans << endl;
    return 0;
}