import Data.List

data Point a = Point a a a deriving (Eq, Show)

type FPoint = Point Float
type Id = Int
type JunctionBox = (Id, FPoint)

replace :: (Eq a) => a -> a -> [a] -> [a]
replace old new = map (\x -> if x == old then new else x)

readPoint :: String -> FPoint
readPoint s = Point x y z
  where
   [x, y, z] = map read . words . replace ',' ' ' $ s

distanceTo :: FPoint -> FPoint -> Float
distanceTo (Point x1 y1 z1) (Point x2 y2 z2) = sqrt $ dx^2 + dy^2 + dz^2
  where
    dx = x2 - x1
    dy = y2 - y1
    dz = z2 - z1

closestJunctionBox :: JunctionBox -> [JunctionBox] -> JunctionBox
closestJunctionBox (_, p) = head . sortBy (\(_, p1) (_, p2) -> (p `distanceTo` p1) `compare` (p `distanceTo` p2))

minJBoxFirst :: (JunctionBox, JunctionBox) -> (JunctionBox, JunctionBox)
minJBoxFirst (jb1@(pId, p), jb2@(qId, q)) = if pId < qId then (jb1, jb2) else (jb2, jb1)

junctionBoxPairs :: [JunctionBox] -> [(JunctionBox, JunctionBox)]
junctionBoxPairs [] = []
junctionBoxPairs (x:xs) = (zip (repeat x) xs) ++ junctionBoxPairs xs

connect :: [[Id]] -> Id -> Id -> [[Id]]
connect circuits p q = if containsP == containsQ then circuits else concat (containsP ++ containsQ) : noPnoQ
  where
    (containsP, noP) = partition (elem p) circuits
    (containsQ, noPnoQ) = partition (elem q) noP

main = do
  contents <- getContents
  let 
    junctionBoxes :: [JunctionBox] = zip [0..] $ map readPoint $ lines contents
    closestPairs = sortBy (\((_, p1), (_, q1)) ((_, p2), (_, q2)) -> (p1 `distanceTo` q1) `compare` (p2 `distanceTo` q2)) $ junctionBoxPairs junctionBoxes
    initCircuits = [[pId] | pId <- [0..length junctionBoxes - 1]]
    circuits1 = foldl' (\x (p, q) -> connect x p q) initCircuits $ map (\((pId, _), (qId, _)) -> (pId, qId)) $ take 1000 closestPairs 
    part1 = product . take 3 . reverse . sort . map length $ circuits1
  print part1
