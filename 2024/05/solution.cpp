#include<cassert>
#include<iostream>
#include<vector>
#include<fstream>
#include<sstream>

class Ordering {
public:
  int first, second;

  Ordering(int first, int second) : first(first), second(second) {}
};

class Update {
public:
  std::vector<int> values;

  Update() {
    values = std::vector<int>();
  }

  int middle_value() const {
    assert(values.size() != 0);
    return values[values.size() / 2];
  }
};

class Input {
public:
  std::vector<Update> updates;
  std::vector<Ordering> orderings;

  Input() {
    updates = std::vector<Update>();
    orderings = std::vector<Ordering>();
  }

  bool isBefore(int first, int second) const {
    for (const auto &ordering : orderings)
      if (ordering.first == second && ordering.second == first)
        return false;
    return true;
  }

  bool valid(const Update &update) const {
    for (int i = 0; i < update.values.size(); ++i)
      for (int j = i + 1; j < update.values.size(); ++j)
        if (!isBefore(update.values[i], update.values[j]))
          return false;
    return true;
  }

  void fix(Update &update) {
    while (!valid(update))
      for (int i = 0; i < update.values.size(); ++i)
        for (int j = i + 1; j < update.values.size(); ++j)
          if (!isBefore(update.values[i], update.values[j]))
            std::swap(update.values[i], update.values[j]);
  }

  static Input parse() {
    std::ifstream file("input.txt");
    Input input;

    // orderings
    while (file) {
      std::string line;
      std::getline(file, line);
      size_t pipe = line.find('|');
      if (pipe == std::string::npos) break;
      int first = std::stoi(line.substr(0, pipe));
      int second = std::stoi(line.substr(pipe + 1));
      input.orderings.push_back(Ordering(first, second));
    }

    // updates
    while (file) {
      std::string line;
      std::getline(file, line);
      std::stringstream ss(line);
      std::string token;
      Update update;
      while (std::getline(ss, token, ','))
        update.values.push_back(std::stoi(token));
      if (update.values.size() != 0)
        input.updates.push_back(update);
    }

    return input;
  }
};

int part1(const Input &input) {
  unsigned sum = 0;
  for (const auto &update : input.updates)
    if (input.valid(update))
      sum += update.middle_value();
  return sum;
}

int part2(Input &input) {
  unsigned sum = 0;
  for (auto &update : input.updates) {
    if (!input.valid(update)) {
      input.fix(update);
      sum += update.middle_value();
    }
  }
  return sum;
}

int main() {
  auto input = Input::parse();
  std::cout << "The solution to part 1 is " << part1(input) << "." << std::endl;
  std::cout << "The solution to part 2 is " << part2(input) << "." << std::endl;

  return 0;
}
