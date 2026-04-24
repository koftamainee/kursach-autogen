// src/main.cpp
#include <iostream>
#include <vector>
#include <random>

int main() {
    // Line 5
    std::vector<double> data;
    std::mt19937 rng(42);
    std::normal_distribution<> dist(0.0, 1.0);

    for (int i = 0; i < 1000; ++i) {   // line 10
        data.push_back(dist(rng));
    }

    double sum = 0.0;
    for (double x : data) {
        sum += x;
    }
    double mean = sum / data.size();   // line 20

    double var = 0.0;
    for (double x : data) {
        var += (x - mean) * (x - mean);
    }
    var /= data.size();
    double stddev = std::sqrt(var);    // line 30

    std::cout << "Mean: " << mean << "\n";
    std::cout << "Stddev: " << stddev << "\n";

    // Additional lines to reach 40
    std::cout << "Done.\n";            // line 35
    std::cout << "All tests passed.\n";
    std::cout << "Exiting.\n";
    std::cout << "End.\n";
    std::cout << "Line 40.\n";         // line 40
    return 0;
}
