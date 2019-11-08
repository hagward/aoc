using Priority_Queue;
using System;
using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode2018
{
    class Day22
    {
        static void Main(string[] args)
        {
            //var depth = 3066;
            //var targetX = 13;
            //var targetY = 726;
            var depth = 510;
            var targetX = 10;
            var targetY = 10;

            var erosion = new int[targetX + 1];
            var map = new int[targetY + 1, targetX + 1];
            var risk = 0;

            for (var i = 0; i <= targetY; i++)
            {
                for (var j = 0; j <= targetX; j++)
                {
                    var geologicalIndex = GeologicalIndex(j, i, targetX, targetY, erosion);
                    var erosionLevel = (geologicalIndex + depth) % 20183;
                    var type = erosionLevel % 3;
                    map[i, j] = type;
                    risk += type;
                    erosion[j] = erosionLevel;
                }
            }

            var dist = Dijkstra2(map);

            Console.WriteLine(risk);
            Console.WriteLine(dist);
            Console.ReadLine();
        }

        static int GeologicalIndex(int x, int y, int targetX, int targetY, int[] v)
        {
            if (x == 0 && y == 0 || x == targetX && y == targetY)
            {
                return 0;
            }

            if (y == 0)
            {
                return x * 16807;
            }

            if (x == 0)
            {
                return y * 48271;
            }

            return v[x - 1] * v[x];
        }

        static int Dijkstra2(int[,] map)
        {
            var height = map.GetLength(0);
            var width = map.GetLength(1);
            var dist = new int[3, height, width];
            for (var i = 0; i < 3; i++)
            {
                for (var j = 0; j < height; j++)
                {
                    for (var k = 0; k < width; k++)
                    {
                        dist[i, j, k] = int.MaxValue;
                    }
                }
            }
            var q = new FastPriorityQueue<Vertex>(10000);
            q.Enqueue(new Vertex(0, 0, 1), 0);

            while (q.Any())
            {
                var u = q.Dequeue();

                var y = u.Y;
                var x = u.X;
                var t = u.Type;
                var d = (int)u.Priority;

                if (x < 0 || x >= width || y < 0 || y >= height || t == map[y, x] || d >= dist[t, y, x]) continue;

                Console.WriteLine($"Visiting {y},{x},{t},{d}");

                var oth = (t + 1) % 3;
                if (oth == map[y, x]) oth = (oth + 1) % 3;

                dist[t, y, x] = d;

                q.Enqueue(new Vertex(x, y, oth), d + 7);
                q.Enqueue(new Vertex(x + 1, y, t), d + 1);
                q.Enqueue(new Vertex(x - 1, y, t), d + 1);
                q.Enqueue(new Vertex(x, y + 1, t), d + 1);
                q.Enqueue(new Vertex(x, y - 1, t), d + 1);
            }

            return dist[1, height - 1, width - 1];
        }
    }

    class Comparer : IComparer<int[]>
    {
        public int Compare(int[] x, int[] y)
        {
            return x[3] - y[3];
        }
    }

    class Vertex : FastPriorityQueueNode
    {
        public int X { get; set; }
        public int Y { get; set; }
        public int Type { get; set; }

        public Vertex(int x, int y, int type)
        {
            X = x;
            Y = y;
            Type = type;
        }
    }
}
