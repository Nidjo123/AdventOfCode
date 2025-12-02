
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

invalidIds :: Range -> [Integer]
invalidIds (Range low high) = filter isInvalidId [low..high]

main = do
    line <- getContents
    let parts = splitOn (== ',') line
        ranges = map fromStr parts
        invalidIdsSum = sum . concat $ map invalidIds ranges
    putStrLn $ show invalidIdsSum
