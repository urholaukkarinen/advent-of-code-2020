{-# LANGUAGE OverloadedStrings #-}

import Data.Either
import Data.List (transpose)
import Data.Text (Text, count, drop, pack, replace, splitOn, take, unpack)
import Data.Text.Read (decimal)

readInput :: IO [[Text]]
readInput = do
  text <- readFile "input.txt"
  return ((map (splitOn " " . replace ":" "" . replace "-" " ") . splitOn "\n") (pack text))

inRange :: Int -> Int -> Int -> Bool
inRange min max val = val >= min && val <= max

toInt :: Text -> Int
toInt a = read (unpack a) :: Int

isValidPartOne :: [Text] -> Bool
isValidPartOne (min : max : c : passwd : _) = inRange (toInt min) (toInt max) (count c passwd)
isValidPartOne (a : b) = False
isValidPartOne [] = False

nth :: Int -> Text -> Text
nth n text = Data.Text.take 1 (Data.Text.drop n text)

isValidPartTwo :: [Text] -> Bool
isValidPartTwo (i : j : c : passwd : _) = (nth (toInt i -1) passwd == c) /= (nth (toInt j -1) passwd == c)
isValidPartTwo (a : b) = False
isValidPartTwo [] = False

partOne :: [[Text]] -> Int
partOne = length . filter isValidPartOne

partTwo :: [[Text]] -> Int
partTwo = length . filter isValidPartTwo

main :: IO ()
main = do
  input <- readInput
  print (partOne input)
  print (partTwo input)