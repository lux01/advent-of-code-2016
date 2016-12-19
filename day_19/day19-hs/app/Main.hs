module Main where

import Day19

main :: IO ()
main = do
  putStrLn $ "Part 1 Test Case = " ++ (show $ part1 5)
  putStrLn $ "Part 1 Challenge = " ++ (show $ part1 3017957)
  putStrLn $ "Part 2 Test Case = " ++ (show $ part2 5)
  putStrLn $ "Part 2 Challenge = " ++ (show $ part2 3017957)
