fid = fopen("input.txt", "r");
[x, count] = fscanf(fid, "%f", [1, Inf])
fclose(fid);

x = reshape(x, count, 1)
x(count+1) = 0
x = sort(x)

diff1 = 0;
diff2 = 0;
diff3 = 1;

maxStreak = 0;
streak = 0;
streaks = [];
lastDiff = 1;

for i=2:size(x,1)
    d = x(i) - x(i-1)
    if d == 1
        diff1 = diff1 + 1;
    end
    if d == 2
        diff2 = diff2 + 1;
    end
    if d == 3
        diff3 = diff3 + 1;
    end

    if d == lastDiff
        streak = streak + 1;
        if streak > maxStreak
            maxStreak = streak;
        end
    else
        if streak > 1
            if streak == 4
                streaks(size(streaks,1)+1,1) = 7;
            elseif streak == 3
                streaks(size(streaks,1)+1,1) = 4;
            else
                streaks(size(streaks,1)+1,1) = 2;
            end
        end
        streak = 0;
    end
end
if streak > 1
    if streak == 4
        streaks(size(streaks,1)+1,1) = 7;
    elseif streak == 3
        streaks(size(streaks,1)+1,1) = 4;
    else
        streaks(size(streaks,1)+1,1) = 2;
    end
end

diff1
diff2
diff3

diff1 * diff3
maxStreak
streaks
prod(streaks)
