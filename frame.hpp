#include <iostream>
#include <vector>
#include <utility>
#include <array>

namespace frame {
  struct pos2 {
    float x;
    float y;
    pos2(float x = 0, float y = 0): x(x), y(y) {}
  };

  std::ostream& operator<<(std::ostream& os, const pos2& p) {
    return os << std::fixed << "(" << p.x << "," << p.y << ")";
  }

  struct new_frame {
    pos2 min;
    pos2 max;
    new_frame(pos2 min, pos2 max): min(min), max(max) {}
  };

  std::ostream& operator<<(std::ostream& os, const new_frame& f) {
    return os << "# " << f.min << " " << f.max;
  }

  struct message {
    std::string msg;
    message(std::string m = ""): msg(m) {}
  };

  std::ostream& operator<<(std::ostream& os, const message& msg) {
    return os << "{{" << msg.msg << "}}";
  }

  struct tag_color {
    int idx;
    tag_color(int i): idx(i) {}
  };

  std::ostream& operator<<(std::ostream& os, const tag_color& c) {
    return os << "tag(" << c.idx << ")";
  }

  struct path {
    std::vector<pos2> ps;
    void add(pos2 p) {
      ps.push_back(p);
    }
  };

  std::ostream& operator<<(std::ostream& os, const path& f) {
    os << "p " << "[";
    for(int i = 0; i < f.ps.size(); i++) {
      os << f.ps[i] << ",]"[i + 1 == f.ps.size()];
    }
    return os;
  }

  struct rect {
    pos2 p1;
    pos2 p2;
  };

  std::ostream& operator<<(std::ostream& os, const rect& f) {
    return os << "r " << f.p1 << " " << f.p2;
  }
}
