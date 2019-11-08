using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;

namespace AdventOfCode2018
{
    class Day4
    {
        private static readonly Regex regex = new Regex(@"Guard #(\d+) begins shift", RegexOptions.Compiled);

        static void Main(string[] args)
        {
            var input = File.ReadLines("./inputs/day4.txt").ToList();
            input.Sort();

            //var input = new List<string>()
            //{
            //    "[1518-11-01 00:00] Guard #10 begins shift",
            //    "[1518-11-01 00:05] falls asleep",
            //    "[1518-11-01 00:25] wakes up",
            //    "[1518-11-01 00:30] falls asleep",
            //    "[1518-11-01 00:55] wakes up",
            //    "[1518-11-01 23:58] Guard #99 begins shift",
            //    "[1518-11-02 00:40] falls asleep",
            //    "[1518-11-02 00:50] wakes up",
            //    "[1518-11-03 00:05] Guard #10 begins shift",
            //    "[1518-11-03 00:24] falls asleep",
            //    "[1518-11-03 00:29] wakes up",
            //    "[1518-11-04 00:02] Guard #99 begins shift",
            //    "[1518-11-04 00:36] falls asleep",
            //    "[1518-11-04 00:46] wakes up",
            //    "[1518-11-05 00:03] Guard #99 begins shift",
            //    "[1518-11-05 00:45] falls asleep",
            //    "[1518-11-05 00:55] wakes up",
            //};

            var n = 4000;
            var times = new int[n];
            var minutes = new int[n, 60];

            for (var i = 0; i < input.Count;)
            {
                var match = regex.Match(input[i++]);
                var guardId = int.Parse(match.Groups[1].Value);

                while (i < input.Count && input[i].EndsWith("falls asleep"))
                {
                    var d1 = DateTime.Parse(input[i].Substring(1, 16));
                    var d2 = DateTime.Parse(input[i + 1].Substring(1, 16));
                    times[guardId] += (int)d2.Subtract(d1).TotalMinutes;
                    while (d1 < d2)
                    {
                        minutes[guardId, d1.Minute]++;
                        d1 = d1.AddMinutes(1);
                    }
                    i += 2;
                }
            }

            var maxIndex = 0;
            for (var i = 0; i < times.Length; i++)
            {
                if (times[i] > times[maxIndex])
                {
                    maxIndex = i;
                }
            }

            var maxMinute = 0;
            for (var i = 0; i < 60; i++)
            {
                if (minutes[maxIndex, i] > minutes[maxIndex, maxMinute])
                {
                    maxMinute = i;
                }
            }

            var maxIndex2 = 0;
            var maxMinute2 = 0;
            for (var i = 0; i < n; i++)
            {
                for (var j = 0; j < 60; j++)
                {
                    if (minutes[i, j] > minutes[maxIndex2, maxMinute2])
                    {
                        maxIndex2 = i;
                        maxMinute2 = j;
                    }
                }
            }

            Console.WriteLine(maxIndex * maxMinute);
            Console.WriteLine(maxIndex2 * maxMinute2);
            Console.ReadLine();
        }
    }

    class Event
    {
        public DateTime Time { get; set; }
        public int Type { get; set; }
        public int Id { get; set; }
    }
}
