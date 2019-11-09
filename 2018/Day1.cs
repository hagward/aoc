using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace AdventOfCode2018
{
    class Day1
    {
        static void Main(string[] args)
        {
            var input = File.ReadLines("./inputs/day1.txt")
                .Select(number => int.Parse(number));

            Console.WriteLine(PartOne(input));
            Console.WriteLine(PartTwo(input));
            Console.ReadLine();
        }

        static int PartOne(IEnumerable<int> input)
        {
            return input.Sum();
        }

        static int PartTwo(IEnumerable<int> input)
        {
            var inputArray = input.ToArray();
            var visited = new HashSet<int>();
            var currentSum = 0;

            for (var i = 0; !visited.Contains(currentSum); i = (i + 1) % inputArray.Length)
            {
                visited.Add(currentSum);
                currentSum += inputArray[i];
            }

            return currentSum;
        }
    }
}
