#include <bits/stdc++.h>
using namespace std;


struct solver {
    int n ;
    solver() {
        n = 300;
        cout << n << " add(n): " << add(n) << endl;
    }

    int add(int n) {
        return n + n ;
    }
    ~solver() {};
};


signed main() {
    solver solve;
    return 0;
}