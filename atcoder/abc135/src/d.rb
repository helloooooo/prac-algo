s = gets.strip.split('').map{|c| c == '?' ? '?' : c.to_i}
MOD = 10 ** 9 + 7
N = 13
n = s.length
 
dp = Array.new(13,0)
dp[0] = 1
s.each do |c|
    next_arr = Array.new(13,0)
    N.times do |j|
        if c == '?'
            (0..9).each do |c|
                next_r = (j * 10 + c) % 13
                next_arr[next_r] += dp[j]
                next_arr[next_r] %= MOD
            end
        else
            next_r = (j * 10 + c) % 13
            next_arr[next_r] += dp[j]
            next_arr[next_r] %= MOD
        end
    end
    dp = next_arr
end
p dp[5]
