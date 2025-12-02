
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

leftZeros :: Int -> Int -> Int -> (Int, Int)
leftZeros currPos 0 zeros = (currPos, zeros)
leftZeros currPos steps zeros = leftZeros ((currPos - 1) `mod` 100) (steps - 1) (zeros + if currPos == 1 then 1 else 0)

swap :: (a, b) -> (b, a)
swap (x, y) = (y, x)

nextState :: (Int, Int) -> Move -> (Int, Int)
nextState (zs, currPos) move = (zs + abs turns, nextPos)
    where
      (turns, nextPos) = case move of
        L steps -> swap $ leftZeros currPos steps 0
        R steps -> (currPos + steps) `divMod` 100

calculatePassword2 :: Int -> [Move] -> (Int, Int)
calculatePassword2 startPos = foldl nextState (if startPos == 0 then 1 else 0, startPos)

main = do
    contents <- getContents
    let moves = map fromStr $ lines contents
        positions = calculatePositions 50 moves
        password1 = length $ filter (== 0) positions
        password2 = fst $ calculatePassword2 50 moves
    putStrLn $ show password1
    putStrLn $ show password2

