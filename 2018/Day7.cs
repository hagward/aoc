using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;

namespace AdventOfCode2018
{
    class Day7
    {
        static void Main(string[] args)
        {
            //var input = new List<string>()
            //{
            //    "Step C must be finished before step A can begin.",
            //    "Step C must be finished before step F can begin.",
            //    "Step A must be finished before step B can begin.",
            //    "Step A must be finished before step D can begin.",
            //    "Step B must be finished before step E can begin.",
            //    "Step D must be finished before step E can begin.",
            //    "Step F must be finished before step E can begin.",
            //};

            var input = File.ReadLines("./inputs/day7.txt");

            var trees = new Dictionary<char, Tree>();

            foreach (var line in input)
            {
                var left = trees.GetValueOrDefault(line[5], new Tree(line[5]));
                var right = trees.GetValueOrDefault(line[36], new Tree(line[36]));
                left.Children.Add(right);
                right.Parents.Add(left);
                if (!trees.ContainsKey(left.Label)) trees.Add(left.Label, left);
                if (!trees.ContainsKey(right.Label)) trees.Add(right.Label, right);
            }

            Tree start = trees.First().Value;
            while (start.Parents.Any())
            {
                start = start.Parents.First();
            }

            Console.WriteLine($"Found starting node {start.Label} with {start.Children.Count} children");

            var visited = new HashSet<char>();
            var comparer = Comparer<Tree>.Create((a, b) =>
            {
                var left = a.Parents.Any(parent => !visited.Contains(parent.Label));
                var right = b.Parents.Any(parent => !visited.Contains(parent.Label));
                if (left && !right) return 1;
                else if (right && !left) return -1;
                else return a.Label.CompareTo(b.Label);
            });

            var q = new SortedSet<Tree>(comparer) { start };

            while (q.Any())
            {
                var current = q.Min;
                q.Remove(current);

                if (visited.Contains(current.Label)) continue;

                Console.Write(current.Label);

                visited.Add(current.Label);
                current.Children.ForEach(child => q.Add(child));
            }

            Console.WriteLine();
            Console.ReadLine();
        }
    }

    class Tree
    {
        public readonly char Label;
        public readonly List<Tree> Children = new List<Tree>();
        public readonly List<Tree> Parents = new List<Tree>();

        public Tree(char label)
        {
            Label = label;
        }
    }
}
