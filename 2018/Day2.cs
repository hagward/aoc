using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace AdventOfCode2018
{
    class Day2
    {
        static void Main(string[] args)
        {
            var input = File.ReadLines("./inputs/day2.txt");

            Console.WriteLine(PartOne(input));
            Console.WriteLine(PartTwo(input));
            Console.ReadLine();
        }

        static int PartOne(IEnumerable<string> input)
        {
            var twos = 0;
            var threes = 0;

            foreach (var chars in input.Select(CountChars).Select(c => c.Values))
            {
                if (chars.Contains(2)) twos++;
                if (chars.Contains(3)) threes++;
            }

            return twos * threes;
        }

        static Dictionary<char, int> CountChars(string s)
        {
            return s.Aggregate(new Dictionary<char, int>(), (prev, current) =>
            {
                prev[current] = prev.GetValueOrDefault(current) + 1;
                return prev;
            });
        }

        static string PartTwo(IEnumerable<string> input)
        {
            var inputArray = input.ToArray();
            for (var i = 0; i < inputArray.Length; i++)
            {
                for (var j = i + 1; j < inputArray.Length; j++)
                {
                    var diffingIndex = GetDiffingIndex(inputArray[i], inputArray[j]);
                    if (diffingIndex != -1)
                    {
                        return inputArray[i].Remove(diffingIndex, 1);
                    }
                }
            }
            return null;
        }

        static int GetDiffingIndex(string s1, string s2)
        {
            var count = 0;
            var index = 0;

            for (var i = 0; i < s1.Length && count <= 1; i++)
            {
                if (s1[i] != s2[i])
                {
                    count++;
                    index = i;
                }
            }

            return count == 1 ? index : -1;
        }
    }
}
