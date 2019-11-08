using System;
using System.Collections.Generic;
using System.Text;

namespace AdventOfCode2018
{
    class Day18
    {
        static void Main(string[] args)
        {
            var input = new List<string>()
            {
                ".#.#...|#.",
                ".....#|##|",
                ".|..|...#.",
                "..|#.....#",
                "#.#|||#|#|",
                "...#.||...",
                ".|....|...",
                "||...#|.#|",
                "|.||||..|.",
                "...#.|..|.",
            };
        }

        static List<string> ApplyChange(List<string> input)
        {
            for (var i = 0; i < input.Count; i++)
            {
                for (var j = 0; j < input[i].Length; j++)
                {
                    var c = input[i][j];
                }
            }
            return new List<string>();
        }

        static int[] Adjacent(List<string> input, int x, int y)
        {
            int[] result = new int[] { 0, 0, 0 };
            return result;
        }
    }
}
