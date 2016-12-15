#!/usr/bin/env wolframscript

(* ::Package:: *)

part1Input = "Disc #1 has 13 positions; at time=0, it is at position 1.
Disc #2 has 19 positions; at time=0, it is at position 10.
Disc #3 has 3 positions; at time=0, it is at position 2.
Disc #4 has 7 positions; at time=0, it is at position 1.
Disc #5 has 5 positions; at time=0, it is at position 3.
Disc #6 has 17 positions; at time=0, it is at position 5.";

part2Input = part1Input <> "\n" <> "Disc #7 has 11 positions; at time=0, it is at position 0.";

solve[input_] := StringCases[input, RegularExpression["Disc #(\\d+) has (\\d+) positions; at time=0, it is at position (\\d+)\\."] -> {"$1","$3","$2"}] // Map[ToExpression, #, 2]& // Map[Mod[t + #[[1]] + #[[2]], #[[3]]] == 0 &] // Append[t >= 0] // Apply[And] // FindInstance[#, t, Integers][[1,1,2]]&

Print["Part 1: time = ", solve[part1Input]];
Print["Part 2: time = ", solve[part2Input]];



