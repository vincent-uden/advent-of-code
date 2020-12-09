#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

typedef enum {
    ACC,
    JMP,
    NOP,
    NUL,
} Op;

typedef struct {
    Op op;
    int arg;
} Instruction;

Instruction* read_program(char* file_name) {
    FILE* fp;
    char* line = NULL;
    size_t len = 0;
    ssize_t read;

    Instruction* program = malloc(sizeof(Instruction) * 1000);

    for ( int i = 0; i < 1000; i++ ) {
        program[i].op = NUL;
        program[i].arg = 0;
    }

    fp = fopen("./input.txt", "r");
    if ( fp == NULL ) {
        exit(1);
    }

    int i = 0;
    while ( (read = getline(&line, &len, fp)) != -1 ) {
        if ( !strncmp(line, "acc", 3) ) {
            program[i].op = ACC;
        } else if ( !strncmp(line, "jmp", 3) ) {
            program[i].op = JMP;
        } else if ( !strncmp(line, "nop", 3) ) {
            program[i].op = NOP;
        }
        program[i].arg = strtol(line + 4, NULL, 10);

        i++;
    }

    fclose(fp);
    if ( line ) {
        free(line);
    }

    return program;
}

int run_program(Instruction* program) {
    int visited[1000];
    int acc = 0;
    int ptr = 0;
    int end = 0;

    for ( int i = 0; i < 1000; i++ ) {
        visited[i] = 0;
        if ( program[i].op == NUL && end == 0 ) {
            end = i;
        }
    }

    while ( !visited[ptr] && ptr != end ) {
        // printf("Op: %i   Arg: %i   i: %i   acc: %i\n", program[ptr].op, program[ptr].arg, ptr, acc);
        visited[ptr] = 1;
        switch ( program[ptr].op ) {
            case JMP:
                ptr += program[ptr].arg;
                break;
            case ACC:
                acc += program[ptr].arg;
            case NOP:
                ptr++;
                break;
            default:
                break;
        }

    }

    return acc;
}

int terminates(Instruction* program) {
    int visited[1000];
    int acc = 0;
    int ptr = 0;
    int end = 0;

    for ( int i = 0; i < 1000; i++ ) {
        visited[i] = 0;
        if ( program[i].op == NUL && end == 0 ) {
            end = i;
        }
    }

    while ( !visited[ptr] ) {
        visited[ptr] = 1;
        if ( ptr == end ) {
            return 1;
        }
        switch ( program[ptr].op ) {
            case JMP:
                ptr += program[ptr].arg;
                break;
            case ACC:
                acc += program[ptr].arg;
            case NOP:
                ptr++;
                break;
            default:
                break;
        }

    }

    return 0;
}

void fix_broken_op(Instruction* program) {
    for ( int i = 0; i < 1000; i++ ) {
        if ( program[i].op == JMP ) {
            program[i].op = NOP;
            if ( terminates(program) ) {
                printf("JMP -> NOP at %i\n", i);
                i = 1001;
            } else {
                program[i].op = JMP;
            }
        } else if ( program[i].op == NOP ) {
            program[i].op = JMP;
            if ( terminates(program) ) {
                printf("NOP -> JMP at %i\n", i);
                i = 1001;
            } else {
                program[i].op = NOP;
            }
        }
    }
}

int main() {
    Instruction* program = read_program("./input.txt");
    int acc = run_program(program);

    fix_broken_op(program);

    printf("Before fixing: %i\n", acc);
    printf("After fixing: %i\n", run_program(program));

    return 0;
}
