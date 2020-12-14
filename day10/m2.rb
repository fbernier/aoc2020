def print_result(value)
  puts "the value is: #{value}"
end

def process(test: false)
  input = test ? test_string : File.read('input')
  adapters = input.split("\n").map(&:to_i).sort
  device_j = adapters.max + 3
  adapters = [0] + adapters + [device_j]
  puts "adapters are #{adapters.length}"

  permutations = [1] + Array.new(adapters.length - 1, 0)

  adapters.each_with_index do |a, i|
    (1..3).to_a.each do |num|
      if ((a - adapters[i - num]) <= 3)
        p i, num
        permutations[i] += permutations[i - num]
      end
    end
  end

  puts "final permutations are #{permutations}"
  puts "len #{permutations.length}"
  print_result(permutations.last)
end

def test_string2
  "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
end

def test_string
  "16
10
15
5
1
11
7
19
6
12
4"
end

process
