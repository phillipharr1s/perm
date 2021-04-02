module Main where 

import Prelude hiding ((*))
import Data.List

main = print 0

p * q = [ p !! (i-1)  | i <- q ] 

a, b :: [Int]
a = [1,9,8,7,2,3,6,5,4]
b = [4,7,5,6,8,1,2,3,9]

len p = length $ filter (uncurry (==)) $ zip p [1..]

powers p = take (length p) [1..] : map (p*) (powers p)

tr n x y | y < x = tr n y x
tr n x y = [1..(x-1)] ++ [y] ++ [(x+1)..(y-1)] ++ [x] ++ [(y+1)..n]
