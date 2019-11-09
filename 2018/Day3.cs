using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;

namespace AdventOfCode2018
{
    class Day3
    {
        static readonly Regex rx = new Regex(@"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)", RegexOptions.Compiled);

        static void Main(string[] args)
        {
            var input = File.ReadAllLines("./inputs/day3.txt");
            var answer = PartOneAndTwo(input);

            Console.WriteLine(answer.Item1);
            Console.WriteLine(answer.Item2);
            Console.ReadLine();
        }

        static Tuple<int, int> PartOneAndTwo(string[] input)
        {
            var overlapping = 0;
            var fabric = new HashSet<int>[1000, 1000];
            var claimIds = new HashSet<int>(input.Select((s, i) => i + 1));

            foreach (var line in input)
            {
                var claim = ParseClaim(line);

                for (var i = claim.top; i < claim.top + claim.height; i++)
                {
                    for (var j = claim.left; j < claim.left + claim.width; j++)
                    {
                        if (fabric[i, j] == null)
                        {
                            fabric[i, j] = new HashSet<int>();
                        }

                        fabric[i, j].Add(claim.id);

                        if (fabric[i, j].Count == 2)
                        {
                            overlapping++;
                        }
                    }
                }
            }

            foreach (var squareInch in fabric)
            {
                if (squareInch == null || squareInch.Count == 1)
                {
                    continue;
                }

                foreach (var claimId in squareInch)
                {
                    claimIds.Remove(claimId);
                }
            }

            return Tuple.Create(overlapping, claimIds.First());
        }

        static Claim ParseClaim(string input)
        {
            var match = rx.Match(input);

            return new Claim
            {
                id = int.Parse(match.Groups[1].Value),
                left = int.Parse(match.Groups[2].Value),
                top = int.Parse(match.Groups[3].Value),
                width = int.Parse(match.Groups[4].Value),
                height = int.Parse(match.Groups[5].Value),
            };
        }

        struct Claim
        {
            public int id;
            public int left;
            public int top;
            public int width;
            public int height;
        }
    }
}
