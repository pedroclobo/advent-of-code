#include<cassert>
#include<iostream>
#include<cstdint>
#include<fstream>
#include<sys/types.h>
#include<vector>
#include<stdint.h>
#include<regex>
#include"z3++.h"

struct Machine {
  std::pair<uint64_t, uint64_t> a;
  std::pair<uint64_t, uint64_t> b;
  std::pair<uint64_t, uint64_t> prize;

  /*
    * a.x * a_pushes + b.x * b_pushes = prize.x
    * a.y * a_pushes + b.y * b_pushes = prize.y
    * minimize: 3 * a_pushes + b_pushes
  */
  uint64_t solve(bool p2 = false) const {
    z3::context c;

    z3::expr a_pushes = c.int_const("a_pushes");
    z3::expr b_pushes = c.int_const("b_pushes");

    uint64_t prize_x = prize.first;
    uint64_t prize_y = prize.second;
    if (p2) {
      prize_x += 10000000000000;
      prize_y += 10000000000000;
    }

    z3::expr eq1 = c.int_val(a.first) * a_pushes +
                   c.int_val(b.first) * b_pushes == c.int_val(prize_x);
    z3::expr eq2 = c.int_val(a.second) * a_pushes +
                   c.int_val(b.second) * b_pushes == c.int_val(prize_y);

    z3::optimize opt(c);
    opt.add(eq1);
    opt.add(eq2);
    if (!p2) {
      opt.add(a_pushes <= 100);
      opt.add(b_pushes <= 100);
    }

    z3::expr obj = 3 * a_pushes + b_pushes;
    opt.minimize(obj);

    if (opt.check() != z3::sat) return 0;

    z3::model model = opt.get_model();
    return 3 * model.eval(a_pushes).get_numeral_uint64()
             + model.eval(b_pushes).get_numeral_uint64();
  }
};

struct Input {
  std::vector<Machine> machines;

  static Input parse() {
    std::regex digit(R"(\d+)");
    std::ifstream file("input.txt");
    std::vector<Machine> machines;

    std::string line;
    while (std::getline(file, line)) {
      if (line.empty()) continue;

      Machine machine;

      // a
      assert(line.substr(0, 10) == "Button A: ");
      auto i = std::sregex_iterator(line.begin(), line.end(), digit);
      machine.a = {std::stoi((i++)->str()), std::stoi((i++)->str())};

      // b
      std::getline(file, line);
      assert(line.substr(0, 10) == "Button B: ");
      i = std::sregex_iterator(line.begin(), line.end(), digit);
      machine.b = {std::stoi((i++)->str()), std::stoi((i++)->str())};

      // prize
      std::getline(file, line);
      assert(line.substr(0, 7) == "Prize: ");
      i = std::sregex_iterator(line.begin(), line.end(), digit);
      machine.prize = {std::stoi((i++)->str()), std::stoi((i++)->str())};

      machines.push_back(machine);
    }

    return {machines};
  }
};

uint64_t part1(const Input &input) {
  uint64_t total = 0;
  for (const auto &machine : input.machines)
    total += machine.solve();
  return total;
}

uint64_t part2(const Input &input) {
  uint64_t total = 0;
  for (const auto &machine : input.machines)
    total += machine.solve(true);
  return total;
}

int main() {
  auto input = Input::parse();

  std::cout << "The solution to part 1 is " << part1(input) << "." << std::endl;
  std::cout << "The solution to part 2 is " << part2(input) << "." << std::endl;

  return 0;
}
