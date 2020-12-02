# Task 1
p File.read("./input.txt").lines.map! { |l| { min: l.split(/[ -]/)[0], max: l.split(/[ -]/)[1], letter: l.split(/[ -]/)[2][0..0], password: l.split(/[ -]/)[3].chomp } }.collect { |r| r[:password].count(r[:letter]).between?(r[:min].to_i,r[:max].to_i) }.filter { |p| p }.count

# Task 2
p File.read("./input.txt").lines.map! { |l| { min: l.split(/[ -]/)[0], max: l.split(/[ -]/)[1], letter: l.split(/[ -]/)[2][0..0], password: l.split(/[ -]/)[3].chomp } }.collect { |r| (r[:password][r[:min].to_i - 1] == r[:letter]) ^ (r[:password][r[:max].to_i - 1] == r[:letter]) }.filter { |p| p }.length
