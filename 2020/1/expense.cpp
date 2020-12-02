#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

int binary_search(std::vector<int>& numbers, int n) {

    int i = 0;
    int j = numbers.size() - 1;

    if ( i > j ) {
        return -1;
    }

    int k;
    while ( i <= j ) {
        k = (i + j) / 2;

        if ( numbers[k] == n ) {
            return k;
        } else if ( numbers[k] < n ) {
            i = k + 1;
        } else {
            j = k - 1;
        }
    }

    return -1;

}

void find_two(std::vector<int>& numbers, int sum) {
    // O(n*log(n)), is the same time-complexity as later procedure for finding which numbers sum to 2020
    std::sort(numbers.begin(), numbers.end());

    int i,j;
    for ( i = 0; i < numbers.size(); i++ ) {
        j = binary_search(numbers, sum - numbers[i]);
        if ( j != -1 ) {
            break;
        }
    }

    std::cout << numbers[i] << " + " << numbers[j] << " = " << numbers[i] + numbers[j] << std::endl;
    std::cout << numbers[i] << " * " << numbers[j] << " = " << numbers[i] * numbers[j] << std::endl;
}

void find_three(std::vector<int>& numbers, int sum) {
    // This is a super dirty brute force solution
    int a,b,c;
    int i,j,k;
    bool searching = true;
    for ( i = 0; i < numbers.size() && searching; i++ ) {
        for ( j = 0; j < numbers.size() && searching; j++ ) {
            for ( k = 0; k < numbers.size() && searching; k++ ) {
                if ( numbers[i] + numbers[j] + numbers[k] == sum ) {
                    searching = false;
                    a = i;
                    b = j;
                    c = k;
                }
            }
        }
    }

    std::cout << numbers[a] << " + " << numbers[b] << " + " << numbers[c] << " = " << numbers[a] + numbers[b] + numbers[c] << std::endl;
    std::cout << numbers[a] << " * " << numbers[b] << " * " << numbers[c] << " = " << ((long int) numbers[a]) * ((long int) numbers[b]) * ((long int) numbers[c]) << std::endl;
}

int main() {
    std::string lineInput;
    // Could be pre-allocated to 200 since I know the size of the input. But I won't for the sake of portability
    std::vector<int> numbers;

    while (std::cin >> lineInput) {
        numbers.push_back(std::stoi(lineInput));
    }

    //find_two(numbers, 2020);
    find_three(numbers, 2020);

}
