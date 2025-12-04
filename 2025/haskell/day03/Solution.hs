import Data.List (elemIndex)


findMaxUpToIdx :: Int -> String -> (Char, String)
findMaxUpToIdx idx s = (maxChar, leftover)
    where
      haystack = take idx s
      maxChar = maximum haystack
      maxCharIdx = case elemIndex maxChar haystack of
        Just idx -> idx
        Nothing -> error "no index found"
      leftover = snd $ splitAt (maxCharIdx + 1) s

findMaxButLeave :: Int -> String -> (Char, String)
findMaxButLeave n s = findMaxUpToIdx (length s - n) s

maxJoltage' :: Int -> String -> String
maxJoltage' 0 s = []
maxJoltage' digitsLeft s = nextDigit : maxJoltage' (digitsLeft - 1) leftover
    where
      (nextDigit, leftover) = findMaxButLeave (digitsLeft - 1) s

maxJoltage :: Int -> String -> Integer
maxJoltage len bank = read $ maxJoltage' len bank

main = do
    contents <- getContents
    let banks = lines contents
    putStrLn $ show . sum $ map (maxJoltage 2) banks
    putStrLn $ show . sum $ map (maxJoltage 12) banks

