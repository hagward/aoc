using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace AdventOfCode2018
{
    class Day8
    {
        static void Main(string[] args)
        {
            var input = File.ReadAllText("./inputs/day8.txt")
                .Split(' ')
                .Select(number => int.Parse(number));

            Console.WriteLine(Sum(new Queue<int>(input), ParseNode1));
            Console.WriteLine(Sum(new Queue<int>(input), ParseNode2));
            Console.ReadLine();
        }

        static int Sum(Queue<int> input, Func<Queue<int>, int> sumFunc)
        {
            var sum = 0;

            while (input.Count > 0)
            {
                sum += sumFunc(input);
            }

            return sum;
        }

        static int ParseNode1(Queue<int> input)
        {
            var sum = 0;

            var numChilds = input.Dequeue();
            var numMetaData = input.Dequeue();

            for (var i = 0; i < numChilds; i++)
            {
                sum += ParseNode1(input);
            }

            for (var i = 0; i < numMetaData; i++)
            {
                sum += input.Dequeue();
            }

            return sum;
        }

        static int ParseNode2(Queue<int> input)
        {
            var sum = 0;

            var numChilds = input.Dequeue();
            var numMetaData = input.Dequeue();
            var childSums = new int[numChilds];

            for (var i = 0; i < numChilds; i++)
            {
                childSums[i] = ParseNode2(input);
            }

            for (var i = 0; i < numMetaData; i++)
            {
                var metaData = input.Dequeue();
                if (numChilds == 0)
                {
                    sum += metaData;
                }
                else if (childSums.Length >= metaData)
                {
                    sum += childSums[metaData - 1];
                }
            }

            return sum;
        }
    }
}
