import Data.List


getOp :: (Num a) => Char -> (a -> a -> a)
getOp c = case c of
  '+' -> (+)
  '*' -> (*)
  _ -> error "unknown operation"

main = do
  contents <- getContents
  let
    table :: [[Integer]] = map (map read) . map words . init . lines $ contents
    operations = map getOp . map head . words . last $ lines contents
    part1 = sum 
      $ map (\(op, nums) -> foldr op (head nums) (tail nums)) 
      $ zip operations 
      $ transpose table
  print part1

