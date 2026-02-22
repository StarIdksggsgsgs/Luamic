local function sumNumbers(numbers: {number}): number
    local total: number = 0
    for i = 1, #numbers do
        total += numbers[i]
    end
    return total
end

local myTable: {number} = table.create(5, 0)
local source = {1, 2, 3, 4, 5}
table.move(source, 1, #source, 1, myTable)
print("Sum of table elements:", sumNumbers(myTable))
