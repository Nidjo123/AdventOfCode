import Data.List

data Coord = Coord Int Int deriving (Eq, Show)
data State = State { origin :: Coord 
                   , splitters :: [Coord]
		   } deriving (Show)

readData :: String -> State
readData s = State {origin = origin, splitters = splitters}
  where
    rows = lines s
    height = length rows
    width = length . head $ rows
    tiles = concat [[ (Coord x y, c) | (x, c) <- zip [0..] row] | (y, row) <- zip [0..] rows]
    origin = fst . head $ filter (\(coord, c) -> c == 'S') tiles 
    splitters = map fst $ filter (\(coord, c) -> c == '^') tiles 

moveDown :: Coord -> Coord
moveDown (Coord x y) = Coord x (y + 1)

solveDown :: [Coord] -> [Coord] -> Int -> Int -> Int
solveDown splitters beams height splits = if height > maxHeight then splits else solveDown splitters newBeams (height + 1) (splits + length beamsToSplit)
  where
    maxHeight = maximum $ map (\(Coord x y) -> y) splitters
    beamsToSplit = filter (\beam -> any (\splitter -> beam == splitter) splitters) beams 
    unsplitBeams = beams \\ beamsToSplit
    newSplitBeams = concat [[Coord (x - 1) y, Coord (x + 1) y] | (Coord x y) <- beamsToSplit]
    newBeams = map moveDown . nub $ unsplitBeams ++ newSplitBeams

solve1 :: State -> Int
solve1 State {origin, splitters} = solveDown splitters [origin] 0 0

main = do
  contents <- getContents
  let 
    state = readData contents
  print $ solve1 state

