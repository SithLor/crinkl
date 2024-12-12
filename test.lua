--[[ Generated with https://github.com/TypeScriptToLua/TypeScriptToLua ]]
t = {
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10
}
do
    i = 1
    while i < #t do
        t[i + 1] = t[i + 1] * t[i]
        i = i + 1
    end
end
