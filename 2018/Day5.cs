using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;

namespace AdventOfCode2018
{
    class Day5
    {
        static void Main(string[] args)
        {
            var input = File.ReadAllText("./inputs/day5.txt").Trim();

            Console.WriteLine(PartOne(input));
            Console.WriteLine(PartTwo(input));
            Console.ReadLine();
        }

        static int PartOne(string input)
        {
            var sb = new StringBuilder(input);
            var length = 0;

            while (length != sb.Length)
            {
                length = sb.Length;
                Reduce(sb);
            }

            return length;
        }

        static int PartTwo(string input)
        {
            var polymerLengths = new List<int>();
            for (char c = 'A'; c <= 'Z'; c++)
            {
                polymerLengths.Add(PartOne(RemoveUnits(input, c)));
            }
            return polymerLengths.Min();
        }

        static string RemoveUnits(string s, char unit)
        {
            var sb = new StringBuilder(s);
            for (var i = 0; i < sb.Length; i++)
            {
                if (sb[i] == unit || sb[i] == unit + 32)
                {
                    sb.Remove(i, 1);
                    i--;
                }
            }
            return sb.ToString();
        }

        static void Reduce(StringBuilder sb)
        {
            for (var i = 0; i < sb.Length - 1; i++)
            {
                if (PolarOpposite(sb[i], sb[i + 1]))
                {
                    sb.Remove(i, 2);
                }
            }
        }

        static bool PolarOpposite(char a, char b)
        {
            return a - b == ' ' || b - a == ' ';
        }
    }
}
