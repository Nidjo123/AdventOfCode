import Data.List

data Range = Range Integer Integer deriving (Show)

rangeFromStr :: String -> Range
rangeFromStr s = Range (read start) (read $ tail end)
 where
  dashPos = case '-' `elemIndex` s of
    Just x -> x
    Nothing -> error "Invalid range format"
  (start, end) = splitAt dashPos s

rangeLength :: Range -> Integer
rangeLength (Range a b) = b - a + 1

rangeContains :: Range -> Integer -> Bool
rangeContains (Range low high) x = low <= x && x <= high

disjointRanges :: Range -> Range -> Bool
disjointRanges (Range a b) (Range c d) = b < c || d < a

containsRange :: Range -> Range -> Bool
containsRange (Range a b) (Range c d) = a <= c && b >= d

canMergeRanges :: Range -> Range -> Bool
canMergeRanges r1 r2 = not $ disjointRanges r1 r2

mergeRanges :: Range -> Range -> Range
mergeRanges r1@(Range a b) r2@(Range c d) =
  if containsRange r1 r2 then r1
  else if containsRange r2 r1 then r2
  else if a < c then Range a d
  else Range c b

addAndMergeRange :: [Range] -> Range -> [Range]
addAndMergeRange ranges range = mergedRange : rest
  where
    (toMerge, rest) = partition (canMergeRanges range) ranges
    mergedRange = foldr mergeRanges range toMerge

minimizeRanges :: [Range] -> [Range]
minimizeRanges = foldr (flip addAndMergeRange) []

main = do
  contents <- getContents
  let (rangeLines, idLines) = partition (elem '-') $ filter (/= "") $ lines contents
  let
    ranges :: [Range] = map rangeFromStr rangeLines
    ids :: [Integer] = map read idLines
    validIds = filter (\x -> any (`rangeContains` x) ranges) ids
    minimizedRanges = minimizeRanges ranges
    totalFreshIds = sum $ map rangeLength minimizedRanges
  print $ length validIds
  print totalFreshIds

