using System;
using System.Collections.Generic;
using System.Text;
using System.Text.RegularExpressions;

namespace AdventOfCode2018
{
    class Day17
    {
        private static readonly Regex regex = new Regex(@"^(x|y)=(\d+), (x|y)=(\d+)..(\d+)$", RegexOptions.Compiled);

        static void Main(string[] args)
        {
            var input = new List<string>()
            {
                "x=495, y=2..7",
                "y=7, x=495..501",
                "x=501, y=3..7",
                "x=498, y=2..4",
                "x=506, y=1..2",
                "x=498, y=10..13",
                "x=504, y=10..13",
                "y=13, x=498..504",
            };

            var scan = Scan(input);

            for (var i = 0; i < 14; i++)
            {
                for (var j = 494; j < 508; j++)
                {
                    Console.Write(scan[i, j] ? '#' : '.');
                }
                Console.WriteLine();
            }
            Console.ReadLine();
        }

        static bool[,] Scan(List<string> input)
        {
            var scan = new bool[1000, 1000];

            foreach (var line in input)
            {
                var match = regex.Match(line);
                var a = int.Parse(match.Groups[2].Value);
                var b = int.Parse(match.Groups[4].Value);
                var c = int.Parse(match.Groups[5].Value);

                for (int i = a, j = b; j <= c; j++)
                {
                    if (match.Groups[1].Value == "x")
                    {
                        scan[j, i] = true;
                    }
                    else
                    {
                        scan[i, j] = true;
                    }
                }
            }

            return scan;
        }
    }
}
