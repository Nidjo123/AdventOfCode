import Data.List

data Point = Point Integer Integer deriving (Eq, Ord, Show)

replace :: (Eq a) => a -> a -> [a] -> [a]
replace old new = map (\x -> if x == old then new else x)

readPoint :: String -> Point
readPoint s = Point x y
  where
    [x, y] = map read . words . replace ',' ' ' $ s

areaBetween :: Point -> Point -> Integer
areaBetween (Point x1 y1) (Point x2 y2) = (abs (x2 - x1) + 1) * (abs (y2 - y1) + 1)

pairs :: [a] -> [(a, a)]
pairs [] = []
pairs (x:xs) = (zip (repeat x) xs) ++ pairs xs

main = do
  contents <- getContents
  let
    points = map readPoint $ lines contents
    maxArea = maximum . map (uncurry areaBetween) . pairs $ points
  print maxArea
