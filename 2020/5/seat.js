fs = require("fs");

function parseLine(line) {
    let row = 0;
    let col = 0;
    
    let p = 64;

    for ( let i = 0; i < 7; i++ ) {
        if ( line[i] == "B" ) {
            row += p;
        }
        p /= 2;
    }

    p = 4;

    for ( let i = 7; i < 10; i++ ) {
        if ( line[i] == "R" ) {
            col += p;
        }
        p /= 2;
    }

    return { row: row, col: col, id: row * 8 + col };
}

content = fs.readFile("./input.txt", "utf-8", function (err, data) {
    if ( err ) {
        console.log(err);
    }

    lines = data.split("\n");

    let max = 0;
    let ids = [];

    for ( let i = 0; i < lines.length - 1; i++ ) {
        console.log(lines[i]);
        let line = parseLine(lines[i]);
        console.log(line);
        ids.push(line.id);
        if ( line.id > max ) {
            max = line.id;
        }
    }

    ids = ids.sort();
    let prev = ids[0];
    for ( let i = 1; i < ids.length; i++ ) {
        if ( prev + 1 != ids[i] ) {
            console.log(prev, ids[i]);
            break;
        }
        prev = ids[i];
    }
    console.log(ids);
    console.log(max);
});

