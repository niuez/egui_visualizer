#include <iostream>
#include <fstream>
#include <random>
#include <format>
#include <algorithm>
#include "../frame.hpp"

int main() {
  std::mt19937 mt(768);
  int N = 100;
  std::vector<std::pair<float, float>> v(N);
  const float MAX_X = 20;
  for(int i = 0; i < N; i++) {
    v[i].first = std::uniform_real_distribution<float>(0, MAX_X)(mt);
    v[i].second = std::uniform_real_distribution<float>(0, MAX_X)(mt);
  }

  std::vector<int> perm(N);
  std::iota(perm.begin(), perm.end(), 0);

  auto draw = [&]() {
    using frame::pos2;
    std::cout << frame::new_frame(pos2(-1, -1), pos2(MAX_X + 1, MAX_X + 1));
    for(int i = 0; i < perm.size(); i++) {
      pos2 p(v[perm[i]].first, v[perm[i]].second);
      pos2 q(v[perm[(i + 1) % N]].first, v[perm[(i + 1) % N]].second);
      frame::path path;
      path.add(p);
      path.add(q);
      std::cout << path;
    }
    for(int i = 0; i < v.size(); i++) {
      pos2 p(v[i].first, v[i].second);
      float m = 0.1;
      std::cout << frame::rect {
        .msg = std::to_string(i),
          .p1 = pos2(p.x - m, p.y - m),
          .p2 = pos2(p.x + m, p.y + m),
      };
    }
  };
  draw();

  auto dist = [&](int i, int j) {
    float x = v[i].first - v[j].first;
    float y = v[i].second - v[j].second;
    return std::sqrt(x * x + y * y);
  };

  auto sum = [&]() {
    float ans = 0;
    for(int i = 0; i < N; i++) {
      ans += dist(perm[i], perm[(i + 1) % N]);
    }
    return ans;
  };

  for(int it = 0; it < 100; it++) {
    bool shrunk = false;
    std::cerr << it << std::endl;
    for(int i = 0; i < N; i++) {
      for(int l = 2; l <= N - 2; l++) {
        int j = (i + l) % N;
        float diff =
          + dist(perm[i], perm[(i - 1 + N) % N]) + dist(perm[j], perm[(j - 1 + N) % N])
          - dist(perm[i], perm[j]) - dist(perm[(i - 1 + N) % N], perm[(j - 1 + N) % N]);
        if(diff >= 1e-9) {
          shrunk |= diff > 0;
          if(i > 0) 
            std::rotate(perm.begin(), perm.begin() + i, perm.end());
          std::reverse(perm.begin() + 0, perm.begin() + l);
          if(i > 0) 
            std::rotate(perm.begin(), perm.begin() + (N - i), perm.end());
          draw();
        }
      }
    }
    if(!shrunk) break;
  }
}
