#include<iostream>
#include<vector>
using namespace std;

int main() {
    float sum = 0.0;
    int n;
    cin >> n;
    for (int i = 0; i < n; i++) {
        float q, y;
        cin >> q >> y;
        sum += q * y;
    }
    cout << sum << endl;
    return 0;
}