import System.Environment

inc :: Maybe(Int) -> Int -> [Int] -> Int
inc Nothing s (x:xs) = inc (Just x) s xs
inc _ s []           = s
inc (Just prev) s (x:xs) 
  | x > prev  = inc (Just x) (s+1) xs
  | otherwise = inc (Just x) s xs

sInc :: Maybe(Int) -> Int -> [Int] -> Int
sInc Nothing s (x:y:z:xs) = sInc (Just (x+y+z)) s (y:z:xs)
sInc (Just prev) s (x:y:z:xs)
  | sm > prev = sInc (Just sm) (s+1) (y:z:xs)
  | otherwise = sInc (Just sm) s (y:z:xs)
    where
      sm = x + y + z
sInc _ s (x:y:xs) = s


main = do
    args <- getArgs
    content <- readFile $ head args
    let inp = map read $ lines content
    let output = inc Nothing 0 inp
    let output2 = sInc Nothing 0 inp

    putStrLn $ show output
    putStrLn $ show output2
