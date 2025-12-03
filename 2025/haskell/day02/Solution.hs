
data Range = Range Integer Integer deriving (Show)

fromStr :: String -> Range
fromStr s = Range low high
    where
      [low, high] = map read $ splitOn (== '-') s

splitOn :: (Char -> Bool) -> String -> [String]
splitOn pred str = splitOn' pred str [] ""
    where
      splitOn' pred "" strings buf = reverse (buf : strings)
      splitOn' pred (c:s) strings buf = if pred c then splitOn' pred s (buf : strings) "" else splitOn' pred s strings (buf ++ [c])

isInvalidId :: Integer -> Bool
isInvalidId x = left == right
    where 
      strX = show x
      (left, right) = splitAt (length strX `div` 2) strX

invalidIds :: (Integer -> Bool) -> Range -> [Integer]
invalidIds filtPred (Range low high) = filter filtPred [low..high]

allEqual :: (Eq a) => [a] -> Bool
allEqual [] = True
allEqual (x:[]) = True
allEqual (x:xs) = x == head xs && allEqual xs

substrings :: String -> Int -> [String]
substrings "" _  = []
substrings s len = s1 : substrings rest len
    where
      (s1, rest) = splitAt len s

isInvalidId2 :: Integer -> Bool
isInvalidId2 x = any allEqual $ map (substrings strX) [len | len <- [1..lenX `div` 2], lenX `mod` len == 0]
    where
      strX = show x
      lenX = length strX

main = do
    line <- getContents
    let parts = splitOn (== ',') line
        ranges = map fromStr parts
        invalidIdsSum1 = sum . concat $ map (invalidIds isInvalidId) ranges
        invalidIdsSum2 = sum . concat $ map (invalidIds isInvalidId2) ranges
    putStrLn $ show invalidIdsSum1
    putStrLn $ show invalidIdsSum2
