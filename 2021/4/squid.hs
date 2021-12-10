import System.Environment
import Data.List

splitWhen :: (a -> Bool) -> [a] -> [[a]]
splitWhen p s = case dropWhile p s of
                  [] -> []
                  s' -> w : splitWhen p s''
                    where (w, s'') = break p s'

parseBoard :: String -> [Int]
parseBoard xs = map read $ splitWhen (`elem` ['\n',' ']) xs

readInpFile :: String -> ([Int], [[Int]])
readInpFile xs = (map read (splitWhen (==',') (head l)), map parseBoard boards)
    where
        l = lines xs
        boards = map (intercalate "\n") $ splitWhen (=="") $ tail $ tail l

getRow :: [a] -> Int -> [a]
getRow xs x = take 5 $ drop (5*x) xs

getCol :: [a] -> Int -> [a]
getCol xs x = map ((xs !!) . (+x)) [0,5..20]

hasBingo :: [Bool] -> Bool
hasBingo xs = horz || vert
    where
        horz = or $ map (and . (getRow xs)) [0..4]
        vert = or $ map (and . (getCol xs)) [0..4]

markNumber :: [Int] -> Int -> [Bool]
markNumber xs x = map (==x) xs

pairOr :: [Bool] -> [Bool] -> [Bool]
pairOr xs ys = map (\(x, y) -> x || y) (zip xs ys)

findBingo :: [[Int]] -> [[Bool]] -> [Int] -> ([[Bool]], [Bool], Int)
findBingo xs bs ns
    | bingo = (marked, map hasBingo marked, n)
    | otherwise = findBingo xs marked (tail ns)
    where
        n = head ns
        nMarked = map (`markNumber` n) xs
        marked = map (\(x, y) -> pairOr x y) (zip bs nMarked)
        bingo = or $ map hasBingo marked

getResults :: ([[Bool]], [Bool], Int) -> [[Int]] -> Int
getResults (bss, bs, n) xs = (foldl (+) 0 unmarked) * n
    where
        unmarked = map fst $ filter (not . snd) (zip bingoBoard marked)
        (bingoBoard, _) = head $ filter snd (zip xs bs)
        (marked, _)     = head $ filter snd (zip bss bs)

findOneBingo :: [Int] -> [Bool] -> [Int] -> Int -> ([Bool], Int, Int, [Int])
findOneBingo xs bs ns i
    | bingo = (marked, i+1, n, xs)
    | otherwise = findOneBingo xs marked (tail ns) (i+1)
    where
        n = head ns
        nMarked = markNumber xs n
        marked = pairOr nMarked bs
        bingo = hasBingo marked

findAllBingos :: [[Int]] -> [Int] -> [([Bool], Int, Int, [Int])]
findAllBingos xs ns = map (\x -> findOneBingo x (markNumber x (-1)) ns 0) xs

maxBingo :: [([Bool], Int, Int, [Int])] -> ([Bool], Int, Int, [Int])
maxBingo xs = foldr1 (\x@(marked, i, n, xs1) y@(m2, i2, n2, xs2) ->  if i > i2 then x else y) xs

getResult :: ([Bool], Int, Int, [Int]) -> Int
getResult (m, i, n, xs) = (foldl (+) 0 unmarked) * n
    where unmarked = map fst $ filter (not . snd) (zip xs m)
 
main = do
    args <- getArgs
    content <- readFile $ head args

    let (header, boards) = readInpFile content
    --putStrLn $ show boards

    --putStrLn $ show $ markNumber (head boards) 1

    --putStrLn $ show $ findBingo boards (map (`markNumber` (-1)) boards) header
    putStrLn $ show $ getResults (findBingo boards (map (`markNumber` (-1)) boards) header) boards
    putStrLn $ show $ findAllBingos boards header
    putStrLn $ show $ getResult $ maxBingo $ findAllBingos boards header
