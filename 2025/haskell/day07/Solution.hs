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

solve1 :: [Coord] -> [Coord] -> Int -> Int -> Int
solve1 splitters beams height splits = if height > maxHeight then splits else solve1 splitters newBeams (height + 1) (splits + length beamsToSplit)
  where
    maxHeight = maximum $ map (\(Coord x y) -> y) splitters
    beamsToSplit = filter (\beam -> any (\splitter -> beam == splitter) splitters) beams 
    unsplitBeams = beams \\ beamsToSplit
    newSplitBeams = concat [[Coord (x - 1) y, Coord (x + 1) y] | (Coord x y) <- beamsToSplit]
    newBeams = map moveDown . nub $ unsplitBeams ++ newSplitBeams

nextTimeline :: [(Int, Int)] -> [Coord] -> [(Int, Int)]
nextTimeline timelines splitters = newTimelines
  where
    timelinesToSplit = concat $ map (\(Coord x _) -> case lookup x timelines of
                                                     Just cnt -> [(x, cnt)]
                                                     Nothing -> []) splitters
    splitTimelines = concat $ map (\(x, cnt) -> [(x - 1, cnt), (x + 1, cnt)]) timelinesToSplit
    newTimelines = map (\xs -> (fst . head $ xs, sum . map snd $ xs))
                   $ groupBy (\(x1, _) (x2, _) -> x1 == x2)
		   $ sortBy (\(x1, _) (x2, _) -> x1 `compare` x2)
                   $ (timelines \\ timelinesToSplit) ++ splitTimelines

solve2 :: [[Coord]] -> [(Int, Int)] -> Int
solve2 splittersByHeight initTimelines = sum . map snd $ foldl' nextTimeline initTimelines splittersByHeight

main = do
  contents <- getContents
  let 
    State {origin=(Coord ox oy), splitters} = readData contents
    splittersByHeight = groupBy (\(Coord _ y1) (Coord _ y2) -> y1 == y2) splitters
  print $ solve1 splitters [Coord ox oy] 0 0
  print $ solve2 splittersByHeight [(ox, 1)]

