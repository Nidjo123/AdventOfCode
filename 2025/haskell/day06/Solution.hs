import Data.List


getOp :: (Num a) => Char -> (a -> a -> a)
getOp c = case c of
  '+' -> (+)
  '*' -> (*)
  _ -> error "unknown operation"

splitBy :: (a -> Bool) -> [a] -> [[a]]
splitBy p xs = splitBy' p xs [] []
    where
      splitBy' p [] res buf = reverse (buf : res)
      splitBy' p (x:xs) res buf = if p x then splitBy' p xs (buf : res) [] else splitBy' p xs res (buf ++ [x])

solve2 :: [String] -> Integer
solve2 xs = foldr op initNum tailNums
  where
    op = getOp . last . head $ xs
    initNum = read . init . head $ xs
    tailNums = map read . tail $ xs

main = do
  contents <- getContents
  let
    table :: [[Integer]] = map (map read) . map words . init . lines $ contents
    operations = map getOp . map head . words . last $ lines contents
    part1 = sum 
      $ map (\(op, nums) -> foldr op (head nums) (tail nums)) 
      $ zip operations 
      $ transpose table
    part2 = sum
      $ map solve2
      $ splitBy (null . words) . transpose
      $ lines contents
  print part1
  print part2
