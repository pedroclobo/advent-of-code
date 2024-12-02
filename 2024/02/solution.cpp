#include<iostream>
#include<vector>
#include<fstream>
#include<sstream>

class Report {
public:
  std::vector<int> levels;

  bool isSafe() const {
    bool increasing = true;
    bool decreasing = true;
    bool safe_diff = true;

    for (unsigned i = 0, e = levels.size() - 1; i < e; ++i) {
      int diff = abs(levels[i + 1] - levels[i]);
      increasing &= levels[i] <= levels[i + 1];
      decreasing &= levels[i] >= levels[i + 1];
      safe_diff &= 1 <= diff && diff <= 3;
    }

    return (increasing || decreasing) && safe_diff;
  }

  Report from(const Report &report, unsigned ignore) const {
    Report new_report;
    for (unsigned i = 0; i < levels.size(); ++i) {
      if (i == ignore)
        continue;
      new_report.levels.push_back(levels[i]);
    }
    return std::move(new_report);
  }

  friend std::ostream &operator<<(std::ostream &os, const Report &report) {
    for (const auto &num : report.levels)
      os << num << " ";
    return os;
  }

  Report() {
    levels = std::vector<int>();
  }
};

class Input {
public:
  std::vector<Report> reports;

  Input() {
    reports = std::vector<Report>();
  }

};

Input parse() {
  std::ifstream file;
  file.open("input.txt");

  Input input;

  std::string line;
  while (std::getline(file, line)) {
    std::istringstream iss(line);
    Report report;
    int num;

    while (iss >> num)
      report.levels.push_back(num);

    if (!report.levels.empty())
      input.reports.push_back(std::move(report));
  }

  return std::move(input);
}

unsigned part1(const Input &input) {
  unsigned count = 0;
  for (const auto &report : input.reports)
    if (report.isSafe())
      ++count;
  return count;
}

int part2(const Input &input) {
  unsigned count = 0;
  for (const auto &report : input.reports)
    for (unsigned i = 0; i < report.levels.size(); ++i) {
      auto new_report = report.from(report, i);
      if (new_report.isSafe()) {
        ++count;
        break;
      }
    }

  return count;
}

int main() {
  auto input = parse();

  std::cout << "The solution to part 1 is " << part1(input) << "." << std::endl;
  std::cout << "The solution to part 2 is " << part2(input) << "." << std::endl;

  return 0;
}
