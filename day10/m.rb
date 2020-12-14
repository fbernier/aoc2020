def print_result(value)
  puts "the value is: #{value}"
end

def process(test: false)
  input = test ? test_string : File.read('input')
  adapters = input.split("\n").map(&:to_i).sort
  device_j = adapters.max + 3
  adapters << device_j
  
  differences = {
    1 => 0,
    2 => 0,
    3 => 0
  }

  prev = 0
  adapters.each do |a|
    diff = a - prev
    differences[diff] = differences[diff] + 1
    prev = a
  end
  
  puts "differences are #{differences}"
  print_result(differences[1] * differences[3])
end

def test_string
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

process
