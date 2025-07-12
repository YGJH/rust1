#include <bits/stdc++.h>
using namespace std;
using ll = long long;

int main(){
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n, m, a, b;
    cin >> n >> m >> a >> b;
    ll g0, x, y, z;
    cin >> g0 >> x >> y >> z;

    int cols = m - b + 1;
    // 存放每一行滑窗後的最小值：共 n 行，cols 列
    vector<int> row_min(n * cols);

    // 生成高度並對每行做寬度為 b 的滑窗最小值
    deque<int> dq;
    ll g = g0;
    vector<int> curRow(m);
    for(int i = 0; i < n; i++){
        // 生成第 i 行的 m 個高度
        for(int j = 0; j < m; j++){
            if(i == 0 && j == 0){
                curRow[j] = g;
            } else {
                g = (g * x + y) % z;
                curRow[j] = g;
            }
        }
        // 對 curRow 做寬度 b 的滑動最小值
        dq.clear();
        for(int j = 0; j < m; j++){
            // 把新的元素入隊，維護單調遞增
            while(!dq.empty() && curRow[dq.back()] >= curRow[j])
                dq.pop_back();
            dq.push_back(j);
            // 如果隊首已經離開視窗，就彈出
            if(dq.front() <= j - b) 
                dq.pop_front();
            // 當 j >= b-1 時，取得一個最小值
            if(j >= b-1){
                row_min[i * cols + (j - b + 1)] = curRow[dq.front()];
            }
        }
    }

    // 對中間矩陣每一列再做高度為 a 的滑窗最小值，並累加
    ll answer = 0;
    for(int j = 0; j < cols; j++){
        dq.clear();
        // 遍歷第 j 列的所有 row_min 值
        for(int i = 0; i < n; i++) {
            int val = row_min[i * cols + j];
            while(!dq.empty() && 
                  row_min[dq.back() * cols + j] >= val)
                dq.pop_back();
            dq.push_back(i);
            if(dq.front() <= i - a)
                dq.pop_front();
            if(i >= a - 1){
                answer += row_min[dq.front() * cols + j];
            }
        }
    }

    cout << answer << "\n";
    return 0;
}
