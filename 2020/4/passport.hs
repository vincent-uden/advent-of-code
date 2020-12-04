
import Data.List.Split ( splitOn )

expectedFields = [ "byr"
                 , "iyr"
                 , "eyr"
                 , "hgt"
                 , "hcl"
                 , "ecl"
                 , "pid"
                 ]

splitPassports :: String -> [String]
splitPassports = splitOn "\n\n"

formatPassport :: String -> String
formatPassport = map (\c -> if c == '\n' then ' ' else c)

extractFields :: String -> [String]
extractFields pport = map ((!! 0) . (splitOn ":")) $ splitOn " " pport

to2Tuple :: [String] -> (String,String)
to2Tuple (x:y:xs) = (x,y)
to2Tuple (x:xs) = ("","")

extractData :: String -> [(String,String)]
extractData pport = map (to2Tuple . splitOn ":") $ splitOn " " pport

validatePair :: (String,String) -> Bool
validatePair ("byr",x) = (read x :: Integer) < 2003 && (read x :: Integer) > 1919
validatePair ("iyr",x) = (read x :: Integer) < 2021 && (read x :: Integer) > 2009
validatePair ("eyr",x) = (read x :: Integer) < 2031 && (read x :: Integer) > 2019
validatePair ("hgt",x) = (unit == "in" && size >= 59 && size <= 76) || (unit == "cm" && size >= 150 && size <= 193)
    where
        unit = reverse $ take 2 $ reverse x
        size = read $ reverse $ drop 2 $ reverse x
validatePair ("hcl",(x:xs)) = x == '#' && and (map (`elem` "0123456789abcdef") xs)
validatePair ("ecl",x) = x `elem` ["amb","blu","brn","gry","grn","hzl","oth"]
validatePair ("pid",x) = and (map (`elem` "0123456789") x) && length x == 9
validatePair _ = True

validateFields :: [String] -> Bool
validateFields xs = and $ map (\x -> x `elem` xs) expectedFields

main = do
    content <- readFile "./input.txt"

    let validAmount = length $ filter id $ map validateFields $ map extractFields $ map formatPassport $ splitPassports $ init content

    putStrLn $ show validAmount

    let passports = map formatPassport $ splitPassports $ init content
    let valid_fields = filter (validateFields . extractFields) passports
    let d = length $ filter id $ map (and . map validatePair) $ map extractData valid_fields

    putStrLn $ show d
