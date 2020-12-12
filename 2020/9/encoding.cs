using System;
using System.IO;
using System.Collections.Generic;

public class Encoding {
    public static void Main(string[] args) {

        var lines = File.ReadLines("./input.txt");

        List<long> nums = new List<long>();

        foreach ( var line in lines ) {
            long number = Int64.Parse(line);
            nums.Add(number);
        }

        System.Console.WriteLine(nums.Count);

        long falseNum = 0;
        bool foundPair = true;
        for( int i = 25; i < nums.Count; i++ ) {
            foundPair = false;
            for ( int j = i - 25; j < i && !foundPair; j++ ) {
                for ( int k = i - 25; k < i && !foundPair; k++ ) {
                    if ( j == k ) {
                        continue;
                    }
                    if ( nums[j] + nums[k] == nums[i] ) {
                        foundPair = true;
                    }
                }
            }

            if ( !foundPair ) {
                falseNum = nums[i];
            }
        }

        System.Console.WriteLine(falseNum);
        System.Console.WriteLine("");

        int start = 0;
        int end = 0;
        for ( int i = 0; i < nums.Count; i++ ) {
            long sum = 0;
            for ( int j = i; j < nums.Count && sum < falseNum; j++ ) {
                sum += nums[j];
                if ( sum == falseNum ) {
                    start = i;
                    end = j;
                    i = nums.Count;
                    System.Console.WriteLine(sum);
                }
            }
        }

        int min = start;
        int max = end;
        for ( int i = start; i <= end; i++ ) {
            if ( nums[i] < nums[min] ) {
                min = i;
            }
            if ( nums[i] > nums[max] ) {
                max = i;
            }
        }

        System.Console.WriteLine(start);
        System.Console.WriteLine(end);
        System.Console.WriteLine(min);
        System.Console.WriteLine(max);
        System.Console.WriteLine(nums[min] + nums[max]);
    }
}
