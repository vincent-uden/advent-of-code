import System.Environment

data Op = Forward Int
        | Down Int
        | Up Int
    deriving (Show, Eq)

strToOp :: String -> Op
strToOp s
    | op == "forward" = Forward mag
    | op == "down"    = Down mag
    | op == "up"      = Up mag
    where
        parts = words s
        op = head parts
        mag = read $ last parts

execOp :: (Int, Int) -> Op -> (Int, Int)
execOp (x, d) (Forward mag) = (x + mag, d)
execOp (x, d) (Down mag)    = (x, d + mag)
execOp (x, d) (Up mag)      = (x, d - mag)

runSub :: [Op] -> Int
runSub ops = x * d
    where
        (x,d) = foldl execOp (0,0) ops
        
execOp' :: (Int, Int, Int) -> Op -> (Int, Int, Int)
execOp' (x, d, aim) (Forward mag) = (x + mag, d + mag*aim, aim)
execOp' (x, d, aim) (Down mag)    = (x, d, aim + mag)
execOp' (x, d, aim) (Up mag)      = (x, d, aim - mag)

runSub' :: [Op] -> Int
runSub' ops = x * d
    where
        (x,d,_) = foldl execOp' (0,0,0) ops

main = do
    args <- getArgs
    content <- readFile $ head args
    let ops = map strToOp $ lines $ content
    putStrLn $ show $ runSub ops
    putStrLn $ show $ runSub' ops

