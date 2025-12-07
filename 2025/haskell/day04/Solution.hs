data Coord = Coord Int Int deriving (Show)
data Tile = Empty | Paper deriving (Show, Eq)
type Grid = [[Tile]]

gridHeight :: Grid -> Int
gridHeight = length

gridWidth :: Grid -> Int
gridWidth grid = if gridHeight grid <= 0 then 0 else length $ head grid

charToTile :: Char -> Tile
charToTile c = case c of
  '.' -> Empty
  '@' -> Paper
  _ -> error "Unknown tile char"

getTileAt :: Grid -> Coord -> Tile
getTileAt grid (Coord x y) = (grid !! y) !! x

convertStrToGrid :: [String] -> Grid
convertStrToGrid = map (map charToTile)

coordDeltasX = [1, 1, 0, -1, -1, -1, 0, 1]
coordDeltasY = [0, -1, -1, -1, 0, 1, 1, 1]

neighbourCoords :: Coord -> [Coord]
neighbourCoords (Coord x y) = [Coord (x + dx) (y + dy) | (dx, dy) <- zip coordDeltasX coordDeltasY]

isCoordValid :: Grid -> Coord -> Bool
isCoordValid grid (Coord x y) = not $ x < 0 || y < 0 || x >= gridWidth grid || y >= gridHeight grid

neighbours :: Grid -> Coord -> [Tile]
neighbours grid coord = map (getNeighbour grid) (neighbourCoords coord)
 where
  getNeighbour grid coord = if isCoordValid grid coord then getTileAt grid coord else Empty

main = do
  content <- getContents
  let grid = convertStrToGrid $ lines content
      allCoords = [Coord x y | x <- [0 .. gridWidth grid - 1], y <- [0 .. gridHeight grid - 1]]
      paperCoords = filter (\coord -> getTileAt grid coord == Paper) allCoords
      part1 = length . filter (< 4) $ map (length . filter (== Paper) . neighbours grid) paperCoords
  print part1
