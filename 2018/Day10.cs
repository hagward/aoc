using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;

namespace AdventOfCode2018
{
    class Day10
    {
        static void Main(string[] args)
        {
            var regex = new Regex(@"^position=<\s*(-?\d+),\s*(-?\d+)>\svelocity=<\s*(-?\d+),\s*(-?\d+)>$");
            var input = File.ReadLines("./inputs/day10.txt")
                .Select(line =>
                {
                    var match = regex.Match(line);

                    return new Point()
                    {
                        X = int.Parse(match.Groups[1].Value),
                        Y = int.Parse(match.Groups[2].Value),
                        VelocityX = int.Parse(match.Groups[3].Value),
                        VelocityY = int.Parse(match.Groups[4].Value),
                    };
                }).ToList();

            var area = int.MaxValue;
            var seconds = 0;

            while (true)
            {
                var newInput = Iterate(input);
                var box = BoundingBox(newInput);
                var newArea = (box.Item2.X - box.Item1.X) * (box.Item2.Y - box.Item1.Y);

                if (newArea > area && area > 0 && area < 1000)
                {
                    Print(input, box);
                    Console.WriteLine($"{seconds} seconds");
                    break;
                }

                area = newArea;
                input = newInput;
                seconds++;
            }

            Console.ReadLine();
        }

        static void Print(IEnumerable<Point> points, Tuple<Point, Point> bounds)
        {
            var (minBound, maxBound) = bounds;
            var xMin = minBound.X;
            var yMin = minBound.Y;
            var xLength = maxBound.X - xMin;
            var yLength = maxBound.Y - yMin;
            var c = new char[xLength + 1, yLength + 1];

            foreach (var point in points)
            {
                c[point.X - xMin, point.Y - yMin] = '#';
            }

            for (var i = 0; i < yLength; i++)
            {
                for (var j = 0; j < xLength; j++)
                {
                    Console.Write(c[j, i]);
                }
                Console.WriteLine();
            }
        }

        static List<Point> Iterate(List<Point> points)
        {
            return points.Select(point => new Point()
            {
                X = point.X + point.VelocityX,
                Y = point.Y + point.VelocityY,
                VelocityX = point.VelocityX,
                VelocityY = point.VelocityY,
            }).ToList();
        }

        static Tuple<Point, Point> BoundingBox(IEnumerable<Point> points)
        {
            var minX = int.MaxValue;
            var maxX = int.MinValue;
            var minY = int.MaxValue;
            var maxY = int.MinValue;

            foreach (var point in points)
            {
                if (point.X < minX) minX = point.X;
                if (point.X > maxX) maxX = point.X;
                if (point.Y < minY) minY = point.Y;
                if (point.Y > maxY) maxY = point.Y;
            }

            return Tuple.Create(new Point(minX, minY), new Point(maxX, maxY));
        }
    }

    class Point
    {
        public int X { get; set; }
        public int Y { get; set; }
        public int VelocityX { get; set; }
        public int VelocityY { get; set; }

        public Point()
        {
        }

        public Point(int x, int y)
        {
            X = x;
            Y = y;
        }
    }
}
