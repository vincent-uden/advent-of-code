import System.Environment
import Data.Char


addBin :: [Int] -> [Int] -> [Int]
addBin (x:xs) (y:ys) = (x+y):(addBin xs ys)
addBin [] [] = []

boolToInt :: Bool -> Int
boolToInt True  = 1
boolToInt False = 0

mCommon :: [Int] -> Int -> [Int]
mCommon xs s = map (boolToInt . (>= s) . (*2)) xs

lCommon :: [Int] -> Int -> [Int]
lCommon xs s = map (boolToInt . (< s) . (*2)) xs

toDec :: [Int] -> Int
toDec [] = 0
toDec xss@(x:xs) = 2^(length xss - 1) * x + toDec xs

bitFilter :: [[Int]] -> [Int] -> Int -> [[Int]]
bitFilter reports bitmap i = filter (\x -> (x !! i) == (bitmap !! i)) reports

filterArray :: [[Int]] -> ([Int] -> Int -> [Int]) -> Int -> [[Int]]
filterArray reports bitmapF i
    | length reports <= 1 = reports
    | i >= length bitmap  = reports
    | otherwise = filterArray (bitFilter reports bitmap i) bitmapF (i+1)
    where
        bitmap = bitmapF (sumBins reports) (length reports)

sumBins :: [[Int]] -> [Int]
sumBins bins = foldl addBin (take (length (head bins)) (repeat 0)) bins

main = do
    args <- getArgs
    content <- readFile $ head args
    let bins = map (map digitToInt) (lines content)
    let sums = sumBins bins

    let gamma = toDec $ mCommon sums (length bins)
    let epsilon = toDec $ lCommon sums (length bins)

    putStrLn $ show gamma
    putStrLn $ show epsilon
    putStrLn $ show (gamma*epsilon)

    putStrLn "---"

    let oxy = toDec $ head $ filterArray bins mCommon 0
    let co2 = toDec $ head $ filterArray bins lCommon 0

    putStrLn $ show $ oxy
    putStrLn $ show $ co2
    putStrLn $ show $ oxy * co2
