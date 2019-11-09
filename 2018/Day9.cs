using System;
using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode2018
{
    class Day9
    {
        static void Main(string[] args)
        {
            var players = 462;
            var marbles = 7193800;

            var scores = new long[players];
            var circle = new LinkedList<int>();
            var node = circle.AddFirst(0);

            for (int marble = 1, player = 0; marble < marbles; marble++, player = (player + 1) % players)
            {
                if (marble % 23 == 0)
                {
                    for (var i = 0; i < 6; i++)
                    {
                        node = PreviousNode(circle, node);
                    }

                    scores[player] += marble + node.Value;

                    var oldNode = node;
                    node = PreviousNode(circle, node);
                    circle.Remove(oldNode);
                }
                else
                {
                    for (var i = 0; i < 2; i++)
                    {
                        node = NextNode(circle, node);
                    }

                    circle.AddAfter(node, marble);
                }
            }

            Console.WriteLine(scores.Max());
            Console.ReadLine();
        }

        static LinkedListNode<T> NextNode<T>(LinkedList<T> list, LinkedListNode<T> currentNode)
        {
            return currentNode.Next ?? list.First;
        }

        static LinkedListNode<T> PreviousNode<T>(LinkedList<T> list, LinkedListNode<T> currentNode)
        {
            return currentNode.Previous ?? list.Last;
        }
    }
}
