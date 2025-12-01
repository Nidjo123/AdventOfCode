
data Move = L Int | R Int

fromStr :: String -> Move
fromStr (dir:steps) = case dir of
    'L' -> L $ read steps
    'R' -> R $ read steps
    _   -> error "Unknown direction"

getMoves :: [String] -> [Move]
getMoves = map fromStr

nextPosition :: Move -> Int -> Int
nextPosition (L steps) currPos = (currPos - steps) `mod` 100
nextPosition (R steps) currPos = (currPos + steps) `mod` 100

calculatePositions :: Int -> [Move] -> [Int]
calculatePositions pos [] = [pos]
calculatePositions pos (m:ms) = nextPos : calculatePositions nextPos ms
    where
      nextPos = nextPosition m pos

main = do
    contents <- getContents
    let moves = map fromStr $ lines contents
        positions = calculatePositions 50 moves
        password = length $ filter (== 0) positions
    putStrLn $ show password

