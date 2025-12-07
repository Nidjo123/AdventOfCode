import qualified Data.Set as Set

data Coord = Coord Int Int deriving (Show, Eq, Ord)
type CoordSet = Set.Set Coord
data Grid = Grid Int Int CoordSet deriving (Show)

gridHeight :: Grid -> Int
gridHeight (Grid _ h _) = h

gridWidth :: Grid -> Int
gridWidth (Grid w _ _) = w

convertStrToGrid :: [String] -> Grid
convertStrToGrid rows = Grid (length rows) (length $ head rows) paperCoords
  where
    gridHeight = length rows
    gridWidth = length $ head rows
    paperCoords = Set.fromList [Coord x y | x <- [0..gridWidth - 1], y <- [0..gridHeight - 1], rows !! y !! x == '@']

coordDeltasX = [1, 1, 0, -1, -1, -1, 0, 1]
coordDeltasY = [0, -1, -1, -1, 0, 1, 1, 1]

neighbourCoords :: Coord -> CoordSet
neighbourCoords (Coord x y) = Set.fromList [Coord (x + dx) (y + dy) | (dx, dy) <- zip coordDeltasX coordDeltasY]

isCoordValid :: Grid -> Coord -> Bool
isCoordValid grid (Coord x y) = not $ x < 0 || y < 0 || x >= gridWidth grid || y >= gridHeight grid

isCoordAccessible :: Grid -> CoordSet -> Coord -> Bool
isCoordAccessible (Grid w h rolls) removedRolls coord = Set.size neighbourRolls < 4
  where
    neighbourRolls = (Set.intersection (neighbourCoords coord) rolls) Set.\\ removedRolls

accessibleRolls :: Grid -> CoordSet -> CoordSet
accessibleRolls grid@(Grid _ _ rolls) removedRolls = Set.filter (isCoordAccessible grid removedRolls) (rolls Set.\\ removedRolls)

cleanupRollsStep :: Grid -> CoordSet -> CoordSet
cleanupRollsStep grid removedRolls = Set.union removedRolls (accessibleRolls grid removedRolls)

cleanupRolls :: Grid -> CoordSet -> CoordSet
cleanupRolls grid removedRolls = if removedRolls == nextRemovedRolls then removedRolls else Set.union removedRolls (cleanupRolls grid nextRemovedRolls)
 where
  nextRemovedRolls = cleanupRollsStep grid removedRolls

main = do
  content <- getContents
  let grid = convertStrToGrid $ lines content
  print . Set.size $ accessibleRolls grid Set.empty
  let removedRolls = cleanupRolls grid Set.empty
  print $ Set.size removedRolls
