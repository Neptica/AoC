#include <fstream>
#include <iostream>
#include <string>
#include <vector>

std::vector<int> split_line(std::string line) {
  std::vector<int> res;
  int num = 0;
  for (int i = 0; i < line.size(); i++) {
    if (line[i] == ' ') {
      res.push_back(num);
      num = 0;
    } else {
      num = num * 10 + (line[i] - '0');
    }
  }
  res.push_back(num);
  return res;
}

bool is_safe(std::vector<int> &report) {
  // checks basic safe without dampening.
  int incf, decf, df;
  incf = decf = df = 1;
  for (int i = 0; i < report.size() - 1; i++) {
    int diff = std::abs(report[i] - report[i + 1]);
    decf = decf && (report[i] > report[i + 1]);
    incf = incf && (report[i] < report[i + 1]);
    df = df && (diff >= 1 && diff <= 3);
    if (!((incf || decf) && df))
      return false;
  }
  return true;
}

bool remove_and_check(std::vector<int> &report, int ind) {
  bool rv = false;
  int temp = report[ind];
  report.erase(report.begin() + ind);
  if (is_safe(report))
    rv = true;
  report.insert(report.begin() + ind, temp);
  return rv;
}

int main() {
  std::ifstream inp_file;
  std::vector<std::vector<int>> reports;
  inp_file.open("day2-in.txt");
  if (inp_file.is_open()) {
    std::string line;
    while (std::getline(inp_file, line)) {
      std::vector<int> temp = split_line(line);
      reports.push_back(temp);
    }
  }
  inp_file.close();

  // part 1
  int scount = 0;
  for (int i = 0; i < reports.size(); i++) {
    if (is_safe(reports[i]))
      scount++;
  }
  std::cout << scount << std::endl;

  // part 2
  int nscount = 0;
  // nscount means now safe count
  for (int i = 0; i < reports.size(); i++) {
    std::vector<int> report = reports[i];
    int l = report.size();
    int icount, dcount, iindx, dindx, diff, diffindx, diffcount;
    icount = dcount = diffcount = 0;
    iindx = dindx = diffindx = -1;
    for (int j = 0; j < l - 1; j++) {
      diff = std::abs(report[j] - report[j + 1]);
      report[j] > report[j + 1] ? dcount++ : dindx = j;
      report[j] < report[j + 1] ? icount++ : iindx = j;
      (diff >= 1 && diff <= 3) ? diffcount++ : diffindx = j;
    }
    if ((icount == l - 1 || dcount == l - 1) && diffcount == l - 1)
      nscount++;
    else if (icount == l - 2 && (remove_and_check(report, iindx) ||
                                 remove_and_check(report, iindx + 1)))
      nscount++;
    else if (dcount == l - 2 && (remove_and_check(report, dindx) ||
                                 remove_and_check(report, dindx + 1)))
      nscount++;
    else if (diffcount == l - 2 && (remove_and_check(report, diffindx) ||
                                    remove_and_check(report, diffindx + 1)))
      nscount++;
  }
  std::cout << nscount << std::endl;
  return 0;
}
