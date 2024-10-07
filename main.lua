function test(message)
    for i = 0, 1000000, 1 do
        print("test " .. message .. " " .. i)
    end
end

test("Yeetus")