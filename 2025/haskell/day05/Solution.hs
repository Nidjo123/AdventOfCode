import Data.List

data Range = Range Integer Integer deriving (Show)

rangeFromStr :: String -> Range
rangeFromStr s = Range (read start) (read $ tail end)
 where
  dashPos = case '-' `elemIndex` s of
    Just x -> x
    Nothing -> error "Invalid range format"
  (start, end) = splitAt dashPos s

rangeContains :: Range -> Integer -> Bool
rangeContains (Range low high) x = low <= x && x <= high

main = do
  contents <- getContents
  let (rangeLines, idLines) = partition (elem '-') $ filter (/= "") $ lines contents
  let
    ranges :: [Range] = map rangeFromStr rangeLines
    ids :: [Integer] = map read idLines
    validIds = filter (\x -> any (`rangeContains` x) ranges) ids
  print $ length validIds
