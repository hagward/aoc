using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace AdventOfCode2018
{
    class Day16
    {
        static void Main(string[] args)
        {
            Console.WriteLine(PartOne(File.ReadAllLines("./inputs/day16-1.txt").ToList()));
            Console.WriteLine(PartTwo(File.ReadAllLines("./inputs/day16-2.txt").ToList()));
            Console.ReadLine();
        }

        static int PartOne(List<string> input)
        {
            var result = 0;

            for (var i = 0; i < input.Count; i += 4)
            {
                int[] regBefore = input[i].Substring(9, 10).Split(", ").Select(int.Parse).ToArray();
                int[] regAfter = input[i + 2].Substring(9, 10).Split(", ").Select(int.Parse).ToArray();
                int[] abc = input[i + 1].Split(' ').Select(int.Parse).ToArray();

                var matching = OpcodesMatching(regBefore, regAfter, abc);
                if (matching >= 3)
                {
                    result++;
                }
            }

            return result;
        }

        static int PartTwo(List<string> input)
        {
            var cpu = new Cpu();

            foreach (var line in input)
            {
                var args = line.Split(" ").Select(int.Parse).ToArray();
                cpu.Execute(args[0], args[1], args[2], args[3]);
            }

            return cpu.Reg[0];
        }

        static int OpcodesMatching(int[] regBefore, int[] regAfter, int[] args)
        {
            var matching = 0;

            foreach (var opcode in Cpu.Opcodes)
            {
                var cpu = new Cpu()
                {
                    Reg = (int[])regBefore.Clone(),
                };

                cpu.Execute(opcode, args[1], args[2], args[3]);

                if (Enumerable.SequenceEqual(regAfter, cpu.Reg))
                {
                    matching++;
                }
            }

            return matching;
        }
    }

    class Cpu
    {
        public static readonly string[] Opcodes = new string[]
        {
            "gtri", "bani", "eqrr", "gtir", "eqir", "bori", "seti", "setr", "addr", "borr", "muli", "banr", "addi", "eqri", "mulr", "gtrr",
        };

        public int[] Reg = new int[4];

        public void Execute(int opcode, int a, int b, int c)
        {
            Execute(Opcodes[opcode], a, b, c);
        }

        public void Execute(string opcode, int a, int b, int c)
        {
            switch (opcode)
            {
                case "addr": Reg[c] = Reg[a] + Reg[b]; break;
                case "addi": Reg[c] = Reg[a] + b; break;
                case "mulr": Reg[c] = Reg[a] * Reg[b]; break;
                case "muli": Reg[c] = Reg[a] * b; break;
                case "banr": Reg[c] = Reg[a] & Reg[b]; break;
                case "bani": Reg[c] = Reg[a] & b; break;
                case "borr": Reg[c] = Reg[a] | Reg[b]; break;
                case "bori": Reg[c] = Reg[a] | b; break;
                case "setr": Reg[c] = Reg[a]; break;
                case "seti": Reg[c] = a; break;
                case "gtir": Reg[c] = a > Reg[b] ? 1 : 0; break;
                case "gtri": Reg[c] = Reg[a] > b ? 1 : 0; break;
                case "gtrr": Reg[c] = Reg[a] > Reg[b] ? 1 : 0; break;
                case "eqir": Reg[c] = a == Reg[b] ? 1 : 0; break;
                case "eqri": Reg[c] = Reg[a] == b ? 1 : 0; break;
                case "eqrr": Reg[c] = Reg[a] == Reg[b] ? 1 : 0; break;
                default: throw new ArgumentException();
            }
        }

        public override string ToString()
        {
            return string.Join(", ", Reg);
        }
    }
}
