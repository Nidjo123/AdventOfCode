import Data.List (elemIndex)


maxJoltage :: String -> Int
maxJoltage bank = read [fstDigit, sndDigit] 
    where
      fstDigit = maximum $ init bank
      fstDigitIndex = case elemIndex fstDigit bank of
        Just idx -> idx
        Nothing -> error "no index found"
      sndDigit = maximum . snd $ splitAt (fstDigitIndex + 1) bank 

main = do
    contents <- getContents
    let banks = lines contents
    putStrLn $ show . sum $ map maxJoltage banks
