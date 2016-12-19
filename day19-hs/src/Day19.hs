module Day19 (part1, part2) where

import qualified Data.Sequence as Seq
import Data.Maybe

-- |The `shift` rotates all elements in a `Seq` to the left by one, wrapping
-- the first element onto the back of the list.
shift :: Seq.Seq a -> Seq.Seq a
shift x
  | null x = Seq.empty
  | otherwise = xs Seq.|> x'
  where (x' Seq.:< xs) = Seq.viewl x

-- |Removes the second element from a sequence. Acts trivially on the empty sequence.
dropNext :: Seq.Seq a -> Seq.Seq a
dropNext xs
  | null xs = Seq.empty
  | otherwise = front Seq.>< back'
  where (front, back) = Seq.splitAt 1 xs
        (_ Seq.:< back') = Seq.viewl back

-- |Removes the "opposite" element from a sequence, this is the element at offset
-- `n / 2` where `n` is the length of the input sequence. Acts trivially on the empty
-- sequence.
dropOpposite :: Seq.Seq a -> Seq.Seq a
dropOpposite xs
  | null xs   = Seq.empty
  | otherwise = front Seq.>< back'
  where (front, back) = Seq.splitAt n xs
        (_ Seq.:< back') = Seq.viewl back
        n = (Seq.length xs) `quot` 2

-- |The iteration step function for part 1
step1 :: Seq.Seq a -> Seq.Seq a
step1 = shift . dropNext

-- |The iteration step function for part 2
step2 :: Seq.Seq a -> Seq.Seq a
step2 = shift . dropOpposite

-- |Generic part runner function. Takes a step funcion `f` and the number of elves `n`.
-- Returns the number of the elf that has all the presents.
part :: (Seq.Seq Int -> Seq.Seq Int) -> Int -> Int
part f n = fromJust
  $ Seq.lookup 0 -- Get the first element in the 1 element sequence
  $ head
  $ dropWhile (\ls -> (Seq.length ls) > 1) -- Iterate until we get the singleton sequence
  $ iterate f
  $ Seq.fromList [1..n]

-- | Part 1 runner function. 
part1 :: Int -> Int
part1 = part step1

-- | Part 2 runner function.
part2 :: Int -> Int
part2 = part step2
