require 'set'

input = File.read("input.txt")
    .gsub(/ bags?\.?|(no other)|(,)/,"")
    .split("\n")
    .map { |line| line.split(" contain ")}

a = {}
b = {}

for line in input
    if !line[1]
        next
    end

    outer = line[0]

    if !a.key?(outer)
        a[outer] = []
    end
    if !b.key?(outer)
        b[outer] = []
    end

    counts = []
    for c in line[1].split(" ")
        d = c.to_i
        if d != 0
            counts << d
        end
    end
    line[1].split(/\s?\d\s/)[1..].each_with_index do |inner, i|
        if !a.key?(inner)
            a[inner] = []
        end
        if !b.key?(inner)
            b[inner] = []
        end
        b[outer] << [inner, counts[i]]
        a[inner] << outer
    end
end

def part_one(map, bags)
    all = Set[]
    for bag in bags
        all.add(bag)
        all.merge(part_one(map, map[bag]))
    end
    return all
end

def part_two(map, bags)
    total = 1

    for bag, count in bags
        total += count * part_two(map, map[bag])
    end

    return total
end


puts part_one(a, a["shiny gold"]).length()
puts part_two(b, b["shiny gold"]) - 1
